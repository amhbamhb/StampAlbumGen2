use oxidize_pdf::parser::PdfDocument;
use stamp_album_gen2::native_pdf::album::NativeAlbum;
use std::{fs, path::Path};

fn fixture() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/native_pdf/stampalbum-v1/basic.stampalbum")
}

#[test]
fn real_stamp_album_file_renders_letter_pdf() -> Result<(), String> {
    let album_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/albumeasy/native-real-stamps-v1/album.stampalbum");
    let album = NativeAlbum::load(&album_path)?;
    let output_dir = tempfile::tempdir().map_err(|error| error.to_string())?;
    let output = output_dir.path().join("real-stamps.pdf");
    let pages = album.write_pdf(&output)?;

    assert_eq!(pages.len(), 1);
    let expected_sizes = [(52.0, 29.0), (26.0, 29.0), (26.0, 29.0), (52.0, 29.0)];
    assert_eq!(pages[0].items.len(), expected_sizes.len());
    for (item, (width, height)) in pages[0].items.iter().zip(expected_sizes) {
        let geometry = item
            .geometry
            .ok_or("real stamp fixture item is missing physical geometry")?;
        let bounds = geometry.physical_bounds();
        assert!(((bounds.right - bounds.left) - width).abs() < 0.001);
        assert!(((bounds.top - bounds.bottom) - height).abs() < 0.001);
    }

    let document = PdfDocument::open(&output).map_err(|error| error.to_string())?;
    assert_eq!(document.page_count().map_err(|error| error.to_string())?, 1);
    let page = document.get_page(0).map_err(|error| error.to_string())?;
    assert!((page.media_box[2] - 612.0).abs() < 0.01);
    assert!((page.media_box[3] - 792.0).abs() < 0.01);
    let streams = document
        .get_page_content_streams(&page)
        .map_err(|error| error.to_string())?;
    let content = streams
        .iter()
        .map(|stream| std::str::from_utf8(stream).map_err(|error| error.to_string()))
        .collect::<Result<Vec<_>, _>>()?
        .join("\n");
    assert_eq!(content.matches(" Do").count(), 4);
    Ok(())
}

#[test]
fn native_image_controls_keep_page_background_independent_of_hidden_stamps() -> Result<(), String> {
    let album_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/albumeasy/stamp-image-controls-v1/album.stampalbum");
    let source = fs::read_to_string(&album_path).map_err(|error| error.to_string())?;
    let source = source.replace(
        "NATIVE_PAGE_TITLE",
        "NATIVE_PAGE_BACKGROUND_IMAGE (\"../stamp-pagination-v1/us-150.png\" 15 1 26 14.5)\nNATIVE_PAGE_TITLE",
    );
    let album = NativeAlbum::parse(
        &source,
        album_path
            .parent()
            .ok_or("fixture has no parent directory")?,
    )?;
    let output_dir = tempfile::tempdir().map_err(|error| error.to_string())?;
    let output = output_dir.path().join("image-controls.pdf");
    let pages = album.write_pdf(&output)?;

    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].items.len(), 7);
    let expected_sizes = [
        (52.0, 29.0),
        (26.0, 29.0),
        (26.0, 29.0),
        (26.0, 29.0),
        (52.0, 29.0),
        (26.0, 29.0),
        (26.0, 29.0),
    ];
    for (item, (width, height)) in pages[0].items.iter().zip(expected_sizes) {
        let geometry = item
            .geometry
            .ok_or("image-control fixture item is missing physical geometry")?;
        let bounds = geometry.physical_bounds();
        assert!(((bounds.right - bounds.left) - width).abs() < 0.001);
        assert!(((bounds.top - bounds.bottom) - height).abs() < 0.001);
        let frame = geometry.frame_bounds();
        assert!(((frame.right - frame.left) - width - 6.0).abs() < 0.001);
        assert!(((frame.top - frame.bottom) - height - 6.0).abs() < 0.001);
    }
    let frame_tops = pages[0]
        .items
        .iter()
        .map(|item| {
            item.geometry
                .ok_or("image-control fixture item is missing physical geometry")
                .map(|geometry| geometry.frame_bounds().top)
        })
        .collect::<Result<Vec<_>, _>>()?;
    for (start, end) in [(0, 2), (2, 4), (4, 7)] {
        assert!(frame_tops[start..end]
            .iter()
            .all(|top| (top - frame_tops[start]).abs() < 0.001));
    }
    assert!(frame_tops[0] > frame_tops[2]);
    assert!(frame_tops[2] > frame_tops[4]);
    let document = PdfDocument::open(&output).map_err(|error| error.to_string())?;
    assert_eq!(document.page_count().map_err(|error| error.to_string())?, 1);
    let page = document.get_page(0).map_err(|error| error.to_string())?;
    let streams = document
        .get_page_content_streams(&page)
        .map_err(|error| error.to_string())?;
    let content = streams
        .iter()
        .map(|stream| std::str::from_utf8(stream).map_err(|error| error.to_string()))
        .collect::<Result<Vec<_>, _>>()?
        .join("\n");
    assert_eq!(content.matches(" Do").count(), 7);
    assert!(content.contains("native_page_background_0 Do"));
    Ok(())
}

