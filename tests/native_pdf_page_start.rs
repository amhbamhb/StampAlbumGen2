use oxidize_pdf::parser::PdfDocument;
use stamp_album_gen2::native_pdf::{album::NativeAlbum, albumeasy};
use std::{error::Error, path::Path};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn fixture_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/page-start-v1")
}

#[test]
fn render_all_page_start_fixtures_to_native_pdfs() -> Result<()> {
    let root = fixture_dir();

    // 1. Basic explicit page start
    let basic_native_path = root.join("basic.stampalbum");
    let basic_native_pdf = root.join("basic-native.pdf");
    let album_basic = NativeAlbum::load(&basic_native_path)?;
    let basic_pages = album_basic.write_pdf(&basic_native_pdf)?;
    assert_eq!(basic_pages.len(), 2);
    let doc_basic = PdfDocument::open(&basic_native_pdf)?;
    assert_eq!(doc_basic.page_count()?, 2);

    let basic_raw_path = root.join("basic-raw.stampalbum");
    let basic_raw_pdf = root.join("basic-raw-native.pdf");
    let basic_raw_count = albumeasy::render_file(&basic_raw_path, &basic_raw_pdf)?;
    assert_eq!(basic_raw_count, 2);
    let doc_basic_raw = PdfDocument::open(&basic_raw_pdf)?;
    assert_eq!(doc_basic_raw.page_count()?, 2);

    // 2. Album group template page start
    let album_native_path = root.join("album.stampalbum");
    let album_native_pdf = root.join("album-native.pdf");
    let album_obj = NativeAlbum::load(&album_native_path)?;
    let album_pages = album_obj.write_pdf(&album_native_pdf)?;
    assert_eq!(album_pages.len(), 2);
    let doc_album = PdfDocument::open(&album_native_pdf)?;
    assert_eq!(doc_album.page_count()?, 2);

    let album_raw_path = root.join("album-raw.stampalbum");
    let album_raw_pdf = root.join("album-raw-native.pdf");
    let album_raw_count = albumeasy::render_file(&album_raw_path, &album_raw_pdf)?;
    assert_eq!(album_raw_count, 2);
    let doc_album_raw = PdfDocument::open(&album_raw_pdf)?;
    assert_eq!(doc_album_raw.page_count()?, 2);

    // 3. Intentional blank pages
    let blank_native_path = root.join("blank-pages.stampalbum");
    let blank_native_pdf = root.join("blank-pages-native.pdf");
    let blank_album = NativeAlbum::load(&blank_native_path)?;
    let blank_pages = blank_album.write_pdf(&blank_native_pdf)?;
    assert_eq!(blank_pages.len(), 5);
    let doc_blank = PdfDocument::open(&blank_native_pdf)?;
    assert_eq!(doc_blank.page_count()?, 5);

    let blank_raw_path = root.join("blank-pages-raw.stampalbum");
    let blank_raw_pdf = root.join("blank-pages-raw-native.pdf");
    let blank_raw_count = albumeasy::render_file(&blank_raw_path, &blank_raw_pdf)?;
    assert_eq!(blank_raw_count, 5);
    let doc_blank_raw = PdfDocument::open(&blank_raw_pdf)?;
    assert_eq!(doc_blank_raw.page_count()?, 5);

    // 4. All-in-one comprehensive showcase
    let all_native_path = root.join("all-in-one-compare.stampalbum");
    let all_native_pdf = root.join("all-in-one-compare-native.pdf");
    let all_album = NativeAlbum::load(&all_native_path)?;
    let all_pages = all_album.write_pdf(&all_native_pdf)?;
    assert_eq!(all_pages.len(), 9);
    let doc_all = PdfDocument::open(&all_native_pdf)?;
    assert_eq!(doc_all.page_count()?, 9);

    let all_raw_path = root.join("all-in-one-compare-raw.stampalbum");
    let all_raw_pdf = root.join("all-in-one-compare-raw-native.pdf");
    let all_raw_count = albumeasy::render_file(&all_raw_path, &all_raw_pdf)?;
    assert_eq!(all_raw_count, 9);
    let doc_all_raw = PdfDocument::open(&all_raw_pdf)?;
    assert_eq!(doc_all_raw.page_count()?, 9);

    Ok(())
}

