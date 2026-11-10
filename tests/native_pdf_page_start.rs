use oxidize_pdf::parser::PdfDocument;
use stamp_album_gen2::native_pdf::{album::NativeAlbum, albumeasy};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/page-start-v1")
}

fn contents(path: &Path) -> Result<Vec<String>, String> {
    let pdf = PdfDocument::open(path).map_err(|error| error.to_string())?;
    (0..pdf.page_count().map_err(|error| error.to_string())?)
        .map(|index| {
            let page = pdf.get_page(index).map_err(|error| error.to_string())?;
            assert!((page.media_box[2] - 612.0).abs() < 0.01);
            assert!((page.media_box[3] - 792.0).abs() < 0.01);
            Ok(pdf
                .get_page_content_streams(&page)
                .map_err(|error| error.to_string())?
                .iter()
                .map(|stream| String::from_utf8_lossy(stream).into_owned())
                .collect::<Vec<_>>()
                .join("\n"))
        })
        .collect()
}

fn assert_pair(pages: &[String]) {
    assert_eq!(pages.len(), 2);
    for (index, page) in pages.iter().enumerate() {
        assert_eq!(page.matches(" Do").count(), 2);
        assert!(page.contains(&format!("(Page {} of 2) Tj", index + 1)));
        for label in if index == 0 {
            ["SG 1125a", "SG 1137a"]
        } else {
            ["SG 1137b", "SG 1148a"]
        } {
            assert!(page.contains(label), "{label}");
        }
        assert!(!page.contains("$$"));
    }
}

#[test]
fn original_txt_and_byte_identical_raw_documents_preserve_page_groups() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|error| error.to_string())?;
    for stem in ["basic", "album"] {
        let txt = fixture().join(format!("{stem}.txt"));
        let raw = fixture().join(format!("{stem}-raw.stampalbum"));
        assert_eq!(
            fs::read(&txt).map_err(|e| e.to_string())?,
            fs::read(&raw).map_err(|e| e.to_string())?
        );
        for input in [txt, raw] {
            let output = temp.path().join("render.pdf");
            assert_eq!(albumeasy::render_file(&input, &output)?, 2);
            let pages = contents(&output)?;
            assert_pair(&pages);
            if stem == "album" {
                for (index, year) in ["1959", "1960"].iter().enumerate() {
                    assert!(pages[index].contains(&format!("Year: {year} | Year repeated: {year}")));
                    assert!(!pages[index].contains(if index == 0 { "1960" } else { "1959" }));
                }
            }
        }
    }
    Ok(())
}

#[test]
fn native_breaks_flush_rows_and_preserve_numbering_in_both_paginators() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|error| error.to_string())?;
    for stem in ["basic", "album"] {
        let source = fs::read_to_string(fixture().join(format!("{stem}.stampalbum")))
            .map_err(|e| e.to_string())?;
        for headings in [true, false] {
            let source = source
                .lines()
                .filter(|line| headings || !line.starts_with("NATIVE_STAMP_HEADING "))
                .collect::<Vec<_>>()
                .join("\n");
            let album = NativeAlbum::parse(&source, &fixture())?;
            let output = temp.path().join("native.pdf");
            let pages = album.write_pdf(&output)?;
            for page in &pages {
                for item in &page.items {
                    let geometry = item.geometry.ok_or("Missing frame")?;
                    let frame = geometry.frame_bounds();
                    let width = if [0, 3].contains(&item.source_index) {
                        58.0
                    } else {
                        32.0
                    };
                    assert!((frame.right - frame.left - width).abs() < 1e-8);
                    assert!((frame.top - frame.bottom - 35.0).abs() < 1e-8);
                    let bounds = page.layout.content_bounds;
                    assert!(
                        item.footprint.left >= bounds.left && item.footprint.right <= bounds.right
                    );
                    assert!(
                        item.footprint.bottom >= bounds.bottom && item.footprint.top <= bounds.top
                    );
                }
            }
            assert_eq!(
                pages
                    .iter()
                    .map(|p| p.items.iter().map(|i| i.source_index).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
                vec![vec![0, 1], vec![2, 3]]
            );
            assert_pair(&contents(&output)?);
            let no_break = NativeAlbum::parse(
                &source.replace("NATIVE_PAGE_BREAK ()", "NATIVE_ROW_BREAK ()"),
                &fixture(),
            )?;
            assert_eq!(no_break.write_pdf(&output)?.len(), 1);
        }
    }
    Ok(())
}

