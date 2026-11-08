use oxidize_pdf::parser::PdfDocument;
use stamp_album_gen2::native_pdf::{album::NativeAlbum, albumeasy};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/albumeasy/stamp-settings-save-restore-v1")
}

fn contents(path: &Path) -> Result<Vec<String>, String> {
    let pdf = PdfDocument::open(path).map_err(|e| e.to_string())?;
    (0..pdf.page_count().map_err(|e| e.to_string())?)
        .map(|index| {
            let page = pdf.get_page(index).map_err(|e| e.to_string())?;
            assert!((page.media_box[2] - 612.0).abs() < 0.01);
            assert!((page.media_box[3] - 792.0).abs() < 0.01);
            Ok(pdf
                .get_page_content_streams(&page)
                .map_err(|e| e.to_string())?
                .iter()
                .map(|stream| String::from_utf8_lossy(stream))
                .collect::<Vec<_>>()
                .join("\n"))
        })
        .collect()
}

fn image_dimensions(content: &str) -> Result<Vec<(f64, f64)>, String> {
    let tokens: Vec<_> = content.split_whitespace().collect();
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
}

fn assert_restored_dimensions(dimensions: &[(f64, f64)]) {
    assert_eq!(dimensions.len(), 8);
    for (first, restored) in dimensions[..2].iter().zip(&dimensions[4..6]) {
        assert!((first.0 - restored.0).abs() < 0.01);
        assert!((first.1 - restored.1).abs() < 0.01);
    }
    assert!((dimensions[0].0 - dimensions[2].0).abs() > 1.0);
}

#[test]
fn txt_and_raw_restore_all_current_stamp_state_without_retroactive_changes() -> Result<(), String> {
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let mut all_contents = Vec::new();
    let fills =
        regex::Regex::new(r"([0-9.]+ [0-9.]+ [0-9.]+) rg\s+f\b").map_err(|e| e.to_string())?;
    let frames = regex::Regex::new(
        r"[0-9.-]+ [0-9.-]+ ([0-9.]+) ([0-9.]+) re\s+(?:0\.000 0\.000 0\.545|1\.000 0\.000 0\.000) RG\s+S\b",
    ).map_err(|e| e.to_string())?;
    for input in ["album.txt", "album-raw.stampalbum"] {
        let output = dir.path().join(format!("{input}.pdf"));
        assert_eq!(albumeasy::render_file(&fixture().join(input), &output)?, 2);
        let pages = contents(&output)?;
        assert_eq!(pages[0].matches(" Do").count(), 6);
        assert_eq!(pages[1].matches(" Do").count(), 2);
        let actual: Vec<_> = pages
            .iter()
            .flat_map(|page| fills.captures_iter(page))
            .map(|capture| capture[1].to_string())
            .collect();
        let blue = "0.678 0.847 0.902";
        let white = "1.000 1.000 1.000";
        assert_eq!(
            actual,
            [blue, blue, white, white, blue, blue, blue, blue, white, white, blue, blue]
        );
        assert_restored_dimensions(&image_dimensions(&pages.join("\n"))?);
        let frame_sizes = frames
            .captures_iter(&pages[0])
            .map(|c| {
                Ok((
                    c[1].parse::<f64>().map_err(|e| e.to_string())?,
                    c[2].parse::<f64>().map_err(|e| e.to_string())?,
                ))
            })
            .collect::<Result<Vec<_>, String>>()?;
        assert_eq!(frame_sizes.len(), 6);
        for (actual, expected) in frame_sizes.iter().zip([
            (58.0, 35.0),
            (32.0, 35.0),
            (60.0, 37.0),
            (34.0, 37.0),
            (58.0, 35.0),
            (32.0, 35.0),
        ]) {
            assert!((actual.0 - expected.0 * 72.0 / 25.4).abs() < 0.02);
            assert!((actual.1 - expected.1 * 72.0 / 25.4).abs() < 0.02);
        }
        assert!(pages[0].contains("1.000 0.000 0.000 RG"));
        assert!(pages[0].contains("0.000 0.000 0.545 RG"));
        assert!(pages[0].contains("1.000 0.000 0.000 rg"));
        assert!(pages[0].contains("0.000 0.000 0.545 rg"));
        all_contents.push(pages);
    }
    assert_eq!(all_contents[0], all_contents[1]);
    Ok(())
}