#[test]
fn native_page_start_aliases_and_spacing() -> Result<()> {
    let temporary = tempfile::tempdir()?;

    // Test NATIVE_PAGE_START alias in native album mode
    let native_source = r#"STAMPALBUM 1
ALBUM_PAGES_SIZE (215.9 279.4)
ALBUM_PAGES_MARGINS (15 15 15 15)
ALBUM_PAGES_SPACING (4 14)
ALBUM_PAGES_BORDER (0.2 0 0 0)
COLOUR_ALBUM_BORDER (black)
NATIVE_ROW_ALIGNMENT (center)
NATIVE_STAMP_FRAME (0.1 black 6)
NATIVE_STAMP_TEXT_POLICY (wrap_to_frame percent 100 0 0 1.5)
NATIVE_STAMP_IMAGE_POLICY (millimetres 0 0 false preserve colour show)

STAMP_IMAGE ("/media/ray/2026-01-11/Rust/Rust/StampAlbumGen2/tests/fixtures/albumeasy/stamp-pagination-v1/us-150.png" 52 29 0.05078125 "HN" 8 "PAGE 1")
NATIVE_PAGE_START (5 15)
STAMP_IMAGE ("/media/ray/2026-01-11/Rust/Rust/StampAlbumGen2/tests/fixtures/albumeasy/stamp-pagination-v1/us-151.png" 26 29 0.0534979424 "HN" 8 "PAGE 2")
PAGE_START ()
STAMP_IMAGE ("/media/ray/2026-01-11/Rust/Rust/StampAlbumGen2/tests/fixtures/albumeasy/stamp-pagination-v1/us-152.png" 26 29 0.052 "HN" 8 "PAGE 3")
"#;

    let native_file = temporary.path().join("aliases-native.stampalbum");
    std::fs::write(&native_file, native_source)?;
    let native_pdf = temporary.path().join("aliases-native.pdf");
    let album = NativeAlbum::load(&native_file)?;
    let pages = album.write_pdf(&native_pdf)?;
    assert_eq!(pages.len(), 3);
    let doc = PdfDocument::open(&native_pdf)?;
    assert_eq!(doc.page_count()?, 3);

    // Test NATIVE_PAGE_BREAK alias in raw AlbumEasy mode
    let raw_source = r#"ALBUM_PAGES_SIZE (215.9 279.4)
ALBUM_PAGES_MARGINS (15 15 15 15)
ALBUM_PAGES_SPACING (4 14)
ALBUM_PAGES_BORDER (0.2 0 0 0)

PAGE_START
ROW_START_ES (HN 8 0.1)
STAMP_ADD_IMG (58 35 "/media/ray/2026-01-11/Rust/Rust/StampAlbumGen2/tests/fixtures/albumeasy/stamp-pagination-v1/us-150.png" "" "PAGE 1" "")

NATIVE_PAGE_BREAK (6 18)
ROW_START_ES (HN 8 0.1)
STAMP_ADD_IMG (32 35 "/media/ray/2026-01-11/Rust/Rust/StampAlbumGen2/tests/fixtures/albumeasy/stamp-pagination-v1/us-151.png" "" "PAGE 2" "")
"#;

    let raw_file = temporary.path().join("aliases-raw.stampalbum");
    std::fs::write(&raw_file, raw_source)?;
    let raw_pdf = temporary.path().join("aliases-raw.pdf");
    let count = albumeasy::render_file(&raw_file, &raw_pdf)?;
    assert_eq!(count, 2);
    let doc_raw = PdfDocument::open(&raw_pdf)?;
    assert_eq!(doc_raw.page_count()?, 2);

    Ok(())
}
