use crate::models::{ArticleStatus, DATE_FORMAT};
use crate::scrapers::dlf;
use crate::{DbConn, ServerError};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use rocket::http::Status;
use std::ops::Sub;
use time::Date;

#[get("/clean")]
pub async fn clean_articles(conn: DbConn) -> Result<Status, Status> {
    use crate::schema::articles::dsl;
    use crate::schema::print_articles::dsl as print_dsl;

    let one_month_ago = time::OffsetDateTime::now_utc()
        .sub(time::Duration::days(30))
        .date();

    // Delete all articles older than one month
    let deleted = conn
        .run(move |c| {
            // Perform the delete operation
            diesel::delete(dsl::articles)
                .filter(dsl::date.lt(one_month_ago.to_string()))
                .execute(c)
                .map_err(|_| Status::InternalServerError)
        })
        .await?;

    // Delete all print articles
    let deleted_print = conn
        .run(move |c| {
            diesel::delete(print_dsl::print_articles)
                .execute(c)
                .map_err(|_| Status::InternalServerError)
        })
        .await?;

    warn!(
        "Cleanup completed. Deleted {} articles and {} print_articles",
        deleted, deleted_print
    );

    Ok(Status::Ok)
}

#[get("/load")]
pub async fn load_new_articles(conn: DbConn) -> Result<Status, Status> {
    use crate::models::Article;
    use crate::schema::articles::dsl::*;

    let wochenrueckblick_articles = dlf::wochenrueckblick().await.map_err(|why| {
        error!("Failed to fetch Wochenrückblick articles: {:?}", why);
        Status::InternalServerError
    })?;
    for article in wochenrueckblick_articles {
        let article_key = article.key.clone();
        let new_article = Article::from(&article);
        let inserted_article = conn
            .run(move |c| {
                diesel::insert_into(articles)
                    .values(&new_article)
                    .on_conflict(key)
                    .do_nothing()
                    .execute(c)
                    .map_err(|why| {
                        error!("Failed to insert article {}: {:?}", &new_article.key, why);
                        Status::InternalServerError
                    })
            })
            .await?;
        if inserted_article != 1 {
            conn.run(move |c| {
                diesel::update(crate::schema::articles::table.filter(key.eq(&article_key)))
                    .set(status.eq(i32::from(ArticleStatus::Uncategorized)))
                    .execute(c)
                    .map_err(|err| {
                        error!("Failed to update article {}: {}", &article_key, err);
                        Status::InternalServerError
                    })
            })
            .await?;
        }
    }
    Ok(Status::Created)
}

#[get("/delete_next?<date>")]
pub async fn delete_next(conn: DbConn, date: Option<String>) -> Result<Status, Status> {
    use crate::schema::articles::dsl;

    let delete_key = get_next_article_key(&conn, date).await?;

    if let Some(key) = delete_key {
        conn.run(move |c| diesel::delete(dsl::articles.filter(dsl::key.eq(key))).execute(c))
            .await
            .map_err(|err| {
                error!("Could not find article to delete: {}", err);
                Status::InternalServerError
            })?;
    }

    Ok(Status::Created)
}

#[get("/demote_next?<date>")]
pub async fn demote_next(conn: DbConn, date: Option<String>) -> Result<Status, Status> {
    use crate::schema::articles::dsl;

    let demote_key = get_next_article_key(&conn, date).await?;

    if let Some(key) = demote_key {
        conn.run(move |c| {
            diesel::update(dsl::articles.find(&key))
                .set(dsl::status.eq(i32::from(ArticleStatus::Demoted)))
                .execute(c)
                .map_err(|_| Status::InternalServerError)
        })
        .await?;
    }

    Ok(Status::Created)
}

async fn get_next_article_key(
    conn: &DbConn,
    date: Option<String>,
) -> Result<Option<String>, ServerError> {
    use crate::schema::articles::dsl;

    Ok(match date {
        Some(date) => {
            let article_date = Date::parse(&date, &DATE_FORMAT).unwrap();
            conn.run(move |c| {
                dsl::articles
                    .filter(diesel::dsl::date(dsl::date).eq(article_date))
                    .filter(dsl::status.eq(i32::from(ArticleStatus::Uncategorized)))
                    .filter(dsl::content.is_null())
                    .select(dsl::key)
                    .first::<String>(c)
                    .optional()
            })
            .await?
        }
        None => {
            conn.run(move |c| {
                dsl::articles
                    .filter(dsl::status.eq(i32::from(ArticleStatus::Uncategorized)))
                    .filter(dsl::content.is_null())
                    .select(dsl::key)
                    .first::<String>(c)
                    .optional()
            })
            .await?
        }
    })
}
