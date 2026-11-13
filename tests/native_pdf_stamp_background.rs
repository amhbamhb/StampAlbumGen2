use oxidize_pdf::parser::PdfDocument;
use stamp_album_gen2::native_pdf::{album::NativeAlbum, albumeasy};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/albumeasy/stamp-background-colours-v1")
}

fn content(path: &Path) -> Result<String, String> {
    let pdf = PdfDocument::open(path).map_err(|e| e.to_string())?;
    assert_eq!(pdf.page_count().map_err(|e| e.to_string())?, 1);
    let page = pdf.get_page(0).map_err(|e| e.to_string())?;
    assert!((page.media_box[2] - 612.0).abs() < 0.01);
    assert!((page.media_box[3] - 792.0).abs() < 0.01);
    Ok(pdf
        .get_page_content_streams(&page)
        .map_err(|e| e.to_string())?
        .iter()
        .map(|s| String::from_utf8_lossy(s))
        .collect::<Vec<_>>()
        .join("\n"))
}

fn check_backgrounds(content: &str) -> Result<(), String> {
    assert_eq!(content.matches(" Do").count(), 7);
    let fills = regex::Regex::new(r"\bre\s+[0-9.]+\s+[0-9.]+\s+[0-9.]+\s+rg\s+f\b")
        .map_err(|e| e.to_string())?;
    assert_eq!(fills.find_iter(content).count(), 6);
    let outlines = regex::Regex::new(r"\bre\s+[0-9.]+\s+[0-9.]+\s+[0-9.]+\s+RG\s+S\b")
        .map_err(|e| e.to_string())?;
    assert_eq!(
        outlines.find_iter(content).count(),
        7,
        "six outlines plus page border"
    );
    let colours = regex::Regex::new(r"\bre\s+([0-9.]+\s+[0-9.]+\s+[0-9.]+)\s+rg\s+f\b")
        .map_err(|e| e.to_string())?;
    let actual: Vec<_> = colours
        .captures_iter(content)
        .map(|c| c[1].to_string())
        .collect();
    assert_eq!(
        actual,
        [
            "0.000 0.000 0.000",
            "0.000 0.000 0.000",
            "0.678 0.847 0.902",
            "0.678 0.847 0.902",
            "0.000 0.000 0.000",
            "1.000 1.000 1.000",
        ]
    );
    // All backgrounds precede image painting, including the hidden-image box.
    let last_fill = fills
        .find_iter(content)
        .last()
        .ok_or("Missing backgrounds")?
        .end();
    let first_image = content.find(" Do").ok_or("Missing images")?;
    assert!(last_fill < first_image);
    Ok(())
}

#[test]
fn txt_and_raw_stampalbum_preserve_background_state_and_layering() -> Result<(), String> {
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let txt = dir.path().join("txt.pdf");
    let raw = dir.path().join("raw.pdf");
    assert_eq!(
        albumeasy::render_file(&fixture().join("album.txt"), &txt)?,
        1
    );
    assert_eq!(
        albumeasy::render_file(&fixture().join("album-raw.stampalbum"), &raw)?,
        1
    );
    let text = content(&txt)?;
    check_backgrounds(&text)?;
    assert_eq!(text, content(&raw)?);
    Ok(())
}

#[test]
fn native_background_and_outline_are_independent_of_image_visibility() -> Result<(), String> {
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("native.pdf");
    let album = NativeAlbum::load(&fixture().join("album.stampalbum"))?;
    assert_eq!(album.write_pdf(&output)?.len(), 1);
    let native_content = content(&output)?;
    check_backgrounds(&native_content)?;
    let txt_output = dir.path().join("txt.pdf");
    albumeasy::render_file(&fixture().join("album.txt"), &txt_output)?;
    let dimensions = |source: &str| -> Result<Vec<(f64, f64)>, String> {
        let tokens: Vec<_> = source.split_whitespace().collect();
        tokens
            .windows(9)
            .filter(|w| w[6] == "cm" && w[8] == "Do")
            .map(|w| {
                Ok((
                    w[0].parse::<f64>().map_err(|e| e.to_string())?,
                    w[3].parse::<f64>().map_err(|e| e.to_string())?,
                ))
            })
            .collect()
    };
    let native_dimensions = dimensions(&native_content)?;
    let txt_dimensions = dimensions(&content(&txt_output)?)?;
    assert_eq!(native_dimensions.len(), 7);
    assert_eq!(txt_dimensions.len(), 7);
    for (native, txt) in native_dimensions.iter().zip(txt_dimensions) {
        assert!((native.0 - txt.0).abs() <= 0.02);
        assert!((native.1 - txt.1).abs() <= 0.02);
    }
    Ok(())
}

