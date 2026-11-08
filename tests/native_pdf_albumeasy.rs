use oxidize_pdf::parser::PdfDocument;
use stamp_album_gen2::native_pdf::albumeasy;
use std::{fs, path::Path};

fn fixture() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/stamp-image-controls-v1")
}

fn check_pdf(path: &Path, pages: u32, images: usize) -> Result<(), String> {
    let pdf = PdfDocument::open(path).map_err(|e| e.to_string())?;
    assert_eq!(pdf.page_count().map_err(|e| e.to_string())?, pages);
    let page = pdf.get_page(0).map_err(|e| e.to_string())?;
    assert!((page.media_box[2] - 612.0).abs() < 0.01);
    assert!((page.media_box[3] - 792.0).abs() < 0.01);
    let streams = pdf
        .get_page_content_streams(&page)
        .map_err(|e| e.to_string())?;
    let content = streams
        .iter()
        .map(|s| String::from_utf8_lossy(s))
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(content.matches(" Do").count(), images);
    assert!(content.contains("PERCENT PADDING"));
    Ok(())
}

#[test]
fn original_albumeasy_txt_renders_natively() -> Result<(), String> {
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("txt-native.pdf");
    assert_eq!(
        albumeasy::render_file(&fixture().join("album.txt"), &output)?,
        1
    );
    check_pdf(&output, 1, 6)
}

#[test]
fn raw_albumeasy_stampalbum_uses_the_same_rendering_path() -> Result<(), String> {
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let source = fs::read_to_string(fixture().join("album.txt")).map_err(|e| e.to_string())?;
    let path = dir.path().join("raw.stampalbum");
    // Absolute fixture paths are used only in this temporary cross-platform test.
    let source = source
        .replace(
            "../stamp-pagination-v1/",
            &format!(
                "{}/",
                fixture()
                    .parent()
                    .ok_or("Missing fixture parent")?
                    .join("stamp-pagination-v1")
                    .to_string_lossy()
                    .replace('\\', "/")
            ),
        )
        .replace(
            "\"us-152-small.png\"",
            &format!(
                "\"{}\"",
                fixture()
                    .join("us-152-small.png")
                    .to_string_lossy()
                    .replace('\\', "/")
            ),
        );
    fs::write(&path, source).map_err(|e| e.to_string())?;
    let output = dir.path().join("raw.pdf");
    assert_eq!(albumeasy::render_file(&path, &output)?, 1);
    check_pdf(&output, 1, 6)
}

#[test]
fn unknown_commands_require_an_explicit_commented_test_copy() -> Result<(), String> {
    let source = fs::read_to_string(fixture().join("album.txt")).map_err(|e| e.to_string())?;
    let source = format!("{source}\nSTAMP_BOXES_SIZE_ADJUST (0) \nSTAMP_INNER_BORDER (1 0.1)\n");
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("preserved.pdf");
    fs::write(&output, b"existing output").map_err(|e| e.to_string())?;
    let error = albumeasy::render(&source, &fixture(), &output).expect_err("unsupported command");
    assert!(error.contains("Unsupported AlbumEasy command STAMP_INNER_BORDER"));
    assert_eq!(
        fs::read(&output).map_err(|e| e.to_string())?,
        b"existing output"
    );
    let (prepared, losses) = albumeasy::commented_test_source(&source)?;
    assert_eq!(losses.len(), 1);
    assert_eq!(losses[0].name, "STAMP_INNER_BORDER");
    assert!(prepared.contains("# NATIVE-TODO: STAMP_INNER_BORDER"));
    assert!(prepared.contains("STAMP_BOXES_SIZE_ADJUST (0)\n"));
    assert_eq!(albumeasy::render(&prepared, &fixture(), &output)?, 1);
    Ok(())
}

#[test]
fn explicit_page_starts_and_missing_images_are_not_ignored() -> Result<(), String> {
    let source = fs::read_to_string(fixture().join("album.txt")).map_err(|e| e.to_string())?;
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("pages.pdf");
    let doubled = format!("{source}\nPAGE_START\nPAGE_TEXT_CENTRE (HN 9 \"Second page\" 5)\n");
    assert_eq!(albumeasy::render(&doubled, &fixture(), &output)?, 2);
    let invalid = source.replace("us-152-small.png", "missing.png");
    let before = fs::read(&output).map_err(|e| e.to_string())?;
    assert!(albumeasy::render(&invalid, &fixture(), &output).is_err());
    assert_eq!(fs::read(&output).map_err(|e| e.to_string())?, before);
    Ok(())
}

