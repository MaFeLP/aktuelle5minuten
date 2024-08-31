//!
//! Holds structures for the database models.
//!

use crate::dlf::PartialArticle;
use diesel::Insertable;
use time::{macros::format_description, PrimitiveDateTime};

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::articles)]
pub struct NewArticle {
    pub key: String,
    pub title: String,
    pub teaser_headline: String,
    pub teaser_text: String,
    pub date: PrimitiveDateTime,
    pub locale_date: String,
    pub kicker: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
    pub status: i32,
}

impl From<&PartialArticle> for NewArticle {
    fn from(partial: &PartialArticle) -> Self {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]+[offset_hour]:[offset_minute]");
        NewArticle {
            key: partial.key.clone(),
            title: partial.title.clone(),
            teaser_headline: partial.teaser_headline.clone().unwrap_or_default().clone(),
            teaser_text: partial.teaser_text.clone().unwrap_or_default().clone(),
            date: PrimitiveDateTime::parse(&partial.date, &format).unwrap(),
            locale_date: partial.locale_date.clone(),
            kicker: None,
            description: None,
            content: None,
            category: None,
            status: 0,
        }
    }
}