#[test]
fn native_stamp_images_without_policy_keep_version_one_defaults() -> Result<(), String> {
    let album_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/albumeasy/stamp-image-controls-v1/album.stampalbum");
    let source = fs::read_to_string(&album_path).map_err(|error| error.to_string())?;
    let source = source
        .lines()
        .filter(|line| !line.trim_start().starts_with("NATIVE_STAMP_IMAGE_POLICY"))
        .collect::<Vec<_>>()
        .join("\n");
    let album = NativeAlbum::parse(
        &source,
        album_path
            .parent()
            .ok_or("fixture has no parent directory")?,
    )?;
    let output_dir = tempfile::tempdir().map_err(|error| error.to_string())?;
    let output = output_dir.path().join("default-image-policy.pdf");
    let pages = album.write_pdf(&output)?;

    assert_eq!(pages.len(), 1);
    let document = PdfDocument::open(&output).map_err(|error| error.to_string())?;
    let page = document.get_page(0).map_err(|error| error.to_string())?;
    let streams = document
        .get_page_content_streams(&page)
        .map_err(|error| error.to_string())?;
    let content = streams
        .iter()
        .map(|stream| std::str::from_utf8(stream).map_err(|error| error.to_string()))
        .collect::<Result<Vec<_>, _>>()?
        .join("\n");
    assert_eq!(content.matches(" Do").count(), 7);
    Ok(())
}

#[test]
fn native_file_writes_a_pdf_without_diagnostic_geometry_overlays() -> Result<(), String> {
    let album = NativeAlbum::load(&fixture())?;
    let output_dir = tempfile::tempdir().map_err(|error| error.to_string())?;
    let output = output_dir.path().join("native.pdf");
    let pages = album.write_pdf(&output)?;

    assert_eq!(pages.len(), 1);
    let metadata = fs::metadata(&output).map_err(|error| error.to_string())?;
    assert!(metadata.len() > 0);

    let document = PdfDocument::open(&output).map_err(|error| error.to_string())?;
    assert_eq!(document.page_count().map_err(|error| error.to_string())?, 1);
    let page = document.get_page(0).map_err(|error| error.to_string())?;
    assert!((page.media_box[2] - 612.0).abs() < 0.01);
    assert!((page.media_box[3] - 792.0).abs() < 0.01);
    let streams = document
        .get_page_content_streams(&page)
        .map_err(|error| error.to_string())?;
    let content = streams
        .iter()
        .map(|stream| std::str::from_utf8(stream).map_err(|error| error.to_string()))
        .collect::<Result<Vec<_>, _>>()?
        .join("\n");
    assert!(content.contains(" Do"));
    assert!(content.contains(" re"));
    assert!(content.contains("0.000 0.000 0.000 RG"));
    assert!(!content.contains("0.000 0.300 0.900 RG"));
    assert!(!content.contains("0.000 0.600 0.200 RG"));
    Ok(())
}

#[test]
fn unknown_commands_and_missing_required_settings_are_errors() {
    let source = "STAMPALBUM 1\nFUTURE_COMMAND (value)\n";
    let error = NativeAlbum::parse(source, Path::new(".")).unwrap_err();
    assert!(error.contains("Line 2: Unsupported native command FUTURE_COMMAND"));

    let source = "STAMPALBUM 1\nALBUM_PAGES_SIZE (215.9 279.4)\n";
    let error = NativeAlbum::parse(source, Path::new(".")).unwrap_err();
    assert!(error.contains("Missing required ALBUM_PAGES_MARGINS command"));
}

#[test]
fn unsupported_versions_are_rejected() {
    let error = NativeAlbum::parse("STAMPALBUM 2\n", Path::new(".")).unwrap_err();
    assert!(error.contains("expected the version header `STAMPALBUM 1`"));
}

#[test]
fn invalid_asset_preserves_existing_pdf() -> Result<(), String> {
    let path = fixture();
    let source = fs::read_to_string(&path).map_err(|error| error.to_string())?;
    let source = source.replace("synthetic-asymmetric.png", "missing-image.png");
    let album = NativeAlbum::parse(&source, path.parent().unwrap_or_else(|| Path::new(".")))?;
    let output_dir = tempfile::tempdir().map_err(|error| error.to_string())?;
    let output = output_dir.path().join("existing.pdf");
    fs::write(&output, b"preserve existing output").map_err(|error| error.to_string())?;

    let error = album.write_pdf(&output).unwrap_err();
    assert!(error.contains("Cannot load native stamp image"));
    let contents = fs::read(&output).map_err(|error| error.to_string())?;
    assert_eq!(contents, b"preserve existing output");
    Ok(())
}

#[test]
fn native_files_require_the_stampalbum_extension() {
    let mut path = fixture();
    path.set_extension("txt");
    let error = NativeAlbum::load(&path).unwrap_err();
    assert!(error.contains("must use the .stampalbum extension"));
}
