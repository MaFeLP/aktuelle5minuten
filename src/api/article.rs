use crate::models::{Article, DATE_FORMAT};
use crate::{dlf, DbConn};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, Table};
use rocket::{http::Status, serde::json::Json};
use time::Date;

#[get("/get/first?<date>")]
pub async fn get_first_article(
    conn: DbConn,
    date: Option<String>,
) -> Result<Json<Article>, Status> {
    use crate::schema::articles::dsl;

    let mut article = match date {
        Some(article_date) => {
            let article_date = Date::parse(&article_date, &DATE_FORMAT).unwrap();
            conn.run(move |c| {
                dsl::articles
                    .select(dsl::articles::all_columns())
                    .filter(diesel::dsl::date(dsl::date).eq(article_date))
                    .select(Article::as_select())
                    .first::<Article>(c)
                    .map_err(|_| Status::NotFound)
            })
            .await?
        }
        None => {
            conn.run(|c| {
                dsl::articles
                    .select(Article::as_select())
                    .first::<Article>(c)
                    .map_err(|_| Status::NotFound)
            })
            .await?
        }
    };
    if article.content.is_some() {
        info!("Found article with key '{}' in cache.", article.key);
    } else {
        info!(
            "Article with key '{}' had no content in the database. Updating...",
            article.key
        );

        let parsed = dlf::article(&format!("{}{}", dlf::PREFIX, &article.key))
            .await
            .unwrap();

        article.merge(&parsed);

        // Sometimes the urls (keys) change, so we no longer need the old article.
        // This can happen if the title of the article is changed, for example.
        if parsed.key != article.key {
            let old_key = article.key.clone();
            conn.run(move |c| {
                diesel::delete(dsl::articles.filter(dsl::key.eq(&old_key)))
                    .execute(c)
                    .map_err(|_| Status::InternalServerError)
            })
            .await?;
            article.key = parsed.key.clone();
        }

        let updated_article = article.clone();
        conn.run(move |c| {
            diesel::update(dsl::articles.find(&updated_article.key))
                .set(&updated_article)
                .execute(c)
                .map_err(|_| Status::InternalServerError)
        })
        .await?;
    }

    Ok(Json(article))
}

#[get("/get?<key>")]
pub async fn get_article_by_key(conn: DbConn, key: String) -> Result<Json<Article>, Status> {
    use crate::schema::articles::dsl;

    let article = conn
        .run(move |c| {
            dsl::articles
                .select(Article::as_select())
                .filter(dsl::key.eq(&key))
                .first::<Article>(c)
                .map_err(|_| Status::NotFound)
        })
        .await?;

    Ok(Json(article))
}

#[get("/dates")]
pub async fn get_all_article_dates(conn: DbConn) -> Result<Json<Vec<String>>, Status> {
    use crate::schema::articles::dsl;

    let dates = conn
        .run(move |c| {
            dsl::articles
                .select(diesel::dsl::date(dsl::date))
                .distinct()
                .order(dsl::date.desc())
                .load::<String>(c)
                .map_err(|_| Status::InternalServerError)
        })
        .await?;

    Ok(Json(dates))
}
