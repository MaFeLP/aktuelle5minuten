use rocket::http::Status;
use rocket_dyn_templates::Template;

#[get("/pdfcreate")]
pub(crate) async fn pdfcreate() -> Result<Template, Status> {
    Err(Status::NotImplemented)
}
