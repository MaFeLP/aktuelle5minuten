use crate::models::ArticleStatus;
use crate::{DbConn, ServerError};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::http::Status;
use rocket::serde::Serialize;
use rocket_dyn_templates::{Template, context};
use time::Date;
use time::macros::format_description;

#[get("/dates")]
pub(crate) async fn dates(db: DbConn) -> Result<Template, Status> {
    use crate::schema::articles::dsl;

    #[derive(Serialize)]
    struct TemplateDate {
        string: String,
        iso: String,
    }

    impl TryFrom<Date> for TemplateDate {
        type Error = time::error::Format;

        fn try_from(date: Date) -> Result<Self, Self::Error> {
            Ok(Self {
                string: date
                    .format(format_description!("[year]-[month]-[day]"))?
                    .to_string(),
                iso: date
                    .format(format_description!("[year]-[month]-[day]"))?
                    .to_string(),
            })
        }
    }

    let dates: Vec<TemplateDate> = db
        .run(move |c| {
            dsl::articles
                .select(diesel::dsl::date(dsl::date))
                .distinct()
                .filter(dsl::status.eq(i32::from(ArticleStatus::Uncategorized)))
                .order(dsl::date.asc())
                .load::<Date>(c)
                .map_err(|err| {
                    error!("Failed to load dates: {}", err);
                    ServerError::DatabaseError(err)
                })
        })
        .await?
        .into_iter()
        .filter_map(|date| TemplateDate::try_from(date).ok())
        .collect();

    Ok(Template::render(
        "dates",
        context! {
            has_dates: !dates.is_empty(),
            dates: dates,
        },
    ))
}
