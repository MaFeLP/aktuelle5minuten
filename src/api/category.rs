//#[post("/bullets", data = "<bullet>")]

use crate::DbConn;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{http::Status, serde::json::Json};

const DEFAULT_CATEGORIES: [&str; 7] = [
    "Aktuelles Ereignis",
    "Außenpolitik",
    "Hamburg",
    "Politik",
    "Sonstiges",
    "USA",
    "Wirtschaft",
];

#[get("/all?<print>")]
pub async fn get_all_categories(
    conn: DbConn,
    print: Option<bool>,
) -> Result<Json<Vec<String>>, Status> {
    use crate::schema::articles::dsl;

    let mut categories: Vec<String> = conn
        .run(move |c| {
            dsl::articles
                .select(dsl::category)
                .filter(dsl::category.is_not_null())
                .distinct()
                .load::<Option<String>>(c)
                .map_err(|_| Status::InternalServerError)
        })
        .await?
        // We can safely unwrap here, as the filter in the query filters out non-null values.
        .iter()
        .map(|c| c.as_ref().unwrap().clone())
        .collect();

    if let Some(print) = print {
        if print {
            for category in DEFAULT_CATEGORIES {
                if !categories.contains(&category.to_string()) {
                    categories.push(category.to_string());
                }
            }
        }
    } else {
        for category in DEFAULT_CATEGORIES {
            categories.push(category.to_string());
        }
    }

    categories.sort();

    Ok(Json(categories))
}
