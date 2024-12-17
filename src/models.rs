//!
//! Holds structures for the database models.
//!

use crate::scrapers::dlf;
use crate::scrapers::dlf::PartialArticle;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::Serialize;
use time::format_description::BorrowedFormatItem;
use time::{macros::format_description, PrimitiveDateTime};

pub const DATE_FORMAT: &[BorrowedFormatItem] = format_description!("[year]-[month]-[day]");
//pub const DATETIME_FORMAT: &[BorrowedFormatItem] = format_description!("[year]-[month]-[day] [year]-[month]-[day] [hour]:[minute]:[second].[subsecond]");

pub const DATETIME_FORMAT: &[BorrowedFormatItem] = format_description!(
    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]+[offset_hour]:[offset_minute]"
);

#[derive(Insertable, AsChangeset, Selectable, Queryable, Serialize, Debug, Clone)]
#[diesel(table_name = crate::schema::articles)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub key: String,
    pub title: String,
    pub teaser_headline: String,
    pub teaser_text: String,
    pub date: PrimitiveDateTime,
    pub locale_date: String,
    pub kicker: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub content_html: Option<String>,

    pub figure_src: Option<String>,
    pub figure_alt: Option<String>,
    pub figure_srcset: Option<String>,
    pub figure_title: Option<String>,
    pub figure_caption: Option<String>,

    pub category: Option<String>,
    /// 0: Uncategorized
    /// 1: Accepted
    /// 2: Demoted, do not print (can be deleted)
    /// 3: Bullet Points have been created and the text is in the print_articles
    #[serde(skip_serializing)]
    status: i32,
}

impl Article {
    pub(crate) fn merge(&mut self, other: &dlf::Article) {
        self.title = other.title.clone();
        self.date = PrimitiveDateTime::parse(&other.date, &DATETIME_FORMAT).unwrap();
        self.locale_date = other.locale_date.clone();
        self.content_html = Some(other.html.clone());
        self.content = Some(other.plaintext.clone());
        self.description = Some(other.description.clone());
        self.kicker = Some(other.kicker.clone());

        if !other.figures.is_empty() {
            self.figure_src = Some(other.figures[0].src.clone());
            self.figure_alt = Some(other.figures[0].alt.clone());
            self.figure_srcset = Some(other.figures[0].srcset.clone());
            self.figure_title = Some(other.figures[0].title.clone());
            self.figure_caption = Some(other.figures[0].caption.clone());
        }
    }
}

impl Default for Article {
    fn default() -> Self {
        Article {
            key: String::new(),
            title: String::new(),
            teaser_headline: String::new(),
            teaser_text: String::new(),
            date: PrimitiveDateTime::MIN,
            locale_date: String::new(),
            kicker: None,
            description: None,
            content: None,
            content_html: None,
            figure_src: None,
            figure_alt: None,
            figure_srcset: None,
            figure_title: None,
            figure_caption: None,
            category: None,
            status: 0,
        }
    }
}

impl From<&PartialArticle> for Article {
    fn from(partial: &PartialArticle) -> Self {
        Article {
            key: partial.key.clone(),
            title: partial.title.clone(),
            teaser_headline: partial.teaser_headline.clone().unwrap_or_default().clone(),
            teaser_text: partial.teaser_text.clone().unwrap_or_default().clone(),
            date: PrimitiveDateTime::parse(&partial.date, &DATETIME_FORMAT).unwrap(),
            locale_date: partial.locale_date.clone(),
            ..Default::default()
        }
    }
}

pub enum ArticleStatus {
    Uncategorized,
    Accepted,
    Demoted,
    BulletsCreated,
}

impl From<i32> for ArticleStatus {
    fn from(status: i32) -> Self {
        match status {
            0 => ArticleStatus::Uncategorized,
            1 => ArticleStatus::Accepted,
            2 => ArticleStatus::Demoted,
            3 => ArticleStatus::BulletsCreated,
            _ => ArticleStatus::Uncategorized,
        }
    }
}

impl From<ArticleStatus> for i32 {
    fn from(status: ArticleStatus) -> Self {
        match status {
            ArticleStatus::Uncategorized => 0,
            ArticleStatus::Accepted => 1,
            ArticleStatus::Demoted => 2,
            ArticleStatus::BulletsCreated => 3,
        }
    }
}
