use oxidize_pdf::parser::PdfDocument;
use stamp_album_gen2::native_pdf::{album::NativeAlbum, albumeasy, config::NativePdfConfig};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/page-start-var-v1")
}

fn contents(path: &Path) -> Result<Vec<String>, String> {
    let pdf = PdfDocument::open(path).map_err(|e| e.to_string())?;
    (0..pdf.page_count().map_err(|e| e.to_string())?)
        .map(|i| {
            let page = pdf.get_page(i).map_err(|e| e.to_string())?;
            assert!((page.media_box[2] - 612.0).abs() < 0.01);
            assert!((page.media_box[3] - 792.0).abs() < 0.01);
            Ok(pdf
                .get_page_content_streams(&page)
                .map_err(|e| e.to_string())?
                .iter()
                .map(|s| String::from_utf8_lossy(s).into_owned())
                .collect::<Vec<_>>()
                .join("\n"))
        })
        .collect()
}

fn text_position(page: &str, text: &str) -> Result<(f64, f64), String> {
    let pattern = regex::Regex::new(&format!(
        r"([0-9.-]+) ([0-9.-]+) Td\s+\({}\) Tj",
        regex::escape(text)
    ))
    .map_err(|e| e.to_string())?;
    let captures = pattern
        .captures(page)
        .ok_or_else(|| format!("Missing positioned text: {text}"))?;
    Ok((
        captures[1].parse::<f64>().map_err(|e| e.to_string())? * 25.4 / 72.0,
        captures[2].parse::<f64>().map_err(|e| e.to_string())? * 25.4 / 72.0,
    ))
}

#[test]
fn original_and_raw_inputs_preserve_spacing_and_substitution_offsets() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    assert_eq!(
        fs::read(fixture().join("album.txt")).map_err(|e| e.to_string())?,
        fs::read(fixture().join("album-raw.stampalbum")).map_err(|e| e.to_string())?
    );
    for input in ["album.txt", "album-raw.stampalbum"] {
        let output = temp.path().join("proof.pdf");
        assert_eq!(albumeasy::render_file(&fixture().join(input), &output)?, 4);
        let pages = contents(&output)?;
        let mut ys = Vec::new();
        let mut titles = Vec::new();
        let mut row_gaps = Vec::new();
        for (i, page) in pages.iter().enumerate() {
            assert_eq!(page.matches(" Do").count(), 4);
            assert!(page.contains(&format!("Year: {} | repeated: {}", 1959 + i, 1959 + i)));
            assert!(page.contains(&format!("Page {} of 4", i + 1)));
            assert!(!page.contains("$$"));
            ys.push(
                text_position(
                    page,
                    &format!("Year: {} | repeated: {}", 1959 + i, 1959 + i),
                )?
                .1,
            );
            titles.push(text_position(page, "UNITED STATES - PER-PAGE SPACING")?.1);
            row_gaps.push(
                text_position(page, "DB 150 / SG 1125a")?.1
                    - text_position(page, "DB 152 / SG 1137b")?.1,
            );
        }
        assert!((ys[1] - ys[0] - 8.0).abs() < 0.01);
        assert!((ys[2] - ys[0] - 14.0).abs() < 0.01);
        assert!((ys[3] - ys[0]).abs() < 0.01);
        assert!(titles.iter().all(|v| (v - titles[0]).abs() < 0.01));
        assert!((row_gaps[0] - row_gaps[1] - 8.0).abs() < 0.01);
        assert!((row_gaps[0] - row_gaps[2] - 14.0).abs() < 0.01);
        assert!((row_gaps[3] - row_gaps[0]).abs() < 0.01);
    }
    Ok(())
}

#[test]
fn variable_start_can_be_the_only_page_start_and_needs_no_group() -> Result<(), String> {
    let source = fs::read_to_string(fixture().join("album.txt")).map_err(|e| e.to_string())?;
    let declarations = source
        .split("PAGE_START_GROUP_BEGIN")
        .next()
        .ok_or("Missing group")?;
    let stamp = "ROW_START_ES (HN 8 0.1)\nSTAMP_ADD_IMG (58 35 \"../stamp-pagination-v1/us-150.png\" \"\" \"single\" \"\")";
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("single.pdf");
    assert_eq!(
        albumeasy::render(
            &format!("{declarations}\nPAGE_START_VAR (12 6)\n{stamp}"),
            &fixture(),
            &output
        )?,
        1
    );
    assert_eq!(contents(&output)?[0].matches(" Do").count(), 1);
    Ok(())
}

