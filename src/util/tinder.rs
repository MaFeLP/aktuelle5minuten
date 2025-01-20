use crate::models::{Article, ArticleStatus, DATE_FORMAT};
use crate::scrapers::dlf;
use crate::{DbConn, ServerError};
use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, Table,
};
use time::Date;

const DEFAULT_CATEGORIES: [&str; 7] = [
    "Aktuelles Ereignis",
    "Außenpolitik",
    "Hamburg",
    "Politik",
    "Sonstiges",
    "USA",
    "Wirtschaft",
];

pub(crate) fn cache_next_article(conn: DbConn, date: Option<String>) {
    rocket::tokio::spawn(async move {
        use crate::schema::articles::dsl;

        let article = match date {
            Some(article_date) => {
                let article_date = Date::parse(&article_date, &DATE_FORMAT).expect("Invalid date");
                conn.run(move |c| {
                    dsl::articles
                        .select(dsl::articles::all_columns())
                        .filter(diesel::dsl::date(dsl::date).eq(article_date))
                        .filter(dsl::status.eq(i32::from(ArticleStatus::Uncategorized)))
                        .filter(dsl::content.is_null())
                        .select(Article::as_select())
                        .first::<Article>(c)
                        .optional()
                })
                .await
                .expect("Database error while caching next article")
            }
            None => conn
                .run(|c| {
                    dsl::articles
                        .select(Article::as_select())
                        .filter(dsl::status.eq(i32::from(ArticleStatus::Uncategorized)))
                        .filter(dsl::content.is_null())
                        .first::<Article>(c)
                        .optional()
                })
                .await
                .expect("Database error while caching next article"),
        };

        match article {
            None => {
                info!("No articles to cache.");
                return;
            }
            Some(article) => {
                let article = download_and_parse_dlf_article(&conn, article)
                    .await
                    .expect("Error while caching (downloading/parsing) next article");
                let article_key = article.key.clone();
                let article_key2 = article.key.clone();
                conn.run(move |c| {
                    diesel::update(dsl::articles.find(&article_key))
                        .set(&article)
                        .execute(c)
                })
                .await
                .expect("Could not cache the next article to the database");
                info!("Precached article with key '{}'", article_key2)
            }
        }
    });
}

pub(crate) async fn get_first_article(
    conn: &DbConn,
    date: Option<String>,
) -> Result<Option<Article>, ServerError> {
    use crate::schema::articles::dsl;

    let article = match date {
        Some(article_date) => {
            let article_date = Date::parse(&article_date, &DATE_FORMAT)?;
            conn.run(move |c| {
                dsl::articles
                    .select(dsl::articles::all_columns())
                    .filter(diesel::dsl::date(dsl::date).eq(article_date))
                    .filter(dsl::status.eq(i32::from(ArticleStatus::Uncategorized)))
                    .select(Article::as_select())
                    .first::<Article>(c)
                    .optional()
            })
            .await?
        }
        None => {
            conn.run(|c| {
                dsl::articles
                    .select(Article::as_select())
                    .filter(dsl::status.eq(i32::from(ArticleStatus::Uncategorized)))
                    .first::<Article>(c)
                    .optional()
            })
            .await?
        }
    };
    let mut article = match article {
        Some(a) => a,
        None => return Ok(None),
    };
    if article.content.is_some() {
        info!("Found article with key '{}' in cache.", article.key);
    } else {
        info!(
            "Article with key '{}' had no content in the database. Updating...",
            article.key
        );
        article = download_and_parse_dlf_article(conn, article).await?;
    }

    Ok(Some(article))
}

async fn download_and_parse_dlf_article(
    conn: &DbConn,
    mut article: Article,
) -> Result<Article, ServerError> {
    use crate::schema::articles::dsl;

    let parsed = dlf::article(&format!("{}{}", dlf::PREFIX, &article.key))
        .await
        .unwrap();

    article.merge(&parsed);

    // Sometimes the urls (keys) change, so we no longer need the old article.
    // This can happen if the title of the article is changed, for example.
    if parsed.key != article.key {
        let old_key = article.key.clone();
        conn.run(move |c| diesel::delete(dsl::articles.filter(dsl::key.eq(&old_key))).execute(c))
            .await?;
        article.key = parsed.key.clone();
    }

    let updated_article = article.clone();
    conn.run(move |c| {
        diesel::update(dsl::articles.find(&updated_article.key))
            .set(&updated_article)
            .execute(c)
    })
    .await?;

    Ok(article)
}

pub(crate) async fn get_categories(conn: &DbConn, print: bool) -> Result<Vec<String>, ServerError> {
    use crate::schema::articles::dsl;

    #[allow(clippy::unwrap_used)]
    let mut categories: Vec<String> = conn
        .run(move |c| {
            dsl::articles
                .select(dsl::category)
                .filter(dsl::category.is_not_null())
                .filter(dsl::status.eq(i32::from(ArticleStatus::Accepted)))
                .distinct()
                .load::<Option<String>>(c)
        })
        .await?
        .iter()
        // We can safely unwrap here, as the filter in the query filters out non-null values.
        .map(|c| c.as_ref().unwrap().clone())
        .collect();

    if !print {
        for category in DEFAULT_CATEGORIES {
            if !categories.contains(&category.to_string()) {
                categories.push(category.to_string());
            }
        }
    }

    categories.sort();

    Ok(categories)
}
