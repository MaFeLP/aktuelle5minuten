use crate::models::ArticleStatus;
use crate::util::count_articles;
use crate::util::tinder::{cache_next_article, get_categories, get_first_article};
use crate::{DbConn, ServerError};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::form::Form;
use rocket::http::Status;
use rocket_dyn_templates::{Template, context};
use time::macros::format_description;

#[derive(FromForm)]
pub struct PromoteForm {
    pub key: String,
    #[field(validate = len(1..64))]
    pub category: String,
    pub date: Option<String>,
    pub current_articles: Option<u32>,
    pub max_articles: Option<i64>,
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

    next_tinder_card(
        conn,
        date,
        form.current_articles.unwrap_or(0),
        form.max_articles,
    )
    .await
}

async fn next_tinder_card(
    conn: DbConn,
    date: Option<String>,
    current_articles: u32,
    max_articles: Option<i64>,
) -> Result<Template, Status> {
    let date = date.filter(|date| !date.is_empty());

    let article = match get_first_article(&conn, date.clone()).await? {
        Some(article) => article,
        None => {
            return Ok(Template::render(
                "components/tinder_partial",
                context! {
                    has_articles: false,
                    date: date.unwrap_or_default(),
                },
            ));
        }
    };
    let number_of_articles = match max_articles {
        Some(max) => max,
        None => count_articles(&conn, date.clone()).await?,
    };
    let categories = get_categories(&conn, false).await?;
    cache_next_article(conn, date.clone());

    // Round to a percentage and give back as an i32
    Ok(Template::render(
        "components/tinder_partial",
        context! {
            categories: categories,
            date: date.unwrap_or_default(),
            release_date_time: &article.date.format(format_description!("[day].[month].[year] um [hour]:[minute] Uhr")).map_err(|err| {
                error!("Error formatting date: {}", err);
                Status::InternalServerError
            })?.to_string(),

            progress_current: current_articles + 1,
            progress_max: number_of_articles,
            progress_percentage: ((current_articles + 1) as f64 / number_of_articles as f64 * 100.0).round() as i32,

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

#[delete("/tinder?<key>&<date>&<current_articles>&<max_articles>")]
pub async fn demote_article(
    key: String,
    conn: DbConn,
    date: Option<String>,
    current_articles: Option<u32>,
    max_articles: Option<i64>,
) -> Result<Template, Status> {
    use crate::schema::articles::dsl;

    conn.run(move |c| {
        diesel::update(dsl::articles.find(key))
            .set(dsl::status.eq(i32::from(ArticleStatus::Demoted)))
            .execute(c)
            .map_err(ServerError::DatabaseError)
    })
    .await?;

    next_tinder_card(conn, date, current_articles.unwrap_or(0), max_articles).await
}
