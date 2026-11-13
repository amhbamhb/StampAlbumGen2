use oxidize_pdf::parser::PdfDocument;
use stamp_album_gen2::native_pdf::{
    album::NativeAlbum,
    albumeasy,
    colour::NamedColour,
    commands::page::album_pages_margin_txt::{
        AlbumPagesMarginText, MarginAlignment, MarginDirection, MarginPosition,
    },
    document::Bounds,
    text::MeasuredText,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/margin-text-v1")
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
                .map(|s| String::from_utf8_lossy(s))
                .collect::<Vec<_>>()
                .join("\n"))
        })
        .collect()
}

#[test]
fn rotated_layout_matches_pinned_reference_coordinates_and_bounds() -> Result<(), String> {
    let margin = Bounds {
        left: 15.0,
        right: 200.9,
        bottom: 15.0,
        top: 264.4,
    };
    let paper = Bounds {
        left: 0.0,
        right: 215.9,
        bottom: 0.0,
        top: 279.4,
    };
    for (text, position, alignment, direction, gap, expected) in [
        (
            "UNITED STATES - LEFT MARGIN",
            "L",
            "CENTRE",
            "UP",
            2.0,
            vec![(36.85040, 326.241)],
        ),
        (
            "LIBERTY ISSUE - RIGHT MARGIN",
            "R",
            "CENTRE",
            "UP",
            2.0,
            vec![(581.61163, 325.23749)],
        ),
        (
            "TOP LEFT\nREADS DOWN",
            "L",
            "TOP",
            "DOWN",
            4.0,
            vec![(24.71910, 749.48029), (15.71910, 749.48029)],
        ),
        (
            "BOTTOM RIGHT\nREADS UP",
            "R",
            "BOTTOM",
            "UP",
            4.0,
            vec![(587.28088, 42.51968), (596.28088, 42.51968)],
        ),
        (
            "LR - ODD LEFT / EVEN RIGHT",
            "LR",
            "TOP",
            "UP",
            1.0,
            vec![(39.68504, 622.95831)],
        ),
        (
            "RL - ODD RIGHT / EVEN LEFT",
            "RL",
            "BOTTOM",
            "DOWN",
            1.0,
            vec![(572.31494, 169.04169)],
        ),
    ] {
        let text = AlbumPagesMarginText::new(
            MeasuredText::new("HN", 9.0, text)?,
            MarginPosition::parse(position)?,
            MarginAlignment::parse(alignment)?,
            MarginDirection::parse(direction)?,
            gap,
            NamedColour::parse("black")?,
        )?;
        let layout = text.layout(margin, paper, 1)?;
        for (line, (x, y)) in layout.lines.iter().zip(expected) {
            assert!((line.x_mm * 72.0 / 25.4 - x).abs() < 0.01, "{position} x");
            assert!((line.y_mm * 72.0 / 25.4 - y).abs() < 0.01, "{position} y");
            assert!(line.bounds.left >= 0.0 && line.bounds.right <= 215.9);
        }
    }
    Ok(())
}

