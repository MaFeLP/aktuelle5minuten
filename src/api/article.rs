use crate::{dlf, DbConn};
use diesel::{ExpressionMethods, QueryDsl, Queryable, RunQueryDsl, Table};
use rocket::{http::Status, serde::json::Json};
use serde::Serialize;
use time::{macros::format_description, Date, PrimitiveDateTime};

#[derive(Serialize, Debug, Queryable)]
pub struct Article {
    key: String,
    title: String,
    teaser_headline: String,
    teaser_text: String,
    date: PrimitiveDateTime,
    locale_date: String,
    kicker: Option<String>,
    description: Option<String>,
    content: Option<String>,
    category: Option<String>,
    /// 0: Uncategorized
    /// 1: Accepted
    /// 2: Demoted, do not print (can be deleted)
    /// 3: Bullet Points have been created and the text is in the print_articles
    status: i32,
}

#[get("/get/first?<date>")]
pub async fn get_first_article(
    conn: DbConn,
    date: Option<String>,
) -> Result<Json<Article>, Status> {
    use crate::schema::articles::dsl;

    let article = match date {
        Some(article_date) => {
            let format = format_description!("[year]-[month]-[day]");
            let article_date = Date::parse(&article_date, &format).unwrap();
            conn.run(move |c| {
                dsl::articles
                    .select(dsl::articles::all_columns())
                    .filter(diesel::dsl::date(dsl::date).eq(article_date))
                    .first::<Article>(c)
                    .map_err(|_| Status::NotFound)
            })
            .await?
        }
        None => {
            conn.run(|c| {
                dsl::articles
                    .select(dsl::articles::all_columns())
                    .first::<Article>(c)
                    .map_err(|_| Status::NotFound)
            })
            .await?
        }
    };
    let parsed_article = dlf::article(&format!("{}{}", dlf::PREFIX, &article.key))
        .await
        .unwrap();
    let article_key = article.key.clone();
    if article.key != parsed_article.key {
        conn.run(move |c|
            // Article has been updated and moved! Work with the newer version and discard the older one!
            diesel::update(dsl::articles.filter(dsl::key.eq(&article_key)))
                .set(dsl::status.eq(2))
                .execute(c)
                .map_err(|_| Status::InternalServerError))
            .await?;
    }

    Ok(Json(article))
}