#[test]
fn intentional_leading_repeated_and_trailing_blank_pages_are_numbered() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let original =
        fs::read_to_string(fixture().join("basic.stampalbum")).map_err(|e| e.to_string())?;
    for headings in [true, false] {
        let source = original.lines().filter(|line| headings || !line.starts_with("NATIVE_STAMP_HEADING ")).collect::<Vec<_>>().join("\n")
            .replacen("STAMP_IMAGE (", "NATIVE_PAGE_BREAK ()\nSTAMP_IMAGE (", 1)
            .replace("NATIVE_PAGE_BREAK ()\nSTAMP_IMAGE (\"../stamp-pagination-v1/us-152.png\"", "NATIVE_PAGE_BREAK ()\nNATIVE_PAGE_BREAK ()\nSTAMP_IMAGE (\"../stamp-pagination-v1/us-152.png\"");
        let source = format!("{source}\nNATIVE_PAGE_BREAK ()\n");
        let output = temp.path().join("blank.pdf");
        let pages = NativeAlbum::parse(&source, &fixture())?.write_pdf(&output)?;
        assert_eq!(
            pages.iter().map(|p| p.items.len()).collect::<Vec<_>>(),
            vec![0, 2, 0, 2, 0]
        );
        let streams = contents(&output)?;
        assert_eq!(streams.len(), 5);
        for (index, page) in streams.iter().enumerate() {
            assert!(page.contains(&format!("(Page {} of 5) Tj", index + 1)));
            assert_eq!(
                page.matches(" Do").count(),
                if [1, 3].contains(&index) { 2 } else { 0 }
            );
        }
    }
    Ok(())
}

#[test]
fn automatic_overflow_and_explicit_breaks_coexist_without_repeating_items() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let original =
        fs::read_to_string(fixture().join("basic.stampalbum")).map_err(|e| e.to_string())?;
    for headings in [true, false] {
        let source = original
            .lines()
            .filter(|line| headings || !line.starts_with("NATIVE_STAMP_HEADING "))
            .collect::<Vec<_>>()
            .join("\n")
            .replace("215.9 279.4", "215.9 140");
        let first = source.find("STAMP_IMAGE (").ok_or("Missing first stamp")?;
        let boundary = source.find("NATIVE_PAGE_BREAK ()").ok_or("Missing break")?;
        let first_row = &source[first..boundary];
        let source = source.replace(
            "NATIVE_PAGE_BREAK ()",
            &format!("NATIVE_ROW_BREAK ()\n{first_row}NATIVE_PAGE_BREAK ()"),
        );
        let output = temp.path().join("overflow.pdf");
        let pages = NativeAlbum::parse(&source, &fixture())?.write_pdf(&output)?;
        assert_eq!(
            pages
                .iter()
                .map(|p| p.items.iter().map(|i| i.source_index).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            vec![vec![0, 1], vec![2, 3], vec![4, 5]]
        );
    }
    Ok(())
}

#[test]
fn caption_continuation_finishes_before_the_next_explicit_page() -> Result<(), String> {
    let original =
        fs::read_to_string(fixture().join("basic.stampalbum")).map_err(|e| e.to_string())?;
    let lines = (0..180)
        .map(|i| format!("Continuation line {i:03}"))
        .collect::<Vec<_>>();
    let source = original
        .lines()
        .filter(|line| !line.starts_with("NATIVE_STAMP_HEADING "))
        .collect::<Vec<_>>()
        .join("\n")
        .replace("First-page wide design", &lines.join("\\n"));
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("continuation.pdf");
    let pages = NativeAlbum::parse(&source, &fixture())?.write_pdf(&output)?;
    assert!(pages.len() > 2);
    let frames = pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter(|item| item.geometry.is_some())
        .map(|item| item.source_index)
        .collect::<Vec<_>>();
    assert_eq!(frames, vec![0, 1, 2, 3]);
    let previous = pages
        .iter()
        .rposition(|p| p.items.iter().any(|i| i.source_index < 2))
        .ok_or("Missing preceding section")?;
    let next = pages
        .iter()
        .position(|p| p.items.iter().any(|i| i.source_index >= 2))
        .ok_or("Missing next section")?;
    assert_eq!(next, previous + 1);
    let streams = contents(&output)?;
    let all = streams.join("\n");
    for line in lines {
        assert_eq!(all.matches(&format!("({line}) Tj")).count(), 1);
    }
    for (index, page) in streams.iter().enumerate() {
        assert!(page.contains(&format!("(Page {} of {}) Tj", index + 1, streams.len())));
    }
    Ok(())
}