#[test]
fn all_three_inputs_render_rotated_text_without_altering_stamps() -> Result<(), String> {
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let reference_transforms = regex::Regex::new(
        r"(-?[0-9.]+) (-?[0-9.]+) (-?[0-9.]+) (-?[0-9.]+) (-?[0-9.]+) (-?[0-9.]+) Tm",
    )
    .map_err(|e| e.to_string())?;
    let native_transforms =
        regex::Regex::new(r"(0\.00) (-?1\.00) (-?1\.00) (0\.00) (-?[0-9.]+) (-?[0-9.]+) cm")
            .map_err(|e| e.to_string())?;
    for (name, labels, rgb) in [
        (
            "album",
            vec![
                "UNITED STATES - LEFT MARGIN",
                "LIBERTY ISSUE - RIGHT MARGIN",
            ],
            "0.000 0.000 0.545",
        ),
        (
            "top-down",
            vec!["TOP LEFT", "READS DOWN", "BOTTOM RIGHT", "READS UP"],
            "0.545 0.000 0.000",
        ),
        (
            "alternating",
            vec!["LR - ODD LEFT / EVEN RIGHT", "RL - ODD RIGHT / EVEN LEFT"],
            "0.000 0.392 0.000",
        ),
    ] {
        let reference = contents(&fixture().join(format!("{name}.pdf")))?;
        let mut outputs = Vec::new();
        for suffix in [".txt", "-raw.stampalbum", ".stampalbum"] {
            let input = fixture().join(format!("{name}{suffix}"));
            let output = dir.path().join(format!("{name}{suffix}.pdf"));
            if suffix == ".stampalbum" {
                assert_eq!(NativeAlbum::load(&input)?.write_pdf(&output)?.len(), 2);
            } else {
                assert_eq!(albumeasy::render_file(&input, &output)?, 2);
            }
            let pages = contents(&output)?;
            assert_eq!(pages.len(), 2);
            assert_eq!(
                pages
                    .iter()
                    .map(|p| p.matches(" Do").count())
                    .sum::<usize>(),
                12
            );
            let outlines = pages
                .iter()
                .map(|page| page.matches(" RG\nS").count())
                .sum::<usize>();
            assert_eq!(outlines, 14, "Twelve stamp frames plus two page borders");
            for (index, page) in pages.iter().enumerate() {
                assert!(page.contains(&format!("(Page {} of 2) Tj", index + 1)));
                assert!(page.contains(&format!("{rgb} rg")));
                for label in &labels {
                    assert!(
                        page.contains(&format!("({label}) Tj")),
                        "{name} {suffix}: {label}"
                    );
                }
                assert!(page.contains("0.00 1.00 -1.00 0.00"), "{page}");
                let expected = reference_transforms
                    .captures_iter(&reference[index])
                    .map(|capture| {
                        (1..=6)
                            .map(|i| capture[i].parse::<f64>().map_err(|e| e.to_string()))
                            .collect::<Result<Vec<_>, String>>()
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                let actual = native_transforms
                    .captures_iter(page)
                    .map(|capture| {
                        (1..=6)
                            .map(|i| capture[i].parse::<f64>().map_err(|e| e.to_string()))
                            .collect::<Result<Vec<_>, String>>()
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                assert_eq!(actual.len(), expected.len());
                for (actual, expected) in actual.iter().zip(expected) {
                    for (actual, expected) in actual.iter().zip(expected) {
                        assert!(
                            (actual - expected).abs() < 0.01,
                            "{name}{suffix} page {}: native {actual}, reference {expected}",
                            index + 1
                        );
                    }
                }
            }
            outputs.push(pages);
        }
        assert_eq!(outputs[0], outputs[1], "TXT/raw content equivalence");
        let dimensions = |pages: &[String]| -> Result<Vec<(f64, f64)>, String> {
            let mut result = Vec::new();
            for page in pages {
                let tokens: Vec<_> = page.split_whitespace().collect();
                for w in tokens.windows(9).filter(|w| w[6] == "cm" && w[8] == "Do") {
                    result.push((
                        w[0].parse::<f64>().map_err(|e| e.to_string())?,
                        w[3].parse::<f64>().map_err(|e| e.to_string())?,
                    ));
                }
            }
            result.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
            Ok(result)
        };
        for (txt, native) in dimensions(&outputs[0])?
            .iter()
            .zip(dimensions(&outputs[2])?)
        {
            assert!(
                (txt.0 - native.0).abs() <= 0.02 && (txt.1 - native.1).abs() <= 0.02,
                "Physical image dimensions differ for {name}"
            );
        }
    }
    Ok(())
}

#[test]
fn global_colour_and_padding_are_resolved_after_definitions_and_no_text_is_valid(
) -> Result<(), String> {
    let source = fs::read_to_string(fixture().join("album.txt")).map_err(|e| e.to_string())?;
    let modified = source.replace(
        "ALBUM_PAGES_MARGIN_TXT (HN 9 \"LIBERTY ISSUE - RIGHT MARGIN\" R)",
        "ALBUM_PAGES_MARGIN_TXT (HN 9 \"LIBERTY ISSUE - RIGHT MARGIN\" R)\nCOLOUR_ALBUM_MARGIN_TXT (darkred)\nALBUM_PAGES_MARGIN_TXT_PAD (1)",
    );
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("global.pdf");
    albumeasy::render(&modified, &fixture(), &output)?;
    let pages = contents(&output)?;
    for page in &pages {
        assert!(page.contains("0.545 0.000 0.000 rg"));
        assert!(page.contains("0.00 1.00 -1.00 0.00 34.02"));
    }
    let no_text = source
        .lines()
        .filter(|line| !line.starts_with("ALBUM_PAGES_MARGIN_TXT ("))
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(albumeasy::render(&no_text, &fixture(), &output)?, 2);
    assert!(!contents(&output)?
        .join("\n")
        .contains("0.00 1.00 -1.00 0.00"));
    Ok(())
}

#[test]
fn physical_page_parity_not_printed_number_controls_margin_side() -> Result<(), String> {
    let source = fs::read_to_string(fixture().join("alternating.txt"))
        .map_err(|e| e.to_string())?
        .replace("HN 9 C 1 \"Page \"", "HN 9 C 10 \"Page \"");
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("parity.pdf");
    albumeasy::render(&source, &fixture(), &output)?;
    let pages = contents(&output)?;
    assert!(pages[0].contains("(Page 10 of 2) Tj"));
    assert!(pages[1].contains("(Page 11 of 2) Tj"));
    // Each page has a single upward and downward margin transform.
    let transforms = regex::Regex::new(r"0\.00 1\.00 -1\.00 0\.00 ([0-9.]+) ([0-9.]+) cm")
        .map_err(|e| e.to_string())?;
    let first = transforms
        .captures(&pages[0])
        .ok_or("Missing odd-page margin")?;
    let second = transforms
        .captures(&pages[1])
        .ok_or("Missing even-page margin")?;
    assert!(first[1].parse::<f64>().map_err(|e| e.to_string())? < 100.0);
    assert!(second[1].parse::<f64>().map_err(|e| e.to_string())? > 500.0);
    Ok(())
}

#[test]
fn malformed_overflow_conflict_and_mid_document_changes_preserve_output() -> Result<(), String> {
    let source = fs::read_to_string(fixture().join("album.txt")).map_err(|e| e.to_string())?;
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("preserved.pdf");
    fs::write(&output, b"preserved").map_err(|e| e.to_string())?;
    let original = "HN 9 \"UNITED STATES - LEFT MARGIN\" L";
    for replacement in [
        "HN 9 \"Text\" C",
        "HN 9 \"Text\" L CENTER",
        "HN 9 \"Text\" L TOP SIDEWAYS",
        "HN -1 \"Text\" L",
        "UNKNOWN 9 \"Text\" L",
        "HN 9 unquoted L",
        "HN 9 \"Text\" \"L\"",
        "HN 900 \"Too large\" L",
    ] {
        let error = albumeasy::render(&source.replace(original, replacement), &fixture(), &output)
            .expect_err("Invalid margin command must fail");
        assert!(error.contains("Line") || error.contains("ALBUM_PAGES_MARGIN_TXT"));
        assert_eq!(fs::read(&output).map_err(|e| e.to_string())?, b"preserved");
    }
    for extra in [
        "ALBUM_PAGES_MARGIN_TXT_PAD (5)",
        "COLOUR_ALBUM_MARGIN_TXT (red)",
        "ALBUM_PAGES_MARGIN_TXT (HN 9 \"Later\" R)",
    ] {
        assert!(albumeasy::render(&format!("{source}\n{extra}\n"), &fixture(), &output).is_err());
    }
    for input in [
        source.replace(
            "LIBERTY ISSUE - RIGHT MARGIN\" R",
            "UNITED STATES - LEFT MARGIN\" L",
        ),
        source.replace(
            "COLOR_ALBUM_MARGIN_TXT (darkblue)",
            "COLOR_ALBUM_MARGIN_TXT (notacolour)",
        ),
        source.replace(
            original,
            "HN 9 \"Text\" L\nALBUM_PAGES_MARGIN_TXT (HN 9 \"Third\" L)",
        ),
        source.replace(
            "COLOR_ALBUM_MARGIN_TXT (darkblue)",
            "ALBUM_PAGES_MARGIN_TXT_PAD (99)",
        ),
    ] {
        assert!(albumeasy::render(&input, &fixture(), &output).is_err());
        assert_eq!(fs::read(&output).map_err(|e| e.to_string())?, b"preserved");
    }
    let native =
        fs::read_to_string(fixture().join("album.stampalbum")).map_err(|e| e.to_string())?;
    for input in [
        native.replace("L CENTRE UP 2 darkblue", "C CENTRE UP 2 darkblue"),
        native.replace("L CENTRE UP 2 darkblue", "L CENTRE UP NaN darkblue"),
        native.replace("L CENTRE UP 2 darkblue", "L CENTRE UP 2"),
        format!("{native}\nNATIVE_PAGE_MARGIN_TEXT (HN 9 \"Third\" R CENTRE UP 2 darkblue)\n"),
    ] {
        assert!(NativeAlbum::parse(&input, &fixture()).is_err());
    }
    let album = NativeAlbum::parse(
        &native.replace("L CENTRE UP 2 darkblue", "L CENTRE UP 100 darkblue"),
        &fixture(),
    )?;
    assert!(album.write_pdf(&output).is_err());
    assert_eq!(fs::read(&output).map_err(|e| e.to_string())?, b"preserved");
    Ok(())
}

#[test]
#[ignore = "Regenerates Denmark v5 with zero deferred occurrences; previous proofs unchanged"]
fn regenerate_denmark_margin_proofs() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let previous = root.join(
        "tests/fixtures/albumeasy/denmark-settings-native-v4/Denmark-1989-1989-supported.txt",
    );
    let before = fs::read(&previous).map_err(|e| e.to_string())?;
    let source = std::str::from_utf8(&before)
        .map_err(|e| e.to_string())?
        .replace(
            "# NATIVE-TODO: COLOR_ALBUM_MARGIN_TXT",
            "COLOR_ALBUM_MARGIN_TXT",
        );
    assert_eq!(source.matches("# NATIVE-TODO:").count(), 0);
    let target = root.join("tests/fixtures/albumeasy/denmark-margin-native-v5");
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
        let mut images = 0;
        for index in 0..16 {
            let page = pdf.get_page(index).map_err(|e| e.to_string())?;
            assert!((page.media_box[2] - 216.0 * 72.0 / 25.4).abs() < 0.01);
            let streams = pdf
                .get_page_content_streams(&page)
                .map_err(|e| e.to_string())?;
            let content = streams
                .iter()
                .map(|s| String::from_utf8_lossy(s))
                .collect::<Vec<_>>()
                .join("\n");
            images += content.matches(" Do").count();
            assert!(content.contains(&format!("(Page {} of 16) Tj", index + 1)));
        }
        assert_eq!(images, 41);
        println!("{input}: 16 pages / 41 images / zero commented deferrals");
    }
    assert_eq!(fs::read(previous).map_err(|e| e.to_string())?, before);
    Ok(())
}
