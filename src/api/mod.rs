use crate::models::DATE_FORMAT;
use crate::{server_error, DbConn};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;
use std::path::PathBuf;
use time::Date;

pub(crate) mod actions;
pub(crate) mod article;
pub(crate) mod category;

#[derive(Serialize, Default)]
pub struct AiStatus {
    pub chatgpt: bool,
    pub claude: bool,
}

#[get("/ai")]
pub fn ai_status() -> Json<AiStatus> {
    const TRUTHY_VALUES: [&str; 10] = [
        "1", "true", "TRUE", "True", "yes", "YES", "Yes", "on", "ON", "On",
    ];
    Json(AiStatus {
        chatgpt: TRUTHY_VALUES
            .contains(&std::env::var("A5M_AI_CHATGPT").unwrap_or_default().as_str()),
        claude: TRUTHY_VALUES
            .contains(&std::env::var("A5M_AI_CLAUDE").unwrap_or_default().as_str()),
    })
}

#[derive(Serialize, Default, Debug)]
pub struct CountResponse {
    pub articles: i64,
    pub categories: i64,
}

#[get("/count?<date>")]
pub async fn count(conn: DbConn, date: Option<String>) -> Result<Json<CountResponse>, Status> {
    use crate::schema::articles::dsl;

    let categories = conn
        .run(|c| {
            dsl::articles
                .select(diesel::dsl::count(dsl::category))
                .first::<i64>(c)
                .map_err(|_| Status::InternalServerError)
        })
        .await?;

    if let Some(date) = date {
        let date = Date::parse(&date, DATE_FORMAT).map_err(|_| Status::BadRequest)?;
        let articles = conn
            .run(move |c| {
                server_error!(dsl::articles
                    .select(diesel::dsl::count_star())
                    .filter(diesel::dsl::date(dsl::date).eq(date))
                    .first::<i64>(c))
            })
            .await?;
        Ok(Json(CountResponse {
            articles,
            categories,
        }))
    } else {
        let articles = conn
            .run(|c| {
                server_error!(dsl::articles
                    .select(diesel::dsl::count_star())
                    .first::<i64>(c))
            })
            .await?;
        Ok(Json(CountResponse {
            articles,
            categories,
        }))
    }
}

#[get("/files")]
pub async fn files() -> Result<Json<Vec<String>>, Status> {
    let pdfs: Vec<String> = std::fs::read_dir(
        PathBuf::from(std::env::var("A5M_DATA_PATH").unwrap_or("/data".to_string())).join("pdfs"),
    )
    .map_err(|err| {
        error!("Error reading the PDF list from disk: {:?}", err);
        Status::InternalServerError
    })?
    .map(|name| {
        let name = name.unwrap();
        name.file_name().into_string().unwrap()
    })
    .collect();

    Ok(Json(pdfs))
}