#[test]
fn groups_reject_malformed_input_and_preserve_existing_output() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("preserved.pdf");
    fs::write(&output, b"preserve me").map_err(|e| e.to_string())?;
    let cases = [
        ("PAGE_START (one)", "require a PAGE_START group"),
        ("PAGE_START_GROUP_END", "no open group"),
        ("PAGE_START_GROUP_BEGIN", "Unclosed"),
        ("PAGE_START_GROUP_BEGIN\nPAGE_START_GROUP_BEGIN", "only one"),
        ("PAGE_START_GROUP_BEGIN\nPAGE_START\nPAGE_START_GROUP_END", "inside its own group"),
        ("PAGE_START\nPAGE_START_GROUP_BEGIN", "before all pages"),
        ("PAGE_START_GROUP_BEGIN\nPAGE_TEXT_CENTRE (HN 9 $$2)\nPAGE_START_GROUP_END\nPAGE_START (one)", "Missing PAGE_START argument for $$2"),
        ("PAGE_START_GROUP_BEGIN\nPAGE_TEXT_CENTRE (HN 9 \"$$0\")\nPAGE_START_GROUP_END\nPAGE_START", "$$1 through $$9"),
        ("PAGE_START_GROUP_BEGIN\nPAGE_TEXT_CENTRE (HN 9 \"$$1\")\nPAGE_START_GROUP_END\nPAGE_START (\"quoted value\")", "invoked at line 4"),
    ];
    for (source, expected) in cases {
        let error = albumeasy::render(source, &fixture(), &output).expect_err(source);
        assert!(error.contains(expected), "{error}");
        assert_eq!(
            fs::read(&output).map_err(|e| e.to_string())?,
            b"preserve me"
        );
    }
    Ok(())
}

#[test]
fn title_page_spacing_matches_reference_before_and_after_the_title() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let source = fs::read_to_string(fixture().join("album.txt")).map_err(|e| e.to_string())?;
    let title = "ALBUM_PAGES_TITLE (TB 16 \"UNITED STATES - LIBERTY ISSUE\" 5)";
    let baseline = regex::Regex::new(r"([0-9.-]+) ([0-9.-]+) Td\s+\(United States\) Tj")
        .map_err(|e| e.to_string())?;
    for with_title in [true, false] {
        let mut baselines = Vec::new();
        for spacing in [0, 14] {
            let source = source.replace(
                "ALBUM_PAGES_SPACING (4 14)",
                &format!("ALBUM_PAGES_SPACING (4 {spacing})"),
            );
            let source = if with_title {
                source
            } else {
                source.replace(title, "")
            };
            let output = temp.path().join("spacing.pdf");
            albumeasy::render(&source, &fixture(), &output)?;
            let page = &contents(&output)?[0];
            let captures = baseline
                .captures(page)
                .ok_or("Missing group text baseline")?;
            baselines.push(captures[2].parse::<f64>().map_err(|e| e.to_string())?);
        }
        let expected_mm = if with_title { 28.0 } else { 0.0 };
        assert!(((baselines[0] - baselines[1]) * 25.4 / 72.0 - expected_mm).abs() < 0.01);
    }
    Ok(())
}

#[test]
fn break_arguments_fail_and_explicit_empty_albums_render() -> Result<(), String> {
    let source =
        fs::read_to_string(fixture().join("basic.stampalbum")).map_err(|e| e.to_string())?;
    let invalid = source.replace("NATIVE_PAGE_BREAK ()", "NATIVE_PAGE_BREAK (1)");
    assert!(NativeAlbum::parse(&invalid, &fixture())
        .expect_err("break args")
        .contains("takes no arguments"));
    let declarations = source
        .split("# Native attached")
        .next()
        .ok_or("Missing fixture marker")?;
    let album = NativeAlbum::parse(
        &format!("{declarations}\nNATIVE_PAGE_BREAK ()\nNATIVE_PAGE_BREAK ()"),
        &fixture(),
    )?;
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("empty.pdf");
    assert_eq!(album.write_pdf(&output)?.len(), 3);
    assert!(contents(&output)?.iter().all(|page| !page.contains(" Do")));
    Ok(())
}

#[test]
#[ignore = "Regenerate native PAGE_START comparison PDFs after reference acceptance"]
fn generate_native_page_start_proofs() -> Result<(), String> {
    for stem in ["basic", "album"] {
        let root = fixture();
        let txt = root.join(format!("{stem}.txt"));
        let raw = root.join(format!("{stem}-raw.stampalbum"));
        assert_eq!(
            fs::read(&txt).map_err(|e| e.to_string())?,
            fs::read(&raw).map_err(|e| e.to_string())?
        );
        albumeasy::render_file(&txt, &root.join(format!("{stem}-txt-native.pdf")))?;
        albumeasy::render_file(&raw, &root.join(format!("{stem}-raw-native.pdf")))?;
        NativeAlbum::load(&root.join(format!("{stem}.stampalbum")))?
            .write_pdf(&root.join(format!("{stem}-native.pdf")))?;
    }
    NativeAlbum::load(&fixture().join("blank-pages.stampalbum"))?
        .write_pdf(&fixture().join("blank-pages-native.pdf"))?;
    Ok(())
}
