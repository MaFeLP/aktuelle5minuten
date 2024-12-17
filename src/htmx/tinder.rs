use crate::models::ArticleStatus;
use crate::DbConn;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::http::Status;
use rocket_dyn_templates::Template;

#[delete("/tinder?<key>")]
pub async fn demote_article(key: String, conn: DbConn) -> Result<Template, Status> {
    info!("Demoting article {key}");

    use crate::schema::articles::dsl;

    conn.run(move |c| {
        diesel::update(dsl::articles.find(key))
            .set(dsl::status.eq(i32::from(ArticleStatus::Demoted)))
            .execute(c)
            .map_err(|_| Status::InternalServerError)
    })
    .await?;

    Err(Status::NotImplemented)
}
