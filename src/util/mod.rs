use crate::{DbConn, ServerError};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

pub(crate) mod pdfcreation;
pub(crate) mod tinder;
pub(crate) mod typst;

pub(crate) const AI_PROMPT: &str = include_str!("../../prompt.txt");

pub(crate) async fn count_articles(
    conn: &DbConn,
    date: Option<String>,
) -> Result<i64, ServerError> {
    use crate::models::DATE_FORMAT;
    use crate::schema::articles::dsl;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
    use time::Date;

    match date {
        Some(article_date) => {
            let article_date = Date::parse(&article_date, &DATE_FORMAT)?;
            let count = conn
                .run(move |c| {
                    dsl::articles
                        .select(diesel::dsl::count_star())
                        .filter(diesel::dsl::date(dsl::date).eq(article_date))
                        .filter(
                            dsl::status.eq(i32::from(crate::models::ArticleStatus::Uncategorized)),
                        )
                        .first::<i64>(c)
                })
                .await?;
            Ok(count)
        }
        None => {
            let count = conn
                .run(|c| {
                    dsl::articles
                        .select(diesel::dsl::count_star())
                        .filter(
                            dsl::status.eq(i32::from(crate::models::ArticleStatus::Uncategorized)),
                        )
                        .first::<i64>(c)
                })
                .await?;
            Ok(count)
        }
    }
}

pub(crate) fn wait_for_false(flag: &AtomicBool, max_retries: u32, delay_ms: u64) -> bool {
    for _ in 0..max_retries {
        if !flag.load(Ordering::Relaxed) {
            return true; // Success - value is false
        }
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    false // Failed to observe false within max retries
}
