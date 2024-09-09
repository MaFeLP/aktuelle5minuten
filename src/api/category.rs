use crate::models::{ArticleStatus, DATETIME_FORMAT};
use crate::typst_helper::SystemWorld;
use crate::{regex, DbConn};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::form::{Form, FromForm};
use rocket::response::Redirect;
use rocket::{http::Status, serde::json::Json};
use serde::Serialize;
use std::path::PathBuf;
use typst::diag::Severity;
use typst::foundations::Smart;

const DEFAULT_CATEGORIES: [&str; 7] = [
    "Aktuelles Ereignis",
    "Außenpolitik",
    "Hamburg",
    "Politik",
    "Sonstiges",
    "USA",
    "Wirtschaft",
];

#[derive(FromForm)]
pub struct BulletsForm<'a> {
    #[field(validate = len(1..63))]
    pub category: &'a str,
    #[field(validate = len(1..))]
    pub bullets: &'a str,
}

fn typst_escape(text: &str) -> String {
    text.replace("#", "\\#")
        .replace("*", "\\*")
        .replace("_", "\\_")
        .replace("~", "\\~")
        .replace("`", "\\`")
        .replace("<", "\\<")
        .replace(">", "\\>")
        .replace("@", "\\@")
        .replace("$", "\\$")
        .replace("%", "\\%")
        .replace("^", "\\^")
        .replace("\\", "\\\\")
        .replace("-?", "\\-?")
}

#[post("/bullets", data = "<bullets>")]
pub async fn bullets(conn: DbConn, bullets: Form<BulletsForm<'_>>) -> Result<Redirect, Status> {
    use crate::schema::articles::dsl;
    use crate::schema::print_articles::dsl as print_dsl;

    let category = typst_escape(bullets.category);
    let category2 = category.clone();
    let category3 = category.clone();
    let text = typst_escape(bullets.bullets);

    conn.run(move |c| {
        diesel::insert_into(print_dsl::print_articles)
            .values((
                print_dsl::category.eq(category),
                print_dsl::bullets.eq(text),
            ))
            .execute(c)
            .map_err(|_| Status::InternalServerError)
    })
    .await?;

    let affected = conn
        .run(move |c| {
            diesel::update(dsl::articles.filter(dsl::category.eq(category2)))
                .set(dsl::status.eq(i32::from(ArticleStatus::BulletsCreated)))
                .execute(c)
                .map_err(|_| Status::InternalServerError)
        })
        .await?;

    info!(
        "{} articles have been marked 'bullets created' for category {}",
        affected, &category3
    );

    // Check how many categories are left to process
    let categories = conn
        .run(move |c| {
            dsl::articles
                .select(diesel::dsl::count_star())
                .filter(dsl::category.is_not_null())
                .filter(dsl::status.eq(i32::from(ArticleStatus::Accepted)))
                .distinct()
                .first::<i64>(c)
                .map_err(|_| Status::InternalServerError)
        })
        .await?;

    info!("{} categories left to process", categories);

    // There are still categories to process
    if categories != 0 {
        return Ok(Redirect::to("/pdfcreate"));
    }

    // Prepare the typst content
    let mut content = include_str!("typst_template.typ")
        .replace(
            "{{ author }}",
            &std::env::var("A5M_PDF_AUTHOR").unwrap_or("Default Author".to_string()),
        )
        .replace(
            "{{ title }}",
            &std::env::var("A5M_PDF_TITLE").unwrap_or("Aktuelle 5 Minuten".to_string()),
        );

    let bullets = conn
        .run(move |c| {
            print_dsl::print_articles
                .select((print_dsl::category, print_dsl::bullets))
                .load::<(String, String)>(c)
                .map_err(|_| Status::InternalServerError)
        })
        .await?;

    info!("Found {} categories to print to the pdf", bullets.len());

    for (category, bullets) in bullets {
        content.push_str(&format!("= {}\n", category));
        content.push_str(&bullets);
    }

    // Compile the document with typst
    let world = SystemWorld::new(content);
    let mut tracer = typst::eval::Tracer::default();
    let document = match typst::compile(&world, &mut tracer) {
        Ok(d) => d,
        Err(e) => {
            error!("Failed to compile typst document");
            for diagnostic in tracer.warnings().iter().chain(e.iter()) {
                match diagnostic.severity {
                    Severity::Error => {
                        error!("{}", diagnostic.message);
                        for hint in &diagnostic.hints {
                            error!("hint: {}", hint);
                        }
                        for point in &diagnostic.trace {
                            error!("at {:?}:{}", point.span, point.v);
                        }
                    }
                    Severity::Warning => {
                        warn!("{}", diagnostic.message);
                        for hint in &diagnostic.hints {
                            warn!("hint: {}", hint);
                        }
                    }
                }
            }
            panic!("Failed to compile typst document");
        }
    };

    let now = time::OffsetDateTime::now_utc();
    let pdf = typst_pdf::pdf(&document, Smart::Auto, None);
    let path = PathBuf::from(std::env::var("A5M_DATA_PATH").unwrap_or("/data".to_string()))
        .join("pdfs")
        .join(format!("{}.pdf", now.format(&DATETIME_FORMAT).unwrap()));
    std::fs::write(path, pdf).expect("Failed to write pdf");

    let pdf_uri = format!("/pdfs/{}.pdf", now.format(&DATETIME_FORMAT).unwrap());

    Ok(Redirect::to(pdf_uri))
}

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
        .iter()
        // We can safely unwrap here, as the filter in the query filters out non-null values.
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

