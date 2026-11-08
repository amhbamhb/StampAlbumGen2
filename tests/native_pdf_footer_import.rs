use oxidize_pdf::parser::PdfDocument;
use stamp_album_gen2::native_pdf::{album::NativeAlbum, albumeasy};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/albumeasy/footer-numbering-import-v1")
}

fn check(path: &Path, start: i32, decorated: bool) -> Result<Vec<String>, String> {
    let pdf = PdfDocument::open(path).map_err(|e| e.to_string())?;
    assert_eq!(pdf.page_count().map_err(|e| e.to_string())?, 2);
    let mut contents = Vec::new();
    let mut images = 0;
    let pattern =
        regex::Regex::new(r"(?s)0\.000 0\.000 0\.545 rg\s+([0-9.-]+) ([0-9.-]+) Td\s+\([^)]*\) Tj")
            .map_err(|e| e.to_string())?;
    let outlines = regex::Regex::new(r"\bre\s+[0-9.]+\s+[0-9.]+\s+[0-9.]+\s+RG\s+S\b")
        .map_err(|e| e.to_string())?;
    for index in 0..2 {
        let page = pdf.get_page(index).map_err(|e| e.to_string())?;
        assert!((page.media_box[2] - 612.0).abs() < 0.01);
        assert!((page.media_box[3] - 792.0).abs() < 0.01);
        let content = pdf
            .get_page_content_streams(&page)
            .map_err(|e| e.to_string())?
            .iter()
            .map(|s| String::from_utf8_lossy(s))
            .collect::<Vec<_>>()
            .join("\n");
        let number = start + index as i32;
        let label = if decorated {
            format!("Page {number} of 2")
        } else {
            number.to_string()
        };
        assert!(
            content.contains(&format!("({label}) Tj")),
            "Missing footer {label}"
        );
        let footer = pattern
            .captures(&content)
            .ok_or("Missing darkblue footer")?;
        let y = footer[2].parse::<f64>().map_err(|e| e.to_string())?;
        assert!(
            y > 0.0 && y < 15.0 * 72.0 / 25.4,
            "Footer below border, above paper edge"
        );
        assert_eq!(
            outlines.find_iter(&content).count(),
            content.matches(" Do").count() + 1,
            "Each visible image has its frame plus the page border"
        );
        images += content.matches(" Do").count();
        contents.push(content);
    }
    assert_eq!(images, 12);
    Ok(contents)
}

#[test]
fn original_txt_raw_and_optional_defaults_preserve_footer_meanings() -> Result<(), String> {
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    for (input, start, decorated) in [
        ("normal.txt", 1, true),
        ("normal-raw.stampalbum", 1, true),
        ("album.txt", 7, true),
        ("album-raw.stampalbum", 7, true),
        ("defaults.txt", 1, false),
        ("defaults-raw.stampalbum", 1, false),
    ] {
        let output = dir.path().join(format!("{input}.pdf"));
        assert_eq!(albumeasy::render_file(&fixture().join(input), &output)?, 2);
        check(&output, start, decorated)?;
    }
    let txt = dir.path().join("normal.txt.pdf");
    let raw = dir.path().join("normal-raw.stampalbum.pdf");
    assert_eq!(check(&txt, 1, true)?, check(&raw, 1, true)?);
    Ok(())
}

#[test]
fn native_document_uses_final_paginated_count_and_explicit_start() -> Result<(), String> {
    let source =
        fs::read_to_string(fixture().join("normal.stampalbum")).map_err(|e| e.to_string())?;
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let mut native_pages = Vec::new();
    for start in [1, 7] {
        let input = source.replace("HN 9 C 1 \"Page \"", &format!("HN 9 C {start} \"Page \""));
        let album = NativeAlbum::parse(&input, &fixture())?;
        let output = dir.path().join(format!("{start}.pdf"));
        assert_eq!(album.write_pdf(&output)?.len(), 2);
        native_pages = check(&output, start, true)?;
    }
    let txt_output = dir.path().join("txt.pdf");
    albumeasy::render_file(&fixture().join("normal.txt"), &txt_output)?;
    let txt_pages = check(&txt_output, 1, true)?;
    let dimensions = |pages: &[String]| -> Result<Vec<(f64, f64)>, String> {
        let mut result = Vec::new();
        for page in pages {
            let tokens: Vec<_> = page.split_whitespace().collect();
            for transform in tokens.windows(9).filter(|w| w[6] == "cm" && w[8] == "Do") {
                result.push((
                    transform[0].parse::<f64>().map_err(|e| e.to_string())?,
                    transform[3].parse::<f64>().map_err(|e| e.to_string())?,
                ));
            }
        }
        result.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
        Ok(result)
    };
    let native_dimensions = dimensions(&native_pages)?;
    let txt_dimensions = dimensions(&txt_pages)?;
    assert_eq!(native_dimensions.len(), 12);
    assert_eq!(txt_dimensions.len(), 12);
    for (native, txt) in native_dimensions.iter().zip(txt_dimensions) {
        assert!((native.0 - txt.0).abs() <= 0.02);
        assert!((native.1 - txt.1).abs() <= 0.02);
    }
    Ok(())
}

