use crate::DbConn;
use crate::models::ArticleStatus;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::form::{Form, FromForm};
use rocket::response::Redirect;
use rocket::{http::Status, serde::json::Json};
use serde::Serialize;
use time::macros::format_description;

#[derive(FromForm)]
pub struct BulletsForm<'a> {
    #[field(validate = len(1..63))]
    pub category: &'a str,
    #[field(validate = len(1..))]
    pub bullets: &'a str,
    pub progress_current: Option<usize>,
    pub progress_max: Option<usize>,
}

pub(crate) fn typst_escape(text: &str) -> String {
    text.replace("\\", "\\\\")
        .replace("#", "\\#")
        .replace("*", "\\*")
        .replace("_", "\\_")
        .replace("~", "\\~")
        .replace("`", "\\`")
        .replace("<", "\\<")
        .replace(">", "\\>")
        .replace("@", "\\@")
        .replace("$", "\\$")
        .replace("%", "\\%")
        .replace("^", "\\^")
        .replace("-?", "\\-?")
}

#[post("/bullets", data = "<bullets>")]
pub async fn bullets(conn: DbConn, bullets: Form<BulletsForm<'_>>) -> Result<Redirect, Status> {
    use crate::schema::articles::dsl;
    use crate::schema::print_articles::dsl as print_dsl;
    use crate::util;

    let category = typst_escape(&bullets.category);
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
    let categories = conn
        .run(move |c| {
            dsl::articles
                .select(diesel::dsl::count_star())
                .filter(dsl::category.is_not_null())
                .filter(dsl::status.eq(i32::from(ArticleStatus::Accepted)))
                .distinct()
                .first::<i64>(c)
                .map_err(|_| Status::InternalServerError)
        })
        .await?;

    info!("{} categories left to process", categories);

    // There are still categories to process
    if categories != 0 {
        return Ok(Redirect::to("/pdfcreate"));
    }

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
    util::typst::create_typst_pdf(conn, &filename).await?;

    info!("{} articles have been marked as printed", affected);

    let pdf_uri = format!("/pdfs/{}", &filename);

    Ok(Redirect::to(pdf_uri))
}

#[get("/all?<print>")]
pub async fn get_all_categories(
    conn: DbConn,
    print: Option<bool>,
) -> Result<Json<Vec<String>>, Status> {
    Ok(Json(
        crate::util::tinder::get_categories(&conn, print.unwrap_or(false)).await?,
    ))
}

#[derive(Serialize, Debug)]
pub struct CategoryResponse {
    pub category: String,
    pub count: usize,
    pub text: String,
}

#[get("/summary?<category>")]
pub async fn get_category_summary(
    conn: DbConn,
    category: String,
) -> Result<Json<CategoryResponse>, Status> {
    use crate::util::pdfcreation;

    let category_return = category.clone();
    let contents = pdfcreation::get_category_contents(&conn, category).await?;

    Ok(Json(CategoryResponse {
        category: category_return,
        count: contents.len(),
        //text: contents.join("\n"),
        text: String::new(),
    }))
}