#[test]
fn invalid_background_colours_preserve_existing_output() -> Result<(), String> {
    let source = fs::read_to_string(fixture().join("album.txt")).map_err(|e| e.to_string())?;
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("preserved.pdf");
    fs::write(&output, b"preserve").map_err(|e| e.to_string())?;
    for value in ["not-a-colour", "\"black\"", "black white", ""] {
        let invalid = source.replace(
            "COLOR_STAMP_BACKGROUND (black)",
            &format!("COLOR_STAMP_BACKGROUND ({value})"),
        );
        assert!(albumeasy::render(&invalid, &fixture(), &output).is_err());
        assert_eq!(fs::read(&output).map_err(|e| e.to_string())?, b"preserve");
    }
    let native =
        fs::read_to_string(fixture().join("album.stampalbum")).map_err(|e| e.to_string())?;
    for input in [
        native.replace(
            "NATIVE_STAMP_BACKGROUND (black)",
            "NATIVE_STAMP_BACKGROUND (invalid)",
        ),
        native.replace(
            "NATIVE_STAMP_OUTLINE (hide)",
            "NATIVE_STAMP_OUTLINE (invalid)",
        ),
    ] {
        assert!(NativeAlbum::parse(&input, &fixture()).is_err());
    }
    Ok(())
}

#[test]
#[ignore = "Regenerates versioned Denmark background proofs without changing v1 references"]
fn regenerate_denmark_background_proofs() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let previous =
        root.join("tests/fixtures/albumeasy/denmark-txt-native-v1/Denmark-1989-1989-supported.txt");
    let original = fs::read(&previous).map_err(|e| e.to_string())?;
    let source = std::str::from_utf8(&original)
        .map_err(|e| e.to_string())?
        .replace(
            "# NATIVE-TODO: COLOR_STAMP_BACKGROUND",
            "COLOR_STAMP_BACKGROUND",
        )
        .replace("\"images/", "\"../denmark-txt-native-v1/images/");
    assert_eq!(source.matches("# NATIVE-TODO:").count(), 5);
    let target = root.join("tests/fixtures/albumeasy/denmark-background-native-v2");
    fs::create_dir_all(&target).map_err(|e| e.to_string())?;
    for (input, output) in [
        (
            "Denmark-1989-1989-supported.txt",
            "Denmark-1989-1989-native.pdf",
        ),
        (
            "Denmark-1989-1989-raw.stampalbum",
            "Denmark-1989-1989-raw-native.pdf",
        ),
    ] {
        fs::write(target.join(input), &source).map_err(|e| e.to_string())?;
        let path = target.join(output);
        assert_eq!(albumeasy::render_file(&target.join(input), &path)?, 16);
        let pdf = PdfDocument::open(&path).map_err(|e| e.to_string())?;
        assert_eq!(pdf.page_count().map_err(|e| e.to_string())?, 16);
        let mut images = 0;
        for index in 0..16 {
            let page = pdf.get_page(index).map_err(|e| e.to_string())?;
            for stream in pdf
                .get_page_content_streams(&page)
                .map_err(|e| e.to_string())?
            {
                images += String::from_utf8_lossy(&stream).matches(" Do").count();
            }
        }
        assert_eq!(images, 41);
        println!(
            "{input}: 16 pages, 41 images; 3 background commands restored, 5 deferrals retained"
        );
    }
    assert_eq!(fs::read(previous).map_err(|e| e.to_string())?, original);
    Ok(())
}
