use rocket::http::Status;
use rocket_dyn_templates::{Template, context};
use std::path::PathBuf;

#[get("/pdflist?<no_categories>")]
pub(crate) async fn pdflist(no_categories: Option<bool>) -> Result<Template, Status> {
    let pdfs: Vec<String> = std::fs::read_dir(
        PathBuf::from(std::env::var("A5M_DATA_PATH").unwrap_or("/data".to_string())).join("pdfs"),
    )
    .map_err(|err| {
        error!("Error reading the PDF list from disk: {:?}", err);
        Status::InternalServerError
    })?
    .map(|name| {
        let name = name.unwrap();
        name.file_name().into_string().unwrap()
    })
    .collect();

    Ok(Template::render(
        "pdflist",
        context! {
            files: pdfs,
            no_categories: no_categories.unwrap_or(false),
        },
    ))
}
