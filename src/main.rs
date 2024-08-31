mod api;
mod dlf;
mod models;
mod schema;

#[macro_use]
extern crate rocket;

#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

use diesel::{sqlite::Sqlite, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{fs::NamedFile, response::content::RawHtml, Orbit, Rocket};
use rocket_sync_db_pools::database;
use std::path::{Path, PathBuf};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

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

//const INDEX_HTML: &str = include_str!("../static/index.html");
const INDEX_HTML: &str = "Hello World!";

#[get("/")]
async fn index() -> RawHtml<&'static str> {
    RawHtml(INDEX_HTML)
}

#[get("/dates")]
async fn dates() -> RawHtml<&'static str> {
    RawHtml(INDEX_HTML)
}

#[get("/tinder")]
async fn tinder() -> RawHtml<&'static str> {
    RawHtml(INDEX_HTML)
}

#[get("/pdflist")]
async fn pdflist() -> RawHtml<&'static str> {
    RawHtml(INDEX_HTML)
}

#[get("/pdfcreate")]
async fn pdfcreate() -> RawHtml<&'static str> {
    RawHtml(INDEX_HTML)
}

#[get("/<file..>", rank = 2)]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(MigrationsFairing)
        .mount(
            "/",
            routes![index, dates, tinder, pdflist, pdfcreate, files,],
        )
        .mount(
            "/api/article/",
            routes![
                api::article::get_first_article,
                api::article::get_article_by_key,
                api::article::get_all_article_dates,
            ],
        )
        .mount("/api/actions/", routes![api::actions::load_new_articles,])
        .mount(
            "/api/category/",
            routes![api::category::get_all_categories,],
        )
}
