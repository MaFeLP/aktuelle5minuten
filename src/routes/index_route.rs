use crate::ServerError;
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};
use std::ops::Sub;
use time::macros::format_description;
use time::Duration;

#[get("/")]
pub(crate) async fn index() -> Result<Template, Status> {
    let today = time::OffsetDateTime::now_local()
        .unwrap_or(time::OffsetDateTime::now_utc())
        .date();
    let last_week = today.sub(Duration::days(7));

    Ok(Template::render(
        "index",
        context! {
            start_date: today.format(format_description!("[year]-[month]-[day]")).map_err(|err| ServerError::DateFormat(err))?.to_string(),
            end_date: last_week.format(format_description!("[year]-[month]-[day]")).map_err(|err| ServerError::DateFormat(err))?.to_string(),
        },
    ))
}
