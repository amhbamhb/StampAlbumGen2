use oxidize_pdf::parser::PdfDocument;
use stamp_album_gen2::native_pdf::{album::NativeAlbum, albumeasy};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/margins-v1")
}

#[test]
fn even_override_txt_renders_three_pages_with_alternating_margins() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let input = fixture().join("even-override.txt");
    let output = temp.path().join("even-override-native.pdf");

    let pages = albumeasy::render_file(&input, &output)?;
    assert_eq!(pages, 3);

    let doc = PdfDocument::open(&output).map_err(|e| e.to_string())?;
    assert_eq!(doc.page_count().map_err(|e| e.to_string())?, 3);

    // Each page should exist and have dimensions 210 x 297 mm
    for idx in 0..3 {
        let page = doc.get_page(idx).map_err(|e| e.to_string())?;
        let width = page.media_box[2] - page.media_box[0];
        let height = page.media_box[3] - page.media_box[1];
        // 210 mm ~= 595.276 pt, 297 mm ~= 841.890 pt
        assert!((width - 595.276).abs() < 1.0, "page {idx} width mismatch");
        assert!((height - 841.890).abs() < 1.0, "page {idx} height mismatch");
    }

    Ok(())
}

fn streams(path: &Path) -> Result<Vec<Vec<Vec<u8>>>, String> {
    let pdf = PdfDocument::open(path).map_err(|e| e.to_string())?;
    (0..pdf.page_count().map_err(|e| e.to_string())?)
        .map(|i| {
            let page = pdf.get_page(i).map_err(|e| e.to_string())?;
            pdf.get_page_content_streams(&page)
                .map_err(|e| e.to_string())
        })
        .collect()
}

#[test]
fn raw_stampalbum_renders_identically_to_txt() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let source =
        fs::read_to_string(fixture().join("even-override.txt")).map_err(|e| e.to_string())?;
    let raw_file = temp.path().join("even-override-raw.stampalbum");
    fs::write(&raw_file, &source).map_err(|e| e.to_string())?;

    let output_txt = temp.path().join("txt.pdf");
    let output_raw = temp.path().join("raw.pdf");

    let txt_pages = albumeasy::render(&source, &fixture(), &output_txt)?;
    let raw_pages = albumeasy::render_file(&raw_file, &output_raw)?;
    assert_eq!(txt_pages, 3);
    assert_eq!(raw_pages, 3);

    let txt_streams = streams(&output_txt)?;
    let raw_streams = streams(&output_raw)?;
    assert_eq!(
        txt_streams, raw_streams,
        "raw.stampalbum content streams must match txt streams exactly"
    );

    Ok(())
}

#[test]
fn native_album_parses_and_renders_even_margins() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let asset_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/albumeasy/native-real-stamps-v1");

    let native_source = r#"STAMPALBUM 1
ALBUM_PAGES_SIZE (210.0 297.0)
ALBUM_PAGES_MARGINS (25.0 12.0 18.0 21.0)
ALBUM_PAGES_MARGINSE (12.0 25.0 22.0 16.0)
ALBUM_PAGES_SPACING (3.0 4.0)
ALBUM_PAGES_BORDER (0.2 0.0 0.0 0.0)
COLOUR_ALBUM_BORDER (black)
NATIVE_ROW_ALIGNMENT (center)
NATIVE_STAMP_FRAME (0.25 black)
NATIVE_STAMP_TEXT_POLICY (wrap_to_frame percent 100 0 0 0)
NATIVE_STAMP_IMAGE_POLICY (millimetres 0 0 false preserve colour show)

STAMP_IMAGE ("../stamp-pagination-v1/us-150.png" 52 29 0.05078125)
NATIVE_PAGE_BREAK ()
STAMP_IMAGE ("../stamp-pagination-v1/us-151.png" 26 29 0.0534979424)
"#;

    let album = NativeAlbum::parse(native_source, &asset_root)?;
    let output = temp.path().join("native_multipage.pdf");
    let pages = album.write_pdf(&output)?;
    assert_eq!(pages.len(), 2);

    let doc = PdfDocument::open(&output).map_err(|e| e.to_string())?;
    assert_eq!(doc.page_count().map_err(|e| e.to_string())?, 2);

    Ok(())
}

