use stamp_album_gen2::native_pdf::{
    album::NativeAlbum, albumeasy, colour::Color,
    commands::colour::colour_album_border::AlbumBorderColour,
};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/color-formats-v1")
}

#[test]
fn color_parser_accepts_all_common_formats() -> Result<()> {
    // 1. 6-digit hex (#RRGGBB)
    let c1 = Color::parse("#1E4A7A")?;
    assert_eq!(c1.rgb_u8(), [30, 74, 122]);
    assert_eq!(c1.to_hex(), "#1E4A7A");

    // Case-insensitivity
    let c1_lower = Color::parse("#1e4a7a")?;
    assert_eq!(c1, c1_lower);

    // Quoted hex
    let c1_quoted = Color::parse("\"#1E4A7A\"")?;
    assert_eq!(c1, c1_quoted);

    // 2. 3-digit shorthand hex (#RGB -> #RRGGBB)
    let c2 = Color::parse("#F80")?;
    assert_eq!(c2.rgb_u8(), [255, 136, 0]);
    assert_eq!(c2.to_hex(), "#FF8800");

    let c3 = Color::parse("#036")?;
    assert_eq!(c3.rgb_u8(), [0, 51, 102]);
    assert_eq!(c3.to_hex(), "#003366");

    // 3. Bare hex without #
    let c4 = Color::parse("1E4A7A")?;
    assert_eq!(c4, c1);

    let c5 = Color::parse("F80")?;
    assert_eq!(c5, c2);

    let c6 = Color::parse("2E8B57")?;
    assert_eq!(c6.rgb_u8(), [46, 139, 87]);
    assert_eq!(c6.to_hex(), "#2E8B57");

    // 4. Functional rgb(r, g, b) with 0-255 integers
    let c7 = Color::parse("rgb(30, 74, 122)")?;
    assert_eq!(c7.rgb_u8(), [30, 74, 122]);

    let c8 = Color::parse("\"rgb(34, 139, 34)\"")?;
    assert_eq!(c8.rgb_u8(), [34, 139, 34]);

    // 5. Functional rgb(r%, g%, b%) with percentages
    let c9 = Color::parse("rgb(100%, 0%, 0%)")?;
    assert_eq!(c9.rgb_u8(), [255, 0, 0]);

    // 6. SVG / CSS named colors
    let black = Color::parse("black")?;
    assert_eq!(black.rgb_u8(), [0, 0, 0]);

    let white = Color::parse("white")?;
    assert_eq!(white.rgb_u8(), [255, 255, 255]);

    let navy = Color::parse("NAVY")?;
    assert_eq!(navy.rgb_u8(), [0, 0, 128]);

    let forest = Color::parse("ForestGreen")?;
    assert_eq!(forest.rgb_u8(), [34, 139, 34]);

    // 7. AlbumBorderColour wrapping Color
    let abc = AlbumBorderColour::parse(&["#1E4A7A"])?;
    assert_eq!(abc.color(), c1);
    assert_eq!(abc.rgb(), c1.rgb());

    let abc_bare = AlbumBorderColour::parse(&["1E4A7A"])?;
    assert_eq!(abc_bare.color(), c1);

    let abc_rgb = AlbumBorderColour::parse(&["rgb(30, 74, 122)"])?;
    assert_eq!(abc_rgb.color(), c1);

    Ok(())
}

