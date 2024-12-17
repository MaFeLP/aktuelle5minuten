#![warn(clippy::unwrap_used)]
#![warn(clippy::clone_on_copy)]

#[macro_use]
extern crate rocket;
mod api;
mod htmx;
mod models;
mod routes;
mod schema;
mod scrapers;
mod util;

#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

#[macro_export]
macro_rules! server_error {
    ($err:expr) => {
        $err.map_err(|err| {
            error!("Internal Server Error: {}", err);
            rocket::http::Status::InternalServerError
        })
    };
    ($reason:literal, $err:expr) => {{
        $err.map_err(|err| {
            error!($reason, err);
            rocket::http::Status::InternalServerError
        })
    }};
}

use diesel::{sqlite::Sqlite, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Orbit, Rocket};
use rocket_dyn_templates::handlebars::{
    Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError,
    RenderErrorReason,
};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use std::path::{Path, PathBuf};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[database("sqlite_db")]
pub struct DbConn(SqliteConnection);

struct MigrationsFairing;

#[async_trait]
impl Fairing for MigrationsFairing {
    fn info(&self) -> Info {
        Info {
            name: "Apply Migrations",
            kind: Kind::Liftoff,
        }
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        fn apply_migrations(
            connection: &mut impl MigrationHarness<Sqlite>,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            // This will run the necessary migrations.
            //
            // See the documentation for `MigrationHarness` for
            // all available methods.
            connection.run_pending_migrations(MIGRATIONS)?;

            Ok(())
        }

        let db = DbConn::get_one(rocket)
            .await
            .expect("Could not get a database connection to apply the migrations");
        db.run(apply_migrations)
            .await
            .expect("Applying migrations failed!");
        rocket::log::private::info!("Applied database migrations!");
    }
}

fn html_raw_helper_checked(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    Ok(out.write(
        h.param(0)
            .ok_or(RenderError::from(RenderErrorReason::ParamNotFoundForIndex(
                "0", 0,
            )))?
            .value()
            .as_str()
            .ok_or(RenderErrorReason::InvalidParamType("string"))?,
    )?)
}

#[launch]
fn rocket() -> _ {
    // Create the file structure to store pdfs
    let path =
        PathBuf::from(std::env::var("A5M_DATA_PATH").unwrap_or("/data".to_string())).join("pdfs");
    std::fs::create_dir_all(&path).expect("Could not create the file structure to store pdfs!");

    // Configure the rocket instance
    rocket::build()
        .attach(DbConn::fairing())
        .attach(MigrationsFairing)
        .attach(Template::custom(|engines| {
            engines
                .handlebars
                .register_helper("html", Box::new(html_raw_helper_checked))
        }))
        .mount(
            "/",
            routes![
                routes::index,
                routes::dates,
                routes::tinder,
                routes::pdflist,
                routes::pdfcreate,
            ],
        )
        .mount(
            "/assets",
            rocket::fs::FileServer::from(Path::new(
                &std::env::var("A5M_ASSETS_PATH").unwrap_or("/app/assets/".to_string()),
            )),
        )
        .mount(
            "/files",
            rocket::fs::FileServer::from(
                Path::new(&std::env::var("A5M_DATA_PATH").unwrap_or("/data".to_string()))
                    .join("pdfs"),
            ),
        )
        .mount("/api", routes![api::ai_status, api::count, api::files,])
        .mount(
            "/api/article",
            routes![
                api::article::get_first_article,
                api::article::get_article_by_key,
                api::article::get_all_article_dates,
                api::article::demote_article,
                api::article::promote_article,
            ],
        )
        .mount(
            "/api/actions",
            routes![
                api::actions::load_new_articles,
                api::actions::clean_articles,
            ],
        )
        .mount(
            "/api/category",
            routes![
                api::category::get_all_categories,
                api::category::get_category_summary,
                api::category::bullets,
            ],
        )
        .mount(
            "/htmx",
            routes![htmx::load_new_dlf_articles, htmx::tinder::demote_article,],
        )
}
