use crate::models::ArticleStatus;
use crate::util::tinder::{cache_next_article, count_articles, get_categories, get_first_article};
use crate::{DbConn, ServerError};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::form::Form;
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};
use time::macros::format_description;

#[derive(FromForm)]
pub struct PromoteForm {
    pub key: String,
    #[field(validate = len(1..64))]
    pub category: String,
    pub date: Option<String>,
}

#[post("/tinder", data = "<form>")]
pub async fn promote_article(conn: DbConn, form: Form<PromoteForm>) -> Result<Template, Status> {
    use crate::schema::articles::dsl;

    let key = form.key.clone();
    let category = form.category.clone();

    conn.run(move |c| {
        diesel::update(dsl::articles.find(&key))
            .set((
                dsl::status.eq(i32::from(ArticleStatus::Accepted)),
                dsl::category.eq(&category),
            ))
            .execute(c)
            .map_err(|err| {
                error!("Could not promote article: {}", err);
                Status::InternalServerError
            })
    })
    .await?;

    let date = match &form.date {
        None => None,
        Some(d) => {
            if d.is_empty() {
                None
            } else {
                Some(d.clone())
            }
        }
    };

    next_tinder_card(conn, date).await?
}

async fn next_tinder_card(
    conn: DbConn,
    date: Option<String>,
) -> Result<Result<Template, Status>, Status> {
    let article = get_first_article(&conn, date.clone()).await?;
    let number_of_articles = count_articles(&conn, date.clone()).await?;
    let categories = get_categories(&conn, false).await?;
    cache_next_article(conn, date.clone());

    // Round to a percentage and give back as an i32
    Ok(Ok(Template::render(
        "tinder_base",
        context! {
            categories: categories,
            date: date.unwrap_or_default(),
            release_date_time: &article.date.format(format_description!("[day].[month].[year] um [hour]:[minute] Uhr")).map_err(|err| {
                error!("Error formatting date: {}", err);
                Status::InternalServerError
            })?.to_string(),

            has_articles: number_of_articles > 0,

            // Render the article page
            figure_src: &article.figure_src,
            figure_alt: &article.figure_alt,
            figure_srcset: &article.figure_srcset,
            figure_title: &article.figure_title,
            article_kicker: &article.kicker,
            article_title: &article.title,
            article_description: &article.description,
            article_content_html: &article.content_html,
            article_key: &article.key,
        },
    )))
}

#[delete("/tinder?<key>&<date>")]
pub async fn demote_article(
    key: String,
    conn: DbConn,
    date: Option<String>,
) -> Result<Template, Status> {
    use crate::schema::articles::dsl;

    conn.run(move |c| {
        diesel::update(dsl::articles.find(key))
            .set(dsl::status.eq(i32::from(ArticleStatus::Demoted)))
            .execute(c)
            .map_err(|err| ServerError::DatabaseError(err))
    })
    .await?;

    next_tinder_card(conn, date).await?
}