#[test]
fn malformed_footer_settings_fail_without_replacing_output() -> Result<(), String> {
    let source = fs::read_to_string(fixture().join("normal.txt")).map_err(|e| e.to_string())?;
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("preserved.pdf");
    fs::write(&output, b"preserved").map_err(|e| e.to_string())?;
    for arguments in [
        "HN 9 C 1.5",
        "HN 9 C 2147483647",
        "HN 9 C \"1\"",
        "HN 9 X",
        "HN -1 C",
        "HN 9 C 1 unquoted",
    ] {
        let input = source.replace("HN 9 C 1 \"Page \" \" of $PAGES$\"", arguments);
        assert!(
            albumeasy::render(&input, &fixture(), &output).is_err(),
            "{arguments}"
        );
        assert_eq!(fs::read(&output).map_err(|e| e.to_string())?, b"preserved");
    }
    for spelling in ["COLOR", "COLOUR"] {
        let input = format!("{source}\n{spelling}_ALBUM_FOOTER (black)\n");
        assert!(albumeasy::render(&input, &fixture(), &output).is_err());
    }
    for colour in ["unknown", "\"darkblue\""] {
        let input = source.replace(
            "COLOR_ALBUM_FOOTER (darkblue)",
            &format!("COLOR_ALBUM_FOOTER ({colour})"),
        );
        assert!(albumeasy::render(&input, &fixture(), &output).is_err());
    }
    let input = source.replace("HN 9 C 1 \"Page \"", "UNKNOWN 9 C 1 \"Page \"");
    let error = albumeasy::render(&input, &fixture(), &output).expect_err("Invalid font must fail");
    assert!(error.contains("Line ") && error.contains("ALBUM_PAGES_FOOTER_NUMBER"));
    assert_eq!(fs::read(&output).map_err(|e| e.to_string())?, b"preserved");
    Ok(())
}

#[test]
fn native_footer_rejects_invalid_settings_duplicates_and_overflow() -> Result<(), String> {
    let source =
        fs::read_to_string(fixture().join("normal.stampalbum")).map_err(|e| e.to_string())?;
    let original = "HN 9 C 1 \"Page \" \" of $PAGES$\" 0 0 darkblue";
    for arguments in [
        "HN 9 C 1 \"Page \" \" of $PAGES$\" 0 0",
        "HN 9 C 1.5 \"Page \" \" of $PAGES$\" 0 0 darkblue",
        "HN 9 C \"1\" \"Page \" \" of $PAGES$\" 0 0 darkblue",
        "HN 9 X 1 \"Page \" \" of $PAGES$\" 0 0 darkblue",
        "HN 9 C 1 unquoted \" of $PAGES$\" 0 0 darkblue",
        "HN 9 C 1 \"Page \" \" of $PAGES$\" NaN 0 darkblue",
        "HN 9 C 1 \"Page \" \" of $PAGES$\" 0 -1 darkblue",
        "HN 9 C 1 \"Page \" \" of $PAGES$\" 0 0 unknown",
    ] {
        let error = match NativeAlbum::parse(&source.replace(original, arguments), &fixture()) {
            Err(error) => error,
            Ok(_) => return Err(format!("Invalid native footer accepted: {arguments}")),
        };
        assert!(error.contains("Line "), "{error}");
    }
    let duplicate = format!("{source}\nNATIVE_PAGE_FOOTER_NUMBER ({original})\n");
    assert!(NativeAlbum::parse(&duplicate, &fixture()).is_err());
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("preserved.pdf");
    fs::write(&output, b"preserved").map_err(|e| e.to_string())?;
    let overflow = source.replace("HN 9 C 1 \"Page \"", "HN 9 C 2147483647 \"Page \"");
    let album = NativeAlbum::parse(&overflow, &fixture())?;
    assert!(album.write_pdf(&output).is_err());
    assert_eq!(fs::read(&output).map_err(|e| e.to_string())?, b"preserved");
    Ok(())
}

