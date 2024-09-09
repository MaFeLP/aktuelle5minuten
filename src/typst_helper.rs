use comemo::Prehashed;
use typst::diag::FileResult;
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source};
use typst::text::{Font, FontBook};
use typst::Library;

/// Main interface that determines the environment for Typst.
pub struct SystemWorld {
    /// The content of a source.
    source: Source,

    /// The standard library.
    library: Prehashed<Library>,

    /// Metadata about all known fonts.
    book: Prehashed<FontBook>,

    /// Metadata about all known fonts.
    fonts: Vec<Font>,

    /// Datetime.
    time: time::OffsetDateTime,
}

impl SystemWorld {
    pub fn new(source: String) -> Self {
        let fonts = std::fs::read_dir(
            std::env::var("A5M_FONTS_DIR").unwrap_or("/usr/share/fonts/liberation/".to_string()),
        )
        .expect("Could not read fonts from disk")
        .map(Result::unwrap)
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "ttf"))
        .flat_map(|entry| {
            let path = entry.path();
            let bytes = std::fs::read(&path).unwrap();
            let buffer = Bytes::from(bytes);
            let face_count = ttf_parser::fonts_in_collection(&buffer).unwrap_or(1);
            (0..face_count).map(move |face| {
                Font::new(buffer.clone(), face).unwrap_or_else(|| {
                    panic!("failed to load font from {path:?} (face index {face})")
                })
            })
        })
        .collect();

        Self {
            library: Prehashed::new(Library::default()),
            book: Prehashed::new(FontBook::from_fonts(&fonts)),
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
    fn library(&self) -> &Prehashed<Library> {
        &self.library
    }

    /// Metadata about all known Books.
    fn book(&self) -> &Prehashed<FontBook> {
        &self.book
    }

    /// Accessing the main source file.
    fn main(&self) -> Source {
        self.source.clone()
    }

    /// Accessing a specified source file (based on `FileId`).
    fn source(&self, _: FileId) -> FileResult<Source> {
        panic!("Not implemented")
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