#[derive(Serialize, Debug)]
pub struct CategoryResponse {
    pub category: String,
    pub count: usize,
    pub text: String,
}

#[get("/summary?<category>")]
pub async fn get_category_summary(
    conn: DbConn,
    category: String,
) -> Result<Json<CategoryResponse>, Status> {
    use crate::schema::articles::dsl;

    let category_return = category.clone();
    let contents: Vec<String> = conn
        .run(move |c| {
            dsl::articles
                .select(dsl::content)
                .filter(dsl::content.is_not_null())
                .filter(dsl::category.eq(category))
                .distinct()
                .load::<Option<String>>(c)
                .map_err(|_| Status::InternalServerError)
        })
        .await?
        // We can safely unwrap here, as the filter in the query filters out non-null values.
        .iter()
        .map(|s| s.as_ref().unwrap().clone())
        // Remove copyright notice (is not needed for the AI)
        .map(|s| regex!(r#"Diese Nachricht wurde am \d{2}\.\d{2}\.\d{4} im Programm Deutschlandfunk gesendet\."#).replace_all(&s, "").to_string())
        .collect();

    Ok(Json(CategoryResponse {
        category: category_return,
        count: contents.len(),
        text: contents.join("\n"),
    }))
}

#[cfg(test)]
mod test {
    use crate::typst_helper::SystemWorld;
    use typst::diag::Severity;
    use typst::foundations::Smart;

    #[test]
    fn typst_compile() {
        let content = include_str!("typst_template.typ")
            .replace("{{ author }}", "Test")
            .replace("{{ title }}", "Test");

        let world = SystemWorld::new(content);
        let mut tracer = typst::eval::Tracer::default();
        let document = match typst::compile(&world, &mut tracer) {
            Ok(d) => d,
            Err(e) => {
                error!("Failed to compile typst document");
                for diagnostic in tracer.warnings().iter().chain(e.iter()) {
                    match diagnostic.severity {
                        Severity::Error => {
                            error!("{}", diagnostic.message);
                            for hint in &diagnostic.hints {
                                error!("hint: {}", hint);
                            }
                            for point in &diagnostic.trace {
                                error!("at {:?}:{}", point.span, point.v);
                            }
                        }
                        Severity::Warning => {
                            warn!("{}", diagnostic.message);
                            for hint in &diagnostic.hints {
                                warn!("hint: {}", hint);
                            }
                        }
                    }
                }
                panic!("Failed to compile typst document");
            }
        };

        let pdf = typst_pdf::pdf(&document, Smart::Auto, None);
        std::fs::write("output.pdf", pdf).expect("Failed to write pdf");
    }
}
