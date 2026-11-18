use oxidize_pdf::parser::PdfDocument;
use stamp_album_gen2::native_pdf::{album::NativeAlbum, albumeasy};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/album-author-v1")
}

fn extract_pdf_field(content: &str, field: &str) -> Option<String> {
    if let Some(pos) = content.find(field) {
        let rest = &content[pos + field.len()..];
        let rest = rest.trim_start();
        if rest.starts_with('(') {
            if let Some(end) = rest.find(')') {
                return Some(rest[1..end].to_string());
            }
        } else if rest.starts_with('<') {
            if let Some(end) = rest.find('>') {
                let hex = &rest[1..end];
                return decode_hex_string(hex);
            }
        }
    }
    None
}

fn decode_hex_string(hex: &str) -> Option<String> {
    let hex = hex.trim();
    if hex.starts_with("FEFF") || hex.starts_with("feff") {
        let bytes = (4..hex.len())
            .step_by(2)
            .filter_map(|i| {
                if i + 2 <= hex.len() {
                    u8::from_str_radix(&hex[i..i + 2], 16).ok()
                } else {
                    None
                }
            })
            .collect::<Vec<u8>>();
        let u16s = bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
            .collect::<Vec<u16>>();
        String::from_utf16(&u16s).ok()
    } else {
        let bytes = (0..hex.len())
            .step_by(2)
            .filter_map(|i| {
                if i + 2 <= hex.len() {
                    u8::from_str_radix(&hex[i..i + 2], 16).ok()
                } else {
                    None
                }
            })
            .collect::<Vec<u8>>();
        String::from_utf8(bytes).ok()
    }
}

fn read_pdf_metadata(path: &Path) -> Result<(Option<String>, Option<String>), String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let content = String::from_utf8_lossy(&bytes);
    Ok((
        extract_pdf_field(&content, "/Author"),
        extract_pdf_field(&content, "/Title"),
    ))
}

#[test]
fn albumeasy_reference_pdfs_contain_expected_author_and_title() -> Result<(), String> {
    for stem in ["basic", "album"] {
        let pdf = fixture().join(format!("{stem}.pdf"));
        let (author, title) = read_pdf_metadata(&pdf)?;
        assert_eq!(
            author.as_deref(),
            Some("John Doe"),
            "{stem}.pdf author mismatch"
        );
        assert_eq!(
            title.as_deref(),
            Some("Liberty Issue"),
            "{stem}.pdf title mismatch"
        );
    }
    Ok(())
}

#[test]
fn txt_and_raw_documents_render_with_author_and_title_metadata() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let assets = fixture();
    for stem in ["basic", "album"] {
        let txt = assets.join(format!("{stem}.txt"));
        let output = temp.path().join(format!("{stem}-native.pdf"));
        let pages = albumeasy::render_file(&txt, &output)?;
        assert!(pages >= 1);

        let pdf = PdfDocument::open(&output).map_err(|e| e.to_string())?;
        assert_eq!(pdf.page_count().map_err(|e| e.to_string())? as usize, pages);

        let (author, title) = read_pdf_metadata(&output)?;
        assert_eq!(
            author.as_deref(),
            Some("John Doe"),
            "Rendered {stem} author mismatch"
        );
        assert_eq!(
            title.as_deref(),
            Some("Liberty Issue"),
            "Rendered {stem} title mismatch"
        );
    }
    Ok(())
}

