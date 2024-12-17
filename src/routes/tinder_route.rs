use crate::util::tinder::{cache_next_article, count_articles, get_categories, get_first_article};
use crate::DbConn;
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};
use time::macros::format_description;

#[get("/tinder?<date>")]
pub(crate) async fn tinder(conn: DbConn, date: Option<String>) -> Result<Template, Status> {
    let article = get_first_article(&conn, date.clone()).await?;
    let number_of_articles = count_articles(&conn, date.clone()).await?;
    let categories = get_categories(&conn, false).await?;
    cache_next_article(conn, date.clone());

    // Round to a percentage and give back as an i32
    let initial_percentage = 1.0_f64 / number_of_articles as f64;
    let rounded = (initial_percentage * 100.0_f64).round() as i32;
    Ok(Template::render(
        "tinder",
        context! {
            categories: categories,
            date: date.unwrap_or_default(),
            initial_percentage: rounded,
            max_articles: number_of_articles,
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
    ))
}