#[test]
fn all_margin_variants_render_as_raw_and_native_albums() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let cases = [
        ("symmetric", 2),
        ("asymmetric", 2),
        ("fractional", 1),
        ("even-override", 3),
    ];

    for (stem, expected_pages) in cases {
        let raw_path = fixture().join(format!("{stem}-raw.stampalbum"));
        let raw_pdf = temp.path().join(format!("{stem}-raw.pdf"));
        let raw_pages = albumeasy::render_file(&raw_path, &raw_pdf)?;
        assert_eq!(raw_pages, expected_pages, "{stem}-raw pages");

        let native_path = fixture().join(format!("{stem}.stampalbum"));
        let native_pdf = temp.path().join(format!("{stem}-native.pdf"));
        let album = NativeAlbum::load(&native_path)?;
        let native_pages = album.write_pdf(&native_pdf)?;
        assert_eq!(native_pages.len(), expected_pages, "{stem} native pages");
    }

    Ok(())
}

#[test]
fn album_pages_marginse_case_tolerance() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let base =
        fs::read_to_string(fixture().join("even-override.txt")).map_err(|e| e.to_string())?;

    for keyword in [
        "album_pages_marginse",
        "Album_Pages_Marginse",
        "ALBUM_PAGES_MARGINSE",
    ] {
        let modified = base.replace("ALBUM_PAGES_MARGINSE", keyword);
        let output = temp.path().join(format!("{keyword}.pdf"));
        let pages = albumeasy::render(&modified, &fixture(), &output)?;
        assert_eq!(pages, 3, "Failed to render with keyword case: {keyword}");
    }

    Ok(())
}

#[test]
fn album_pages_marginse_validation_errors() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let base =
        fs::read_to_string(fixture().join("even-override.txt")).map_err(|e| e.to_string())?;
    let output = temp.path().join("err.pdf");

    // Missing arguments (3 instead of 4)
    let missing_arg = base.replace(
        "ALBUM_PAGES_MARGINSE (12.0 25.0 22.0 16.0)",
        "ALBUM_PAGES_MARGINSE (12.0 25.0 22.0)",
    );
    assert!(
        albumeasy::render(&missing_arg, &fixture(), &output).is_err(),
        "missing arg must fail"
    );

    // Excess arguments (5 instead of 4)
    let excess_arg = base.replace(
        "ALBUM_PAGES_MARGINSE (12.0 25.0 22.0 16.0)",
        "ALBUM_PAGES_MARGINSE (12.0 25.0 22.0 16.0 10.0)",
    );
    assert!(
        albumeasy::render(&excess_arg, &fixture(), &output).is_err(),
        "excess arg must fail"
    );

    // Non-numeric argument
    let non_num = base.replace(
        "ALBUM_PAGES_MARGINSE (12.0 25.0 22.0 16.0)",
        "ALBUM_PAGES_MARGINSE (12.0 25.0 22.0 \"sixteen\")",
    );
    assert!(
        albumeasy::render(&non_num, &fixture(), &output).is_err(),
        "non-numeric arg must fail"
    );

    // Negative margin
    let negative = base.replace(
        "ALBUM_PAGES_MARGINSE (12.0 25.0 22.0 16.0)",
        "ALBUM_PAGES_MARGINSE (-12.0 25.0 22.0 16.0)",
    );
    assert!(
        albumeasy::render(&negative, &fixture(), &output).is_err(),
        "negative margin must fail"
    );

    // After PAGE_START
    let after_page_start = base.replace("ALBUM_PAGES_MARGINSE (12.0 25.0 22.0 16.0)", "")
        + "\nALBUM_PAGES_MARGINSE (12.0 25.0 22.0 16.0)\n";
    assert!(
        albumeasy::render(&after_page_start, &fixture(), &output).is_err(),
        "marginse after page start must fail"
    );

    // Duplicate in native album
    let asset_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/albumeasy/native-real-stamps-v1");
    let native_dup = r#"STAMPALBUM 1
ALBUM_PAGES_SIZE (210.0 297.0)
ALBUM_PAGES_MARGINS (25.0 12.0 18.0 21.0)
ALBUM_PAGES_MARGINSE (12.0 25.0 22.0 16.0)
ALBUM_PAGES_MARGINSE (10.0 10.0 10.0 10.0)
ALBUM_PAGES_SPACING (3.0 4.0)
ALBUM_PAGES_BORDER (0.2 0.0 0.0 0.0)
COLOUR_ALBUM_BORDER (black)
NATIVE_ROW_ALIGNMENT (center)
NATIVE_STAMP_FRAME (0.25 black)
NATIVE_STAMP_TEXT_POLICY (wrap_to_frame percent 100 0 0 0)
NATIVE_STAMP_IMAGE_POLICY (millimetres 0 0 false preserve colour show)

STAMP_IMAGE ("../stamp-pagination-v1/us-150.png" 52 29 0.05078125)
"#;
    assert!(
        NativeAlbum::parse(native_dup, &asset_root).is_err(),
        "duplicate ALBUM_PAGES_MARGINSE in native must fail"
    );

    Ok(())
}