#[test]
fn mixed_height_row_uses_each_stamps_own_caption_footprint() -> Result<(), String> {
    let source = format!(
        "ALBUM_PAGES_SIZE (215.9 279.4)\n\
         ALBUM_PAGES_MARGINS (10 10 10 10)\n\
         ALBUM_PAGES_SPACING (4 4)\n\
         ALBUM_PAGES_BORDER (0.2 0 0)\n\
         PAGE_START\nPAGE_VSPACE (50)\nROW_START_FS (HN 10 0.2 4)\n\
         STAMP_ADD_IMG (60 200 \"us-152-small.png\" \"\" \"\" \"\")\n\
         STAMP_ADD_IMG (30 20 \"us-152-small.png\" \"\" \"{}\" \"\")\n",
        ["Short stamp"; 12].join("\\n")
    );
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("mixed.pdf");
    assert_eq!(albumeasy::render(&source, &fixture(), &output)?, 1);
    let pdf = PdfDocument::open(&output).map_err(|e| e.to_string())?;
    assert_eq!(pdf.page_count().map_err(|e| e.to_string())?, 1);
    Ok(())
}

#[test]
fn invalid_arguments_are_errors_not_silently_commented_commands() -> Result<(), String> {
    let source = fs::read_to_string(fixture().join("album.txt")).map_err(|e| e.to_string())?;
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("invalid.pdf");
    for extra in [
        "STAMP_BOXES_SIZE_ADJUST (NaN)",
        "ROW_START_FS (HN 8 -1 4)",
        "STAMP_BORDER_STYLE (DASH)",
        "ALBUM_PAGES_SIZE (215.9 279.4)",
    ] {
        let input = format!("{source}\n{extra}\n");
        let (_, losses) = albumeasy::commented_test_source(&input)?;
        assert!(losses.is_empty());
        let error = albumeasy::render(&input, &fixture(), &output).expect_err(extra);
        assert!(error.contains("Line "));
        assert!(!output.exists());
    }
    assert!(albumeasy::commented_test_source("STAMP_ADD_IMG (\"unterminated)\n").is_err());
    Ok(())
}

#[test]
#[ignore = "Opt-in Denmark 1989 source inspection and native PDF; requires the local album"]
fn render_denmark_commented_test_copy() -> Result<(), String> {
    let input =
        Path::new("/media/ray/program/Rust/StampAlbumGen2/Albums/Denmark/Denmark-1989-1989.txt");
    let original = fs::read(input).map_err(|e| e.to_string())?;
    let source = std::str::from_utf8(&original).map_err(|e| e.to_string())?;
    let (prepared, losses) = albumeasy::commented_test_source(source)?;
    for loss in &losses {
        println!(
            "Deferred line {}: {}: {}",
            loss.line, loss.name, loss.source
        );
    }
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/albumeasy/denmark-txt-native-v1");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let image_pattern =
        regex::Regex::new(r#"^STAMP_ADD_IMG\s*\([^"]*"([^"]+)""#).map_err(|e| e.to_string())?;
    let mut images = std::collections::BTreeSet::new();
    for line in prepared.lines() {
        if let Some(captures) = image_pattern.captures(line.trim()) {
            let image = captures.get(1).ok_or("Missing image capture")?.as_str();
            let relative = Path::new(image);
            if relative
                .components()
                .any(|part| !matches!(part, std::path::Component::Normal(_)))
            {
                return Err(format!(
                    "Denmark fixture image must be a portable relative path: {image}"
                ));
            }
            images.insert(image.to_string());
        }
    }
    for image in &images {
        let destination = dir.join(image);
        fs::create_dir_all(destination.parent().ok_or("Missing image directory")?)
            .map_err(|e| e.to_string())?;
        fs::copy(
            input.parent().ok_or("Missing asset root")?.join(image),
            destination,
        )
        .map_err(|e| e.to_string())?;
    }
    fs::write(dir.join("Denmark-1989-1989-supported.txt"), &prepared).map_err(|e| e.to_string())?;
    fs::write(dir.join("Denmark-1989-1989-raw.stampalbum"), &prepared)
        .map_err(|e| e.to_string())?;
    let output = dir.join("Denmark-1989-1989-native.pdf");
    let count = albumeasy::render(&prepared, &dir, &output)?;
    println!("Rendered {count} pages to {}", output.display());
    println!(
        "Copied {} unique images; {} commands explicitly deferred",
        images.len(),
        losses.len()
    );
    assert_eq!(count, 16);
    let pdf = PdfDocument::open(&output).map_err(|e| e.to_string())?;
    let mut image_count = 0;
    for index in 0..16 {
        let page = pdf.get_page(index).map_err(|e| e.to_string())?;
        for stream in pdf
            .get_page_content_streams(&page)
            .map_err(|e| e.to_string())?
        {
            image_count += String::from_utf8_lossy(&stream).matches(" Do").count();
        }
    }
    assert_eq!(image_count, 41);
    assert_eq!(fs::read(input).map_err(|e| e.to_string())?, original);
    Ok(())
}