#[test]
fn invalid_overrides_and_recursive_groups_preserve_outputs() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("preserve.pdf");
    fs::write(&output, b"existing").map_err(|e| e.to_string())?;
    for source in [
        "PAGE_START_VAR",
        "PAGE_START_VAR (4)",
        "PAGE_START_VAR (\"4\" 6)",
        "PAGE_START_VAR (NaN 6)",
        "PAGE_START_VAR (-1 6)",
        "PAGE_START_VAR (4 -1)",
        "PAGE_START_VAR (4 6 1 2 3 4 5 6 7 8 9 10)",
        "PAGE_START_VAR (4 6 extra)",
        "PAGE_START_GROUP_BEGIN\nPAGE_START_VAR (4 6)\nPAGE_START_GROUP_END",
        "PAGE_START_GROUP_BEGIN\nPAGE_TEXT_CENTRE (HN 9 $$2)\nPAGE_START_GROUP_END\nPAGE_START_VAR (4 6 onlyone)",
    ] {
        assert!(
            albumeasy::render(source, &fixture(), &output).is_err(),
            "{source}"
        );
        assert_eq!(fs::read(&output).map_err(|e| e.to_string())?, b"existing");
    }
    let native =
        fs::read_to_string(fixture().join("album.stampalbum")).map_err(|e| e.to_string())?;
    for args in ["12", "12 6 1", "-1 6", "12 NaN", "\"12\" 6"] {
        assert!(
            NativeAlbum::parse(
                &native.replace(
                    "NATIVE_PAGE_BREAK (12 6)",
                    &format!("NATIVE_PAGE_BREAK ({args})")
                ),
                &fixture()
            )
            .is_err()
        );
    }
    Ok(())
}

#[test]
fn native_overrides_restore_and_preserve_frame_geometry_with_both_paginators() -> Result<(), String>
{
    let source =
        fs::read_to_string(fixture().join("album.stampalbum")).map_err(|e| e.to_string())?;
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    for headings in [true, false] {
        let source = source
            .lines()
            .filter(|l| headings || !l.starts_with("NATIVE_STAMP_HEADING "))
            .collect::<Vec<_>>()
            .join("\n");
        let output = temp.path().join("native.pdf");
        let pages = NativeAlbum::parse(&source, &fixture())?.write_pdf(&output)?;
        assert_eq!(pages.len(), 4);
        for (i, page) in pages.iter().enumerate() {
            assert_eq!(page.items.len(), 4);
            let expected_left = [19.2, 27.2, 15.2, 19.2][i];
            assert!((page.layout.content_bounds.left - expected_left).abs() < 1e-8);
            for item in &page.items {
                let frame = item.geometry.ok_or("Missing frame")?.frame_bounds();
                assert!((frame.top - frame.bottom - 35.0).abs() < 1e-8);
                assert!(item.footprint.left >= page.layout.content_bounds.left);
                assert!(item.footprint.right <= page.layout.content_bounds.right);
            }
        }
        let titles = pages
            .iter()
            .map(|p| {
                p.layout
                    .title
                    .as_ref()
                    .ok_or("Missing title")
                    .map(|t| t.lines[0].baseline_mm)
            })
            .collect::<Result<Vec<_>, _>>()?;
        assert!(titles.iter().all(|v| (v - titles[0]).abs() < 1e-8));
        let top = pages[0].layout.content_bounds.top;
        assert!((pages[1].layout.content_bounds.top - top - 8.0).abs() < 1e-8);
        assert!((pages[2].layout.content_bounds.top - top - 14.0).abs() < 1e-8);
        assert!((pages[3].layout.content_bounds.top - top).abs() < 1e-8);
    }
    Ok(())
}

#[test]
fn configured_document_numbering_keeps_per_page_bounds() -> Result<(), String> {
    let source = include_str!("../config_template.toml").replace(
        "[native_pdf.footer_number]\nenabled = false",
        "[native_pdf.footer_number]\nenabled = true",
    );
    let config = NativePdfConfig::from_toml(&source)?;
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("numbered.pdf");
    let album = NativeAlbum::load(&fixture().join("album.stampalbum"))?;
    let pages = album.write_pdf_with_config(&config, &output)?;
    assert_eq!(pages.len(), 4);
    assert!((pages[1].layout.content_bounds.left - 27.2).abs() < 1e-8);
    for (i, page) in contents(&output)?.iter().enumerate() {
        assert!(page.contains(&format!("Page {} of 4", i + 1)));
    }
    Ok(())
}

#[test]
fn variable_group_supports_all_nine_values_after_spacing_arguments() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("nine.pdf");
    let source = "ALBUM_PAGES_SIZE (215.9 279.4)
