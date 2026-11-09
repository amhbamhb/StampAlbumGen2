use oxidize_pdf::parser::PdfDocument;
use stamp_album_gen2::native_pdf::{
    album::NativeAlbum,
    albumeasy,
    config::{NativeFooterNumberingConfig, NativePageNumberingScope, NativePdfConfig},
};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/albumeasy/footer-numbering-import-v1")
}

fn section_config(start: i32) -> Result<NativePdfConfig> {
    let source = include_str!("../config_template.toml")
        .replace(
            "[native_pdf.footer_number]\nenabled = false",
            "[native_pdf.footer_number]\nenabled = true",
        )
        .replace("scope = \"document\"", "scope = \"section\"")
        .replace("section_start = 1", &format!("section_start = {start}"));
    Ok(NativePdfConfig::from_toml(&source)?)
}

fn page_contents(path: &Path) -> Result<Vec<String>> {
    let pdf = PdfDocument::open(path)?;
    let mut pages = Vec::new();
    for index in 0..pdf.page_count()? {
        let page = pdf.get_page(index)?;
        pages.push(
            pdf.get_page_content_streams(&page)?
                .iter()
                .map(|stream| String::from_utf8_lossy(stream).into_owned())
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }
    Ok(pages)
}

fn has_label(page: &str, label: &str) -> bool {
    page.contains(&format!("({label}) Tj"))
}

#[test]
fn templates_supply_document_default_and_local_start() -> Result<()> {
    for source in [
        include_str!("../config_template.toml"),
        include_str!("../config_full_example.toml"),
    ] {
        let config = NativePdfConfig::from_toml(source)?;
        assert_eq!(
            config.footer_numbering,
            Some(NativeFooterNumberingConfig {
                enabled: false,
                scope: NativePageNumberingScope::Document,
                section_start: Some(1),
            })
        );
    }
    let config = section_config(10)?;
    assert_eq!(
        config.footer_numbering,
        Some(NativeFooterNumberingConfig {
            enabled: true,
            scope: NativePageNumberingScope::Section,
            section_start: Some(10),
        })
    );
    Ok(())
}

#[test]
fn section_scope_requires_a_start_and_supported_scope() -> Result<()> {
    let template = include_str!("../config_template.toml");
    for invalid in [
        template.replace("scope = \"document\"", "scope = \"year\""),
        template
            .replace("scope = \"document\"", "scope = \"section\"")
            .replace("section_start = 1\n", ""),
        template.replace("section_start = 1", "section_start = 2147483648"),
        template.replace("scope = \"document\"", "scope = \"section\""),
        template
            .replace("scope = \"document\"", "scope = \"section\"")
            .replace(
                "[native_pdf.footer_number]\nenabled = false",
                "[native_pdf.footer_number]\nenabled = true",
            )
            .replace(
                "album_pages_footer_number_enabled = true",
                "album_pages_footer_number_enabled = false",
            ),
    ] {
        assert!(NativePdfConfig::from_toml(&invalid).is_err());
    }
    Ok(())
}

#[test]
fn section_markers_require_enabled_native_section_scope() -> Result<()> {
    let source = fs::read_to_string(fixture().join("normal.stampalbum"))?;
    let first_stamp =
        "STAMP_IMAGE (\"../stamp-pagination-v1/us-150.png\" 52 29 0.05078125 \"HN\" 8 \"DB 150 / SG 1125a\\n52 x 29 mm\\nFirst-page wide design\")\n";
    let marked = source.replacen(
        first_stamp,
        &format!("{first_stamp}NATIVE_PAGE_NUMBER_SECTION ()\n"),
        1,
    );
    let album = NativeAlbum::parse(&marked, &fixture())?;
    let directory = tempfile::tempdir()?;
    let output = directory.path().join("numbering.pdf");
    assert!(album.write_pdf(&output).is_err());
    let document_config = include_str!("../config_template.toml").replace(
        "[native_pdf.footer_number]\nenabled = false",
        "[native_pdf.footer_number]\nenabled = true",
    );
    assert!(album
        .write_pdf_with_config(&NativePdfConfig::from_toml(&document_config)?, &output)
        .is_err());
    Ok(())
}

#[test]
fn explicit_sections_reset_on_next_page_and_keep_document_count_tokens() -> Result<()> {
    let base = fs::read_to_string(fixture().join("normal.stampalbum"))?
        .replace(
            "ALBUM_PAGES_SIZE (215.9 279.4)",
            "ALBUM_PAGES_SIZE (180 140)",
        )
        .replace(
            "ALBUM_PAGES_MARGINS (15 15 15 15)",
            "ALBUM_PAGES_MARGINS (5 5 5 5)",
        )
        .replace("ALBUM_PAGES_SPACING (4 14)", "ALBUM_PAGES_SPACING (4 8)")
        .replace(
            "\" of $PAGES$\"",
            "\" of $PAGES$; s$SECTION_PAGE$/$SECTION_PAGES$; d$DOCUMENT_PAGES$\"",
        );
    let first_stamp =
        "STAMP_IMAGE (\"../stamp-pagination-v1/us-150.png\" 52 29 0.05078125 \"HN\" 8 \"DB 150 / SG 1125a\\n52 x 29 mm\\nFirst-page wide design\")\n";
    let directory = tempfile::tempdir()?;
    let mut page_counts = Vec::new();
    for height in [140, 220] {
        let mut height_page_count = None;
        for start in [1, 10] {
            let mut source = base.replace(
                "ALBUM_PAGES_SIZE (180 140)",
                &format!("ALBUM_PAGES_SIZE (180 {height})"),
            );
            assert!(source.contains(first_stamp));
            source = source.replacen(
                first_stamp,
                &format!("{first_stamp}NATIVE_PAGE_NUMBER_SECTION ()\n"),
                1,
            );
            let album = NativeAlbum::parse(&source, &fixture())?;
            let output = directory
                .path()
                .join(format!("section-{height}-start-{start}.pdf"));
            let pages = album.write_pdf_with_config(&section_config(start)?, &output)?;
            let contents = page_contents(&output)?;
            assert_eq!(pages.len(), contents.len());
            assert!(pages.len() >= 2);
            assert!(
                contents[0].contains("(DB 151 / SG 1137a) Tj"),
                "the first page contains content from both sections"
            );
            assert!(
                has_label(
                    &contents[0],
                    &format!("Page {start} of {}; s1/1; d{}", pages.len(), pages.len())
                ),
                "the page containing the marker stays with its preceding section"
            );
            assert!(
                has_label(
                    &contents[1],
                    &format!(
                        "Page {start} of {}; s1/{}; d{}",
                        pages.len(),
                        pages.len() - 1,
                        pages.len()
                    )
                ),
                "the next page starts section-local numbering at the configured value"
            );
            assert!(
                has_label(
                    contents.last().ok_or("last page")?,
                    &format!(
                        "Page {} of {}; s{}/{}; d{}",
                        start + pages.len() as i32 - 2,
                        pages.len(),
                        pages.len() - 1,
                        pages.len() - 1,
                        pages.len()
                    )
                ),
                "local ordinal, section count, and document count remain distinct"
            );
            height_page_count = Some(pages.len());
        }
        page_counts.push(height_page_count.ok_or("missing pagination result")?);
    }
    assert_ne!(
        page_counts[0], page_counts[1],
        "the page size must change pagination"
    );
    Ok(())
}

#[test]
fn imported_albumeasy_keeps_unrecognized_section_tokens_literal() -> Result<()> {
    let source = fs::read_to_string(fixture().join("normal.txt"))?
        .replace("\" of $PAGES$\"", "\" of $PAGES$; $SECTION_PAGES$\"");
    let directory = tempfile::tempdir()?;
    let txt_output = directory.path().join("section-token.txt.pdf");
    let raw_output = directory.path().join("section-token.raw.pdf");
    assert_eq!(albumeasy::render(&source, &fixture(), &txt_output)?, 2);
    let raw_source = fs::read_to_string(fixture().join("normal-raw.stampalbum"))?
        .replace("\" of $PAGES$\"", "\" of $PAGES$; $SECTION_PAGES$\"");
    assert_eq!(albumeasy::render(&raw_source, &fixture(), &raw_output)?, 2);
    for output in [&txt_output, &raw_output] {
        let contents = page_contents(output)?;
        assert!(contents.iter().all(|page| page.contains("$SECTION_PAGES$")));
    }
    assert_eq!(page_contents(&txt_output)?, page_contents(&raw_output)?);
    Ok(())
}
