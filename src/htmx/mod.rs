use crate::{api, DbConn};
use rocket::http::Status;
use rocket::response::Redirect;

pub mod pdfcreate;
pub mod tinder;

#[get("/actions/load/dlf")]
pub async fn load_new_dlf_articles(conn: DbConn) -> Result<Redirect, Status> {
    api::actions::load_new_articles(conn)
        .await
        .map(|_| Redirect::to("/dates"))
}
