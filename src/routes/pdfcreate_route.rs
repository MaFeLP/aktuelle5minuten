use crate::util::pdfcreation::get_category_contents;
use crate::util::tinder::get_categories;
use crate::util::AI_PROMPT;
use crate::DbConn;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::Either;
use rocket_dyn_templates::{context, Template};

#[get("/pdfcreate")]
pub(crate) async fn pdfcreate(conn: DbConn) -> Result<Either<Redirect, Template>, Status> {
    let categories = get_categories(&conn, true).await?;
    match categories.first() {
        Some(category) => {
            let articles = get_category_contents(&conn, category.clone())
                .await
                .map_err(|err| {
                    error!(
                        "Could not get category contents for category '{}': {}",
                        category, err
                    );
                    Status::InternalServerError
                })?;

            dbg!(AI_PROMPT);

            // Round to a percentage and give back as an i32
            let initial_percentage = 1.0_f64 / categories.len() as f64;
            let rounded = (initial_percentage * 100.0_f64).round() as i32;
            Ok(Either::Right(Template::render(
                "pdfcreate",
                context! {
                    progress_current: 1,
                    progress_max: categories.len(),
                    progress_percentage: rounded,

                    // For the first category card
                    prompt: AI_PROMPT,
                    title: category,
                    articles: articles,
                    chatgpt_enabled: true,
                    claude_enabled: true,
                },
            )))
        }
        None => Ok(Either::Left(Redirect::to("/pdflist?no_categories=true"))),
    }
}
