use crate::server_error;
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
            start_date: server_error!("Could not format the date: {}", today.format(format_description!("[year]-[month]-[day]")))?.to_string(),
            end_date: server_error!("Could not format the date: {}", last_week.format(format_description!("[year]-[month]-[day]")))?.to_string(),
        },
    ))
}
