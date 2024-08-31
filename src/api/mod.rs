use rocket::serde::json::Json;
use serde::Serialize;

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
        chatgpt: TRUTHY_VALUES.contains(&std::env::var("CHATGPT").unwrap_or_default().as_str()),
        claude: TRUTHY_VALUES.contains(&std::env::var("CLAUDE").unwrap_or_default().as_str()),
    })
}
