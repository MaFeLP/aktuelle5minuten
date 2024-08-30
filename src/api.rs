use crate::{dlf, DbConn};
use diesel::{ExpressionMethods, QueryDsl, Queryable, RunQueryDsl, Table};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use time::{Date, PrimitiveDateTime};
use crate::dlf::PartialArticle;
use crate::schema::articles;
use diesel::Insertable;
use time::macros::format_description;

#[derive(Serialize, Debug, Queryable)]
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
    pub category: Option<String>,
    /// 0: Uncategorized
    /// 1: Accepted
    /// 2: Demoted, do not print (can be deleted)
    /// 3: Bullet Points have been created and the text is in the print_articles
    pub status: i32,
}

#[get("/api/article/get/first?<article_date>")]
pub async fn get_first_article(
    conn: DbConn,
    article_date: Option<String>,
) -> Result<Json<Article>, Status> {
    use crate::schema::articles::dsl::*;

    let article = match article_date {
        Some(article_date) => {
            let format = format_description!("[year]-[month]-[day]");
            let article_date = Date::parse(&article_date, &format).unwrap();
            conn.run(move |c| {
                articles
                    .select(articles::all_columns())
                    .filter(diesel::dsl::date(date).eq(article_date))
                    .first::<Article>(c)
                    .map_err(|_| Status::NotFound)
            })
            .await?
        }
        None => {
            conn.run(|c| {
                articles
                    .select(articles::all_columns())
                    .first::<Article>(c)
                    .map_err(|_| Status::NotFound)
            })
            .await?
        }
    };
    let parsed_article = dlf::article(&format!("{}{}", dlf::PREFIX, &article.key))
        .await
        .unwrap();
    let article_key = article.key.clone();
    if article.key != parsed_article.key {
        conn.run(move |c|
        // Article has been updated and moved! Work with the newer version and discard the older one!
        diesel::update(articles.filter(key.eq(&article_key)))
            .set(status.eq(2))
            .execute(c)
            .map_err(|_| Status::InternalServerError))
            .await?;
    }

    Ok(Json(article))
}

#[derive(Insertable, Debug)]
#[diesel(table_name = articles)]
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

// A route /api/actions/load that gets the latest dlf::wockenrueckblick and then inserts all the articles into the articles database
#[get("/api/actions/load")]
pub async fn load_articles(conn: DbConn) -> Result<Status, Status> {
    use crate::schema::articles::dsl::*;

    let wochenrueckblick_articles = dlf::wochenrueckblick()
        .await
        .map_err(|_| Status::InternalServerError)?;
    for article in wochenrueckblick_articles {
        let article_key = article.key.clone();
        let new_article = NewArticle::from(&article);
        let inserted_article = conn
            .run(move |c| {
                diesel::insert_into(articles)
                    .values(&new_article)
                    .on_conflict(key)
                    .do_nothing()
                    .execute(c)
                    .map_err(|_| Status::InternalServerError)
            })
            .await?;
        if inserted_article != 1 {
            conn.run(move |c| {
                diesel::update(crate::schema::articles::table.filter(key.eq(&article_key)))
                    .set(status.eq(1))
                    .execute(c)
                    .map_err(|_| Status::InternalServerError)
            })
            .await?;
        }
    }
    Ok(Status::Ok)
}