#[test]
fn native_snapshot_restores_mutable_image_background_and_outline_state() -> Result<(), String> {
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("native.pdf");
    let album = NativeAlbum::load(&fixture().join("album.stampalbum"))?;
    let pages = album.write_pdf(&output)?;
    assert_eq!(pages.len(), 2);
    let content = contents(&output)?.join("\n");
    assert_restored_dimensions(&image_dimensions(&content)?);
    assert_eq!(content.matches(" Do").count(), 8);
    let outlines = regex::Regex::new(r"\bre\s+[0-9.]+\s+[0-9.]+\s+[0-9.]+\s+RG\s+S\b")
        .map_err(|e| e.to_string())?;
    assert_eq!(
        outlines.find_iter(&content).count(),
        10,
        "Eight stamp outlines plus two page borders"
    );
    Ok(())
}

#[test]
fn lifecycle_errors_are_line_numbered_and_preserve_existing_outputs() -> Result<(), String> {
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("preserved.pdf");
    fs::write(&output, b"preserved").map_err(|e| e.to_string())?;
    let source = fs::read_to_string(fixture().join("album.txt")).map_err(|e| e.to_string())?;
    for input in [
        source.replace("ALBUM_STAMP_SETTINGS_SAVE", "ALBUM_STAMP_SETTINGS_RESTORE"),
        source.replace(
            "ALBUM_STAMP_SETTINGS_SAVE",
            "ALBUM_STAMP_SETTINGS_SAVE\nALBUM_STAMP_SETTINGS_SAVE",
        ),
        source.replace("ALBUM_STAMP_SETTINGS_SAVE", "ALBUM_STAMP_SETTINGS_SAVE (1)"),
        source.replace(
            "ALBUM_STAMP_SETTINGS_RESTORE",
            "ALBUM_STAMP_SETTINGS_RESTORE\nALBUM_STAMP_SETTINGS_RESTORE",
        ),
        source.replace(
            "ALBUM_STAMP_SETTINGS_RESTORE",
            "ALBUM_STAMP_SETTINGS_RESTORE (1)",
        ),
    ] {
        let error = albumeasy::render(&input, &fixture(), &output)
            .expect_err("Invalid lifecycle must fail");
        assert!(error.contains("Line ") && error.contains("ALBUM_STAMP_SETTINGS"));
        assert_eq!(fs::read(&output).map_err(|e| e.to_string())?, b"preserved");
    }
    let source =
        fs::read_to_string(fixture().join("album.stampalbum")).map_err(|e| e.to_string())?;
    for input in [
        source.replace(
            "NATIVE_STAMP_SETTINGS_SAVE ()",
            "NATIVE_STAMP_SETTINGS_RESTORE ()",
        ),
        source.replace(
            "NATIVE_STAMP_SETTINGS_SAVE ()",
            "NATIVE_STAMP_SETTINGS_SAVE ()\nNATIVE_STAMP_SETTINGS_SAVE ()",
        ),
        source.replace(
            "NATIVE_STAMP_SETTINGS_SAVE ()",
            "NATIVE_STAMP_SETTINGS_SAVE (1)",
        ),
        source.replace(
            "NATIVE_STAMP_SETTINGS_RESTORE ()",
            "NATIVE_STAMP_SETTINGS_RESTORE ()\nNATIVE_STAMP_SETTINGS_RESTORE ()",
        ),
        source.replace(
            "NATIVE_STAMP_SETTINGS_RESTORE ()",
            "NATIVE_STAMP_SETTINGS_RESTORE (1)",
        ),
    ] {
        let error = match NativeAlbum::parse(&input, &fixture()) {
            Err(error) => error,
            Ok(_) => return Err("Invalid native lifecycle accepted".into()),
        };
        assert!(error.contains("Line ") && error.contains("NATIVE_STAMP_SETTINGS"));
        assert_eq!(fs::read(&output).map_err(|e| e.to_string())?, b"preserved");
    }
    Ok(())
}

