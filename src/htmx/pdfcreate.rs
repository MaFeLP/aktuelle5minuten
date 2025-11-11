use crate::api::category::{BulletsForm, typst_escape};
use crate::models::ArticleStatus;
use crate::util::pdfcreation::get_category_contents;
use crate::util::tinder::get_categories;
use crate::util::{AI_PROMPT, wait_for_false};
use crate::{DbConn, regex};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::Either;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::content::RawHtml;
use rocket_dyn_templates::{Template, context};
use std::sync::atomic::{AtomicBool, Ordering};
use time::macros::format_description;

static COMPILE_RUNNING: AtomicBool = AtomicBool::new(false);

#[post("/bullets/next_category", data = "<bullets>")]
pub async fn next_category(
    conn: DbConn,
    bullets: Form<BulletsForm<'_>>,
) -> Result<Template, Status> {
    use crate::schema::articles::dsl;
    use crate::schema::print_articles::dsl as print_dsl;
    use crate::util;

    let category = typst_escape(bullets.category);
    let category2 = category.clone();
    let category3 = category.clone();
    let text = typst_escape(bullets.bullets);

    conn.run(move |c| {
        diesel::insert_into(print_dsl::print_articles)
            .values((
                print_dsl::category.eq(category),
                print_dsl::bullets.eq(text),
            ))
            .execute(c)
            .map_err(|_| Status::InternalServerError)
    })
    .await?;

    let affected = conn
        .run(move |c| {
            diesel::update(dsl::articles.filter(dsl::category.eq(category2)))
                .set(dsl::status.eq(i32::from(ArticleStatus::BulletsCreated)))
                .execute(c)
                .map_err(|_| Status::InternalServerError)
        })
        .await?;

    info!(
        "{} articles have been marked 'bullets created' for category {}",
        affected, &category3
    );

    // Check how many categories are left to process
    let categories = get_categories(&conn, true).await?;
    info!("{} categories left to process", categories.len());

    let progress_current = bullets.progress_current.unwrap_or(0) + 1;
    let progress_max = bullets.progress_max.unwrap_or(categories.len());
    let progress_percentage = ((progress_current as f64 / progress_max as f64) * 100.0).round();
    // There are still categories to process
    if let Some(category) = categories.first() {
        let articles = get_category_contents(&conn, category.clone()).await?;
        return Ok(Template::render(
            "components/pdfcreate_partial",
            context! {
                progress_current: progress_current,
                progress_max: progress_max,
                progress_percentage: progress_percentage,

                prompt: AI_PROMPT,
                title: category,
                articles: articles,
                chatgpt_enabled: true,
                claude_enabled: true,
            },
        ));
    }

    // Fetch the categories from the database including content and generate a PDF
    let filename = format!(
        "{}.pdf",
        time::OffsetDateTime::now_utc()
            .format(format_description!(
                "[year]-[month]-[day]T[hour]:[minute]:[second]"
            ))
            .map_err(|err| {
                error!("Error formatting date: {}", err);
                Status::InternalServerError
            })?
    );

    let filename2 = filename.clone();
    rocket::tokio::spawn(async move {
        if !wait_for_false(&COMPILE_RUNNING, 25, 500) {
            error!("Could not obtain compile lock for PDF generation!")
        }
        COMPILE_RUNNING.store(true, Ordering::Relaxed);
        match util::typst::create_typst_pdf(conn, &filename).await {
            Ok(_) => {}
            Err(err) => error!("Could not create PDF after 25 failed attempts: {}", err),
        }
        COMPILE_RUNNING.store(false, Ordering::Relaxed);
    });

    let pdf_uri = format!("/files/{}", &filename2);
    Ok(Template::render(
        "wait_for_pdf",
        context! {
            redirect_uri: pdf_uri,
        },
    ))
}

#[get("/compile_running?<redirect_uri>")]
pub async fn compile_running(
    redirect_uri: String,
) -> Result<Either<&'static str, RawHtml<String>>, Status> {
    if COMPILE_RUNNING.load(Ordering::Relaxed) {
        Ok(Either::Left("Dokument wird noch erstellt..."))
    } else {
        let re = regex!(r"^/files/[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}\.pdf$");
        if !re.is_match(&redirect_uri) {
            return Err(Status::BadRequest);
        }
        Ok(Either::Right(RawHtml(format!(
          "<a class=\"btn btn-outline-primary\" id=\"pdf-download-button\" href=\"{redirect_uri}\">
            <i class=\"bi bi-file-earmark-pdf\"></i>
            Das PDF ist fertig! Hier klicken, um es herunterzuladen.
          </a>"
        ))))
    }
}