#[test]
fn native_commands_accept_optional_color_parameter() -> Result<()> {
    let dir = fixture_dir();
    let tmp = std::env::temp_dir();

    // 1. NATIVE_PAGE_TITLE with 4 args (default black) and 5 args (custom color)
    let src_title_default = r#"STAMPALBUM 1
ALBUM_PAGES_SIZE (215.9 279.4)
ALBUM_PAGES_MARGINS (15 15 15 15)
ALBUM_PAGES_SPACING (4 10)
COLOR_ALBUM_BORDER (black)
NATIVE_PAGE_TITLE (TB 15 "DEFAULT TITLE" 8)
NATIVE_ROW_ALIGNMENT (left)
NATIVE_STAMP_FRAME (0.6 black 2)
NATIVE_STAMP_TEXT_POLICY (expand_footprint percent 140 2.0 1.0 2.5)
STAMP_IMAGE ("../spacing-v1/images/san-marino.jpg" 35 26 0.175 "HN" 8 "San Marino")
"#;
    let album_title_default = NativeAlbum::parse(src_title_default, &dir)?;
    let p1 = album_title_default.write_pdf(&tmp.join("test_title_default.pdf"))?;
    assert_eq!(p1.len(), 1);

    let src_title_colored = r#"STAMPALBUM 1
ALBUM_PAGES_SIZE (215.9 279.4)
ALBUM_PAGES_MARGINS (15 15 15 15)
ALBUM_PAGES_SPACING (4 10)
COLOR_ALBUM_BORDER (#1E4A7A)
NATIVE_PAGE_TITLE (TB 15 "COLORED TITLE" 8 #1E4A7A)
NATIVE_ROW_ALIGNMENT (left)
NATIVE_STAMP_FRAME (0.6 #2B5B84 2)
NATIVE_STAMP_TEXT_POLICY (expand_footprint percent 140 2.0 1.0 2.5)
STAMP_IMAGE ("../spacing-v1/images/san-marino.jpg" 35 26 0.175 "HN" 8 "San Marino")
"#;
    let album_title_colored = NativeAlbum::parse(src_title_colored, &dir)?;
    let p2 = album_title_colored.write_pdf(&tmp.join("test_title_colored.pdf"))?;
    assert_eq!(p2.len(), 1);

    // 2. NATIVE_PAGE_HEADER with 6 args (default black) and 7 args (custom color)
    let src_header_colored = r#"STAMPALBUM 1
ALBUM_PAGES_SIZE (215.9 279.4)
ALBUM_PAGES_MARGINS (15 15 15 15)
ALBUM_PAGES_SPACING (4 10)
COLOR_ALBUM_BORDER (black)
NATIVE_PAGE_HEADER (HN 9 C "RUNNING HEADER" 0 0 "rgb(178, 34, 34)")
NATIVE_ROW_ALIGNMENT (left)
NATIVE_STAMP_FRAME (0.6 black 2)
NATIVE_STAMP_TEXT_POLICY (expand_footprint percent 140 2.0 1.0 2.5)
STAMP_IMAGE ("../spacing-v1/images/san-marino.jpg" 35 26 0.175 "HN" 8 "San Marino")
"#;
    let album_header_colored = NativeAlbum::parse(src_header_colored, &dir)?;
    let p3 = album_header_colored.write_pdf(&tmp.join("test_header_colored.pdf"))?;
    assert_eq!(p3.len(), 1);

    Ok(())
}

#[test]
fn color_parser_rejects_invalid_values_and_transparency() {
    assert!(Color::parse("").is_err());
    assert!(Color::parse("   ").is_err());
    assert!(Color::parse("none").is_err());
    assert!(Color::parse("transparent").is_err());
    assert!(Color::parse("currentcolor").is_err());
    assert!(Color::parse("not-a-color-at-all").is_err());

    // Invalid hex
    assert!(Color::parse("#GG00FF").is_err());
    assert!(Color::parse("#12").is_err());
    assert!(Color::parse("#12345").is_err());
    assert!(Color::parse("#1234567").is_err());

    // Non-opaque transparency
    assert!(Color::parse("rgba(0, 0, 0, 0.5)").is_err());
    assert!(Color::parse("#1E4A7A80").is_err());
    assert!(Color::parse("#F808").is_err());
}

#[test]
fn render_all_color_formats_fixtures_and_generate_native_pdfs() -> Result<()> {
    let dir = fixture_dir();

    let variations = [
        "hex-6digit",
        "hex-3digit",
        "bare-hex",
        "rgb-integer",
        "rgb-percent",
        "svg-named",
    ];

    for var in variations {
        // 1. Render native format (*.stampalbum -> *-native.pdf)
        let native_source_path = dir.join(format!("{var}.stampalbum"));
        let native_pdf_path = dir.join(format!("{var}-native.pdf"));
        let native_source = fs::read_to_string(&native_source_path)?;
        let native_album = NativeAlbum::parse(&native_source, &dir)?;
        let pages = native_album.write_pdf(&native_pdf_path)?;
        assert_eq!(pages.len(), 1, "Variation {var} should render 1 page");
        assert!(native_pdf_path.exists());
        assert!(fs::metadata(&native_pdf_path)?.len() > 1000);

        // 2. Render raw format (*-raw.stampalbum -> *-raw-native.pdf)
        let raw_source_path = dir.join(format!("{var}-raw.stampalbum"));
        let raw_pdf_path = dir.join(format!("{var}-raw-native.pdf"));
        let raw_pages = albumeasy::render_file(&raw_source_path, &raw_pdf_path)?;
        assert_eq!(raw_pages, 1, "Raw variation {var} should render 1 page");
        assert!(raw_pdf_path.exists());
        assert!(fs::metadata(&raw_pdf_path)?.len() > 1000);
    }

    // 3. Render comprehensive 6-page all-in-one suite
    let all_in_one_path = dir.join("all-in-one-compare.stampalbum");
    let all_in_one_pdf = dir.join("all-in-one-compare-native.pdf");
    let all_in_one_source = fs::read_to_string(&all_in_one_path)?;
    let all_in_one_album = NativeAlbum::parse(&all_in_one_source, &dir)?;
    let aio_pages = all_in_one_album.write_pdf(&all_in_one_pdf)?;
    assert_eq!(
        aio_pages.len(),
        6,
        "all-in-one-compare must contain exactly 6 pages"
    );
    assert!(all_in_one_pdf.exists());
    assert!(fs::metadata(&all_in_one_pdf)?.len() > 1000);

    // 4. Render raw 6-page all-in-one suite
    let raw_aio_path = dir.join("all-in-one-compare-raw.stampalbum");
    let raw_aio_pdf = dir.join("all-in-one-compare-raw-native.pdf");
    let raw_aio_pages = albumeasy::render_file(&raw_aio_path, &raw_aio_pdf)?;
    assert_eq!(
        raw_aio_pages, 6,
        "all-in-one-compare-raw must contain exactly 6 pages"
    );
    assert!(raw_aio_pdf.exists());
    assert!(fs::metadata(&raw_aio_pdf)?.len() > 1000);

    Ok(())
}
