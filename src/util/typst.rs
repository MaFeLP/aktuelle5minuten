use crate::{DbConn, ServerError};
use std::io::Write;
use std::path::PathBuf;
use typst::Library;
use typst::LibraryExt;
use typst::diag::FileResult;
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;

/// Main interface that determines the environment for Typst.
pub struct SystemWorld {
    /// The content of a source.
    source: Source,

    /// The standard library.
    library: LazyHash<Library>,

    /// Metadata about all known fonts.
    book: LazyHash<FontBook>,

    /// Metadata about all known fonts.
    fonts: Vec<Font>,

    /// Datetime.
    time: time::OffsetDateTime,
}

impl SystemWorld {
    pub fn new(source: String) -> Self {
        let fonts: Vec<Font> = std::fs::read_dir(
            std::env::var("A5M_FONTS_DIR")
                .unwrap_or("/usr/share/fonts/truetype/liberation/".to_string()),
        )
        .expect("Could not read fonts from disk")
        .map(Result::unwrap)
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "ttf"))
        .flat_map(|entry| {
            let path = entry.path();
            let bytes = std::fs::read(&path).unwrap();
            let buffer = Bytes::new(bytes);
            let face_count = ttf_parser::fonts_in_collection(&buffer).unwrap_or(1);
            (0..face_count).map(move |face| {
                Font::new(buffer.clone(), face).unwrap_or_else(|| {
                    panic!("failed to load font from {path:?} (face index {face})")
                })
            })
        })
        .collect();

        Self {
            library: LazyHash::new(Library::default()),
            book: LazyHash::new(FontBook::from_fonts(&fonts)),
            fonts,
            source: Source::detached(source),
            time: time::OffsetDateTime::now_utc(),
        }
    }
}

/// This is the interface we have to implement such that `typst` can compile it.
///
/// I have tried to keep it as minimal as possible
impl typst::World for SystemWorld {
    /// Standard library.
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    /// Metadata about all known Books.
    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    /// Accessing the main source file.
    fn main(&self) -> FileId {
        self.source.id()
    }

    /// Accessing a specified source file (based on `FileId`).
    fn source(&self, file_id: FileId) -> FileResult<Source> {
        if file_id != self.source.id() {
            Err(typst::diag::FileError::NotFound(PathBuf::new()))
        } else {
            Ok(self.source.clone())
        }
    }

    /// Accessing a specified file (non-file).
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        panic!("Not needed!")
    }

    /// Accessing a specified font per index of font book.
    fn font(&self, id: usize) -> Option<Font> {
        self.fonts.get(id).cloned()
    }

    /// Get the current date.
    ///
    /// Optionally, an offset in hours is given.
    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let offset = offset.unwrap_or(0);
        let offset = time::UtcOffset::from_hms(offset.try_into().ok()?, 0, 0).ok()?;
        let time = self.time.checked_to_offset(offset)?;
        Some(Datetime::Date(time.date()))
    }
}

pub async fn create_typst_pdf(conn: DbConn, filename: &str) -> Result<Vec<u8>, ServerError> {
    use crate::schema::print_articles::dsl as print_dsl;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
    use typst::diag::Severity;
    use typst::foundations::Smart;
    use typst_pdf::{PdfOptions, PdfStandard, PdfStandards};

    //let content = Template::show("pdf", context! {}).ok_or(ServerError::TemplateError)?;

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
                .filter(print_dsl::printed.eq(false))
                .load::<(String, String)>(c)
        })
        .await?;

    info!("Found {} categories to print to the pdf", bullets.len());

    for (category, bullets) in bullets {
        content.push_str(&format!("\n\n= {}\n", category));
        content.push_str(&bullets);
    }

    if std::path::Path::new("/tmp/typst_builder.typ").exists() {
        std::fs::remove_file("/tmp/typst_builder.typ").unwrap();
    }
    let mut file = std::fs::File::create("/tmp/typst_builder.typ").unwrap();
    file.write_all(content.as_bytes()).unwrap();

    // Compile the document with typst
    let world = SystemWorld::new(content);
    let compile_result = typst::compile(&world);
    let document = compile_result.output?;
    for diagnostic in compile_result.warnings {
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

    let now = time::OffsetDateTime::now_utc();
    let options = PdfOptions {
        ident: Smart::Auto,
        timestamp: Some(typst_pdf::Timestamp::new_utc(
            typst::foundations::Datetime::from_ymd_hms(
                now.year(),
                now.month().into(),
                now.day(),
                now.hour(),
                now.minute(),
                now.second(),
            )
            .unwrap(),
        )),
        page_ranges: None, // Export all pages
        standards: PdfStandards::new(&[PdfStandard::A_2b]).unwrap(),
        tagged: true,
    };

    let pdf = typst_pdf::pdf(&document, &options)?;
    let pdf_directory =
        PathBuf::from(std::env::var("A5M_DATA_PATH").unwrap_or("/data".to_string())).join("pdfs");
    if !pdf_directory.as_path().exists() {
        std::fs::create_dir_all(&pdf_directory)
            .expect("Could not create pdf directory on the filesystem");
    }
    let path = pdf_directory.join(filename);
    std::fs::write(path, &pdf)?;

    let affected = conn
        .run(move |c| {
            diesel::update(print_dsl::print_articles)
                .set(print_dsl::printed.eq(true))
                .execute(c)
        })
        .await?;
    info!("{} articles have been marked as printed", affected);
    Ok(pdf)
}

#[cfg(test)]
mod test {
    use super::*;
    use typst::diag::{Severity, SourceDiagnostic};
    use typst::ecow::EcoVec;

    #[test]
    fn typst_compile() -> Result<(), EcoVec<SourceDiagnostic>> {
        let content = include_str!("typst_template.typ")
            .replace("{{ author }}", "Test")
            .replace("{{ title }}", "Test");

        let world = SystemWorld::new(content);
        let compile_result = typst::compile(&world);
        let document = compile_result.output?;
        for diagnostic in compile_result.warnings {
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

        let options = typst_pdf::PdfOptions {
            ident: typst::foundations::Smart::Auto,
            timestamp: None,
            page_ranges: None, // Export all pages
            standards: typst_pdf::PdfStandards::new(&[typst_pdf::PdfStandard::A_2b]).unwrap(),
            tagged: true,
        };

        let pdf = typst_pdf::pdf(&document, &options)?;
        std::fs::write("output.pdf", pdf).expect("Failed to write pdf");

        Ok(())
    }
}