#[test]
fn native_album_parses_and_renders_author_and_title() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let assets = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/albumeasy/native-real-stamps-v1");
    let original = fs::read_to_string(assets.join("album.stampalbum")).map_err(|e| e.to_string())?;

    // Default metadata when omitted
    let output_default = temp.path().join("default.pdf");
    let album = NativeAlbum::parse(&original, &assets)?;
    album.write_pdf(&output_default)?;
    let (author, title) = read_pdf_metadata(&output_default)?;
    assert_eq!(author.as_deref(), Some("Stamp Album Generator 2"));
    assert_eq!(title.as_deref(), Some("Stamp Album"));

    // Explicit ALBUM_AUTHOR and ALBUM_TITLE
    let with_metadata = original.replacen(
        "ALBUM_PAGES_SIZE",
        "ALBUM_AUTHOR (\"Custom Author\")\nALBUM_TITLE (\"Custom Title\")\nALBUM_PAGES_SIZE",
        1,
    );
    let output_custom = temp.path().join("custom.pdf");
    let album = NativeAlbum::parse(&with_metadata, &assets)?;
    album.write_pdf(&output_custom)?;
    let (author, title) = read_pdf_metadata(&output_custom)?;
    assert_eq!(author.as_deref(), Some("Custom Author"));
    assert_eq!(title.as_deref(), Some("Custom Title"));

    // Case tolerance for native commands
    for (author_kw, title_kw) in [
        ("album_author", "album_title"),
        ("Album_Author", "Album_Title"),
    ] {
        let variant = original.replacen(
            "ALBUM_PAGES_SIZE",
            &format!("{author_kw} (\"Case Author\")\n{title_kw} (\"Case Title\")\nALBUM_PAGES_SIZE"),
            1,
        );
        let output_case = temp.path().join("case.pdf");
        let album = NativeAlbum::parse(&variant, &assets)?;
        album.write_pdf(&output_case)?;
        let (author, title) = read_pdf_metadata(&output_case)?;
        assert_eq!(author.as_deref(), Some("Case Author"));
        assert_eq!(title.as_deref(), Some("Case Title"));
    }

    Ok(())
}

#[test]
fn album_author_and_title_validation_errors() -> Result<(), String> {
    let assets = fixture();
    let original = fs::read_to_string(assets.join("basic.txt")).map_err(|e| e.to_string())?;
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("error.pdf");

    // Missing arguments
    for cmd in ["ALBUM_AUTHOR", "ALBUM_TITLE"] {
        let invalid = original.replace(&format!("{cmd} ("), &format!("{cmd} () #"));
        assert!(
            albumeasy::render(&invalid, &assets, &output).is_err(),
            "{cmd} missing args must fail"
        );
    }

    // Excess arguments
    for cmd in ["ALBUM_AUTHOR", "ALBUM_TITLE"] {
        let invalid = original.replace(
            &format!("{cmd} ("),
            &format!("{cmd} (\"First\" \"Second\") #"),
        );
        assert!(
            albumeasy::render(&invalid, &assets, &output).is_err(),
            "{cmd} excess args must fail"
        );
    }

    // Unquoted argument
    for cmd in ["ALBUM_AUTHOR", "ALBUM_TITLE"] {
        let invalid = original.replace(&format!("{cmd} ("), &format!("{cmd} (UnquotedValue) #"));
        assert!(
            albumeasy::render(&invalid, &assets, &output).is_err(),
            "{cmd} unquoted arg must fail"
        );
    }

    // Duplicate in native album
    let native_assets = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/albumeasy/native-real-stamps-v1");
    let native_source = fs::read_to_string(native_assets.join("album.stampalbum"))
        .map_err(|e| e.to_string())?;
    let dup_author = native_source.replacen(
        "ALBUM_PAGES_SIZE",
        "ALBUM_AUTHOR (\"One\")\nALBUM_AUTHOR (\"Two\")\nALBUM_PAGES_SIZE",
        1,
    );
    assert!(NativeAlbum::parse(&dup_author, &native_assets).is_err());

    let dup_title = native_source.replacen(
        "ALBUM_PAGES_SIZE",
        "ALBUM_TITLE (\"One\")\nALBUM_TITLE (\"Two\")\nALBUM_PAGES_SIZE",
        1,
    );
    assert!(NativeAlbum::parse(&dup_title, &native_assets).is_err());

    Ok(())
}
