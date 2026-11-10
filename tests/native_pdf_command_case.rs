use oxidize_pdf::parser::PdfDocument;
use stamp_album_gen2::native_pdf::{album::NativeAlbum, albumeasy};
use std::{fs, path::Path};

fn change_keywords(source: &str, mixed: bool) -> String {
    source
        .lines()
        .map(|line| {
            if line.trim_start().starts_with('#') {
                return line.to_string();
            }
            let end = line.find('(').or_else(|| {
                (!line.trim().is_empty()).then(|| line.find(' ').unwrap_or(line.len()))
            });
            match end {
                Some(end) => {
                    let keyword = if mixed {
                        line[..end]
                            .chars()
                            .enumerate()
                            .map(|(i, c)| {
                                if i % 2 == 0 {
                                    c.to_ascii_lowercase()
                                } else {
                                    c.to_ascii_uppercase()
                                }
                            })
                            .collect::<String>()
                    } else {
                        line[..end].to_ascii_lowercase()
                    };
                    format!("{keyword}{}", &line[end..])
                }
                None => line.to_string(),
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn streams(path: &Path) -> Result<Vec<Vec<Vec<u8>>>, String> {
    let pdf = PdfDocument::open(path).map_err(|e| e.to_string())?;
    (0..pdf.page_count().map_err(|e| e.to_string())?)
        .map(|i| {
            let page = pdf.get_page(i).map_err(|e| e.to_string())?;
            pdf.get_page_content_streams(&page)
                .map_err(|e| e.to_string())
        })
        .collect()
}

#[test]
fn txt_and_raw_keywords_accept_lower_and_mixed_case_without_changing_values() -> Result<(), String>
{
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy");
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    for family in ["page-start-var-v1", "stamp-image-controls-v1"] {
        let assets = root.join(family);
        let source = fs::read_to_string(assets.join("album.txt")).map_err(|e| e.to_string())?;
        let expected = temp.path().join("expected.pdf");
        albumeasy::render(&source, &assets, &expected)?;
        let expected = streams(&expected)?;
        for mixed in [false, true] {
            let changed = change_keywords(&source, mixed);
            let (retained, unsupported) = albumeasy::commented_test_source(&changed)?;
            assert!(unsupported.is_empty());
            assert_eq!(retained.trim_end(), changed.trim_end());
            let output = temp.path().join("case.pdf");
            albumeasy::render(&changed, &assets, &output)?;
            assert_eq!(streams(&output)?, expected, "{family}, mixed={mixed}");
        }
    }
    let (_, unsupported) = albumeasy::commented_test_source("unknown_command (\"Keep My Text\")")?;
    assert_eq!(unsupported.len(), 1);
    assert_eq!(unsupported[0].name, "UNKNOWN_COMMAND");
    assert_eq!(unsupported[0].source, "unknown_command (\"Keep My Text\")");
    Ok(())
}

#[test]
fn native_keywords_and_header_accept_lower_and_mixed_case_without_changing_values(
) -> Result<(), String> {
    let assets =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/page-start-var-v1");
    let source = fs::read_to_string(assets.join("album.stampalbum")).map_err(|e| e.to_string())?;
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("native.pdf");
    NativeAlbum::parse(&source, &assets)?.write_pdf(&output)?;
    let expected = streams(&output)?;
    for mixed in [false, true] {
        NativeAlbum::parse(&change_keywords(&source, mixed), &assets)?.write_pdf(&output)?;
        assert_eq!(streams(&output)?, expected);
    }
    for invalid in ["stampalbum 2", "stampalbum 1 extra"] {
        assert!(NativeAlbum::parse(&source.replace("STAMPALBUM 1", invalid), &assets).is_err());
    }
    for invalid in ["stamp-image", "stamp_image2", "stämp_image"] {
        assert!(NativeAlbum::parse(&source.replace("STAMP_IMAGE", invalid), &assets).is_err());
    }
    Ok(())
}
