use crate::models::ArticleStatus;
use crate::{dlf, DbConn};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::http::Status;
use std::ops::Sub;

#[get("/clean")]
pub async fn clean_articles(conn: DbConn) -> Result<Status, Status> {
    use crate::schema::articles::dsl;
    use crate::schema::print_articles::dsl as print_dsl;

    // Delete all articles older than one month
    let deleted = conn
        .run(move |c| {
            diesel::delete(dsl::articles.filter(dsl::date.lt(
                diesel::dsl::now.sub(diesel::dsl::sql::<diesel::sql_types::Interval>("'1 month'")),
            )))
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
