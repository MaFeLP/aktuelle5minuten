use crate::{dlf, DbConn};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::http::Status;

// A route /api/actions/load that gets the latest dlf::wockenrueckblick and then inserts all the articles into the articles database
#[get("/load")]
pub async fn load_new_articles(conn: DbConn) -> Result<Status, Status> {
    use crate::models::NewArticle;
    use crate::schema::articles::dsl::*;

    let wochenrueckblick_articles = dlf::wochenrueckblick()
        .await
        .map_err(|_| Status::InternalServerError)?;
    for article in wochenrueckblick_articles {
        let article_key = article.key.clone();
        let new_article = NewArticle::from(&article);
        let inserted_article = conn
            .run(move |c| {
                diesel::insert_into(articles)
                    .values(&new_article)
                    .on_conflict(key)
                    .do_nothing()
                    .execute(c)
                    .map_err(|_| Status::InternalServerError)
            })
            .await?;
        if inserted_article != 1 {
            conn.run(move |c| {
                diesel::update(crate::schema::articles::table.filter(key.eq(&article_key)))
                    .set(status.eq(1))
                    .execute(c)
                    .map_err(|_| Status::InternalServerError)
            })
            .await?;
        }
    }
    Ok(Status::Ok)
}
