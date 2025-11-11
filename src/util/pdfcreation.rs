use crate::{DbConn, ServerError, regex};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PrintArticle {
    pub title: String,
    pub content: String,
}

pub async fn get_category_contents(
    conn: &DbConn,
    category: String,
) -> Result<Vec<PrintArticle>, ServerError> {
    use crate::schema::articles::dsl;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

    #[allow(clippy::unwrap_used)]
    let contents: Vec<PrintArticle> = conn
        .run(move |c| {
            dsl::articles
                .select((dsl::title, dsl::content))
                .filter(dsl::content.is_not_null())
                .filter(dsl::category.eq(category))
                .distinct()
                .load::<(String, Option<String>)>(c)
        })
        .await?
        .iter()
        .map(|(title, content)|
            // We can safely unwrap here, as the filter in the query filters out non-null values.
            (title.clone(), content.as_ref().unwrap().clone())
        )
        // Remove copyright notice (is not needed for the AI)
        .map(|s| (s.0, regex!(r#"Diese Nachricht wurde am \d{2}\.\d{2}\.\d{4} im Programm Deutschlandfunk gesendet\."#).replace_all(&s.1, "").to_string()))
        .map(|s| PrintArticle {
            title: s.0,
            content: s.1,
        })
        .collect();

    Ok(contents)
}