ALBUM_PAGES_MARGINS (15 15 15 15)
ALBUM_PAGES_SPACING (4 14)
ALBUM_PAGES_BORDER (0.2 0 0 0)
PAGE_START_GROUP_BEGIN
PAGE_TEXT_CENTRE (HN 9 $$1)
PAGE_TEXT_CENTRE (HN 9 \"$$2|$$3|$$4|$$5|$$6|$$7|$$8|$$9\")
PAGE_START_GROUP_END
PAGE_START_VAR (12 6 \"two words\" 2 3 4 5 6 7 8 nine)";
    assert_eq!(albumeasy::render(source, &fixture(), &output)?, 1);
    let pages = contents(&output)?;
    assert!(pages[0].contains("(two words) Tj"));
    assert!(pages[0].contains("(2|3|4|5|6|7|8|nine) Tj"));
    Ok(())
}

#[test]
fn automatic_continuations_inherit_override_until_the_next_explicit_break() -> Result<(), String> {
    let original =
        fs::read_to_string(fixture().join("album.stampalbum")).map_err(|e| e.to_string())?;
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    for headings in [true, false] {
        let source = original
            .lines()
            .filter(|line| headings || !line.starts_with("NATIVE_STAMP_HEADING "))
            .collect::<Vec<_>>()
            .join("\n")
            .replace("215.9 279.4", "215.9 150");
        let output = temp.path().join("overflow.pdf");
        let pages = NativeAlbum::parse(&source, &fixture())?.write_pdf(&output)?;
        assert!(pages.len() > 4);
        assert_eq!(
            pages
                .iter()
                .flat_map(|p| p.items.iter().map(|i| i.source_index))
                .collect::<Vec<_>>(),
            (0..16).collect::<Vec<_>>()
        );
        let overrides = pages
            .iter()
            .filter(|p| p.items.iter().any(|i| (4..8).contains(&i.source_index)))
            .collect::<Vec<_>>();
        assert_eq!(overrides.len(), 2);
        assert!(
            overrides
                .iter()
                .all(|p| (p.layout.content_bounds.left - 27.2).abs() < 1e-8)
        );
        let restored = pages
            .iter()
            .filter(|p| p.items.iter().any(|i| i.source_index >= 12));
        assert!(
            restored
                .into_iter()
                .all(|p| (p.layout.content_bounds.left - 19.2).abs() < 1e-8)
        );
    }
    Ok(())
}

#[test]
fn leading_repeated_and_trailing_variable_breaks_keep_blank_page_layouts() -> Result<(), String> {
    let original =
        fs::read_to_string(fixture().join("album.stampalbum")).map_err(|e| e.to_string())?;
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    for headings in [true, false] {
        let source = original
            .lines()
            .filter(|line| headings || !line.starts_with("NATIVE_STAMP_HEADING "))
            .collect::<Vec<_>>()
            .join("\n")
            .replace(
                "NATIVE_PAGE_BREAK (12 6)",
                "NATIVE_PAGE_BREAK (12 6)\nNATIVE_PAGE_BREAK (12 6)",
            )
            .replacen(
                "STAMP_IMAGE (",
                "NATIVE_PAGE_BREAK (12 6)\nSTAMP_IMAGE (",
                1,
            );
        let source = format!("{source}\nNATIVE_PAGE_BREAK (12 6)");
        let output = temp.path().join("blank.pdf");
        let pages = NativeAlbum::parse(&source, &fixture())?.write_pdf(&output)?;
        assert_eq!(
            pages.iter().map(|p| p.items.len()).collect::<Vec<_>>(),
            vec![0, 4, 0, 4, 4, 4, 0]
        );
        for (page, left) in pages.iter().zip([19.2, 27.2, 27.2, 27.2, 15.2, 19.2, 27.2]) {
            assert!((page.layout.content_bounds.left - left).abs() < 1e-8);
        }
        for (i, page) in contents(&output)?.iter().enumerate() {
            assert!(page.contains(&format!("(Page {} of 7) Tj", i + 1)));
        }
    }
    Ok(())
}

#[test]
fn section_numbering_preserves_override_and_reset_layouts() -> Result<(), String> {
    let source = fs::read_to_string(fixture().join("album.stampalbum"))
        .map_err(|e| e.to_string())?
        .replace(
            "NATIVE_PAGE_BREAK (0 0)",
            "NATIVE_PAGE_NUMBER_SECTION ()\nNATIVE_PAGE_BREAK (0 0)",
        );
    let config = NativePdfConfig::from_toml(
        &include_str!("../config_template.toml")
            .replace(
                "[native_pdf.footer_number]\nenabled = false",
                "[native_pdf.footer_number]\nenabled = true",
            )
            .replace("scope = \"document\"", "scope = \"section\"")
            .replace("section_start = 1", "section_start = 10"),
    )?;
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("sections.pdf");
    let pages = NativeAlbum::parse(&source, &fixture())?.write_pdf_with_config(&config, &output)?;
    assert_eq!(pages.len(), 4);
    for (page, left) in pages.iter().zip([19.2, 27.2, 15.2, 19.2]) {
        assert!((page.layout.content_bounds.left - left).abs() < 1e-8);
    }
    for (page, ordinal) in contents(&output)?.iter().zip([10, 11, 10, 11]) {
        assert!(page.contains(&format!("(Page {ordinal} of 4) Tj")));
    }
    Ok(())
}

#[test]
#[ignore = "Regenerate PAGE_START_VAR native proofs after reference acceptance"]
fn generate_page_start_var_proofs() -> Result<(), String> {
    let root = fixture();
    assert_eq!(
        fs::read(root.join("album.txt")).map_err(|e| e.to_string())?,
        fs::read(root.join("album-raw.stampalbum")).map_err(|e| e.to_string())?
    );
    albumeasy::render_file(&root.join("album.txt"), &root.join("album-txt-native.pdf"))?;
    albumeasy::render_file(
        &root.join("album-raw.stampalbum"),
        &root.join("album-raw-native.pdf"),
    )?;
    NativeAlbum::load(&root.join("album.stampalbum"))?.write_pdf(&root.join("album-native.pdf"))?;
    Ok(())
}
