use crate::models::{Article, ArticleStatus};
use crate::scrapers::dlf;
use crate::DbConn;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};
use time::macros::format_description;

#[get("/tinder")]
pub(crate) async fn tinder(conn: DbConn) -> Result<Template, Status> {
    // TODO: Fix promoting not working
    // TODO: implement demote route for htmx article
    use crate::schema::articles::dsl;

    let mut article = conn
        .run(|c| {
            dsl::articles
                .select(Article::as_select())
                .filter(dsl::status.eq(i32::from(ArticleStatus::Uncategorized)))
                .first::<Article>(c)
                .map_err(|_| Status::NotFound)
        })
        .await?;

    if article.content.is_some() {
        info!("Found article with key '{}' in cache.", article.key);
    } else {
        info!(
            "Article with key '{}' had no content in the database. Updating...",
            article.key
        );

        let parsed = dlf::article(&format!("{}{}", dlf::PREFIX, &article.key))
            .await
            .unwrap();

        article.merge(&parsed);

        // Sometimes the urls (keys) change, so we no longer need the old article.
        // This can happen if the title of the article is changed, for example.
        if parsed.key != article.key {
            let old_key = article.key.clone();
            conn.run(move |c| {
                diesel::delete(dsl::articles.filter(dsl::key.eq(&old_key)))
                    .execute(c)
                    .map_err(|_| Status::InternalServerError)
            })
            .await?;
            article.key = parsed.key.clone();
        }

        let updated_article = article.clone();
        conn.run(move |c| {
            diesel::update(dsl::articles.find(&updated_article.key))
                .set(&updated_article)
                .execute(c)
                .map_err(|_| Status::InternalServerError)
        })
        .await?;
    }

    // Round to a percentage and give back as an i32
    let initial_percentage = 1.0_f64 / 50.0_f64;
    let rounded = (initial_percentage * 100.0_f64).round() as i32;
    Ok(Template::render(
        "tinder",
        context! {
            categories: vec!["Innenpolitik"],
            initial_percentage: rounded,
            max_articles: 50,
            release_date_time: &article.date.format(format_description!("[day].[month].[year] um [hour]:[minute] Uhr")).map_err(|err| {
                error!("Error formatting date: {}", err);
                Status::InternalServerError
            })?.to_string(),

            has_articles: true,

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