#[test]
fn heading_colour_is_restored_and_unsupported_saved_controls_remain_explicit() -> Result<(), String>
{
    let source = fs::read_to_string(fixture().join("album.txt")).map_err(|e| e.to_string())?;
    let source = source
        .split("\nPAGE_START")
        .take(2)
        .collect::<Vec<_>>()
        .join("\nPAGE_START")
        .replace(
            "COLOR_STAMP_TEXT (darkblue)",
            "COLOR_STAMP_TEXT (darkblue)\nCOLOR_STAMP_HEADING (darkblue)",
        )
        .replace(
            "COLOR_STAMP_TEXT (red)",
            "COLOR_STAMP_TEXT (red)\nCOLOR_STAMP_HEADING (red)",
        );
    let mut input = String::new();
    for line in source.lines() {
        input.push_str(line);
        input.push('\n');
        if line.starts_with("STAMP_ADD_IMG") {
            input.push_str("STAMP_HEADING_VA (HN 7 \"State heading\")\n");
        }
    }
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("headings.pdf");
    assert_eq!(albumeasy::render(&input, &fixture(), &output)?, 1);
    let content = contents(&output)?.join("\n");
    let pattern = regex::Regex::new(
        r"([0-9.]+ [0-9.]+ [0-9.]+) rg\s+[0-9.-]+ [0-9.-]+ Td\s+\(State heading\) Tj",
    )
    .map_err(|e| e.to_string())?;
    let colours: Vec<_> = pattern
        .captures_iter(&content)
        .map(|c| c[1].to_string())
        .collect();
    assert_eq!(
        colours,
        [
            "0.000 0.000 0.545",
            "0.000 0.000 0.545",
            "1.000 0.000 0.000",
            "1.000 0.000 0.000",
            "0.000 0.000 0.545",
            "0.000 0.000 0.545",
        ]
    );
    for control in [
        "STAMP_INNER_BORDER (1 0.1)",
        "STAMP_HEADING_PADDING (2)",
        "STAMP_FOOTER_TXT_PAD (2)",
    ] {
        let error = albumeasy::render(&format!("{source}\n{control}\n"), &fixture(), &output)
            .expect_err("Unmapped settings must not be silently accepted");
        assert!(
            error.contains("Unsupported") || error.contains("unsupported"),
            "{error}"
        );
    }
    Ok(())
}

#[test]
#[ignore = "Regenerates Denmark v4; earlier bundles remain immutable"]
fn regenerate_denmark_settings_proofs() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let previous = root
        .join("tests/fixtures/albumeasy/denmark-footer-native-v3/Denmark-1989-1989-supported.txt");
    let before = fs::read(&previous).map_err(|e| e.to_string())?;
    let source = std::str::from_utf8(&before)
        .map_err(|e| e.to_string())?
        .replace(
            "# NATIVE-TODO: ALBUM_STAMP_SETTINGS_SAVE",
            "ALBUM_STAMP_SETTINGS_SAVE",
        )
        .replace(
            "# NATIVE-TODO: ALBUM_STAMP_SETTINGS_RESTORE",
            "ALBUM_STAMP_SETTINGS_RESTORE",
        );
    assert_eq!(source.matches("# NATIVE-TODO:").count(), 1);
    let target = root.join("tests/fixtures/albumeasy/denmark-settings-native-v4");
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
        let output = target.join(output);
        assert_eq!(albumeasy::render_file(&target.join(input), &output)?, 16);
        let pdf = PdfDocument::open(&output).map_err(|e| e.to_string())?;
        assert_eq!(pdf.page_count().map_err(|e| e.to_string())?, 16);
        let mut images = 0;
        for index in 0..16 {
            let page = pdf.get_page(index).map_err(|e| e.to_string())?;
            let content = pdf
                .get_page_content_streams(&page)
                .map_err(|e| e.to_string())?
                .iter()
                .map(|stream| String::from_utf8_lossy(stream))
                .collect::<Vec<_>>()
                .join("\n");
            images += content.matches(" Do").count();
            assert!(content.contains(&format!("(Page {} of 16) Tj", index + 1)));
        }
        assert_eq!(images, 41);
        println!("{input}: 16 pages / 41 images / 1 retained deferral");
    }
    assert_eq!(fs::read(previous).map_err(|e| e.to_string())?, before);
    Ok(())
}