#[test]
fn footer_optional_arguments_and_alternating_positions_are_explicit() -> Result<(), String> {
    let source = fs::read_to_string(fixture().join("normal.txt")).map_err(|e| e.to_string())?;
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    for (arguments, label) in [
        ("HN 9 C", "1"),
        ("HN 9 C 7", "7"),
        ("HN 9 C 7 \"Sheet \"", "Sheet 7"),
        ("HN 9 C 7 \"Sheet \" \" / $PAGES$\"", "Sheet 7 / 2"),
    ] {
        let input = source.replace("HN 9 C 1 \"Page \" \" of $PAGES$\"", arguments);
        let output = dir.path().join("optional.pdf");
        assert_eq!(albumeasy::render(&input, &fixture(), &output)?, 2);
        let pdf = PdfDocument::open(&output).map_err(|e| e.to_string())?;
        let page = pdf.get_page(0).map_err(|e| e.to_string())?;
        let streams = pdf
            .get_page_content_streams(&page)
            .map_err(|e| e.to_string())?;
        let content = streams
            .iter()
            .map(|s| String::from_utf8_lossy(s))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(content.contains(&format!("({label}) Tj")));
    }
    let pattern = regex::Regex::new(
        r"0\.000 0\.000 0\.545 rg\s+([0-9.-]+) [0-9.-]+ Td\s+\(Page [12] of 2\) Tj",
    )
    .map_err(|e| e.to_string())?;
    for position in ["L", "R", "LR", "RL"] {
        let input = source.replace("HN 9 C 1", &format!("HN 9 {position} 1"));
        let output = dir.path().join("position.pdf");
        albumeasy::render(&input, &fixture(), &output)?;
        let pages = check(&output, 1, true)?;
        let x: Vec<_> = pages
            .iter()
            .map(|c| {
                let capture = pattern.captures(c).ok_or("Missing footer position")?;
                capture[1].parse::<f64>().map_err(|e| e.to_string())
            })
            .collect::<Result<_, String>>()?;
        match position {
            "L" => assert!((x[0] - x[1]).abs() < 0.01 && x[0] < 100.0),
            "R" => assert!((x[0] - x[1]).abs() < 0.01 && x[0] > 400.0),
            "LR" => assert!(x[0] < 100.0 && x[1] > 400.0),
            "RL" => assert!(x[0] > 400.0 && x[1] < 100.0),
            _ => unreachable!(),
        }
    }
    Ok(())
}

#[test]
#[ignore = "Regenerates Denmark v3 footer proofs; prior fixture versions remain untouched"]
fn regenerate_denmark_footer_proofs() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let previous = root.join(
        "tests/fixtures/albumeasy/denmark-background-native-v2/Denmark-1989-1989-supported.txt",
    );
    let original = fs::read(&previous).map_err(|e| e.to_string())?;
    let source = std::str::from_utf8(&original)
        .map_err(|e| e.to_string())?
        .replace("# NATIVE-TODO: COLOR_ALBUM_FOOTER", "COLOR_ALBUM_FOOTER")
        .replace(
            "# NATIVE-TODO: ALBUM_PAGES_FOOTER_NUMBER",
            "ALBUM_PAGES_FOOTER_NUMBER",
        );
    assert_eq!(source.matches("# NATIVE-TODO:").count(), 3);
    let target = root.join("tests/fixtures/albumeasy/denmark-footer-native-v3");
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
            let streams = pdf
                .get_page_content_streams(&page)
                .map_err(|e| e.to_string())?;
            let content = streams
                .iter()
                .map(|s| String::from_utf8_lossy(s))
                .collect::<Vec<_>>()
                .join("\n");
            assert!(content.contains(&format!("(Page {} of 16) Tj", index + 1)));
            images += content.matches(" Do").count();
        }
        assert_eq!(images, 41);
        println!("{input}: 16 pages / 41 images; numbered footer restored, 3 deferrals retained");
    }
    assert_eq!(fs::read(previous).map_err(|e| e.to_string())?, original);
    Ok(())
}
