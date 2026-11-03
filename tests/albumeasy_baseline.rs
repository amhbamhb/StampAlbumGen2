use oxidize_pdf::parser::PdfDocument;
use roxmltree::{Document, Node};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const PDF_SHA256: &str = "b186888681074cfa9b2c6be8ad39a17a6198c8824403a57f4d9f7a9a13a71081";
const MM_PER_POINT: f64 = 25.4 / 72.0;

#[derive(Deserialize)]
struct Expectations {
    input_sha256: String,
    pages: usize,
    page_size_mm: [f64; 2],
    geometry_tolerance_mm: f64,
    frames_by_page: Vec<Vec<[f64; 4]>>,
    required_text_by_page: Vec<Vec<String>>,
}

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/core-v1")
}

fn expectations() -> Result<Expectations> {
    Ok(serde_json::from_slice(&fs::read(
        fixture().join("expected.json"),
    )?)?)
}

fn check_hash(bytes: &[u8], expected: &str, label: &str) -> Result<()> {
    let actual: String = Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    if actual != expected {
        return Err(format!("{label} checksum drift: expected {expected}, got {actual}").into());
    }
    Ok(())
}

fn near(actual: f64, expected: f64, tolerance: f64, label: &str) -> Result<()> {
    if !actual.is_finite() || (actual - expected).abs() > tolerance {
        return Err(
            format!("{label}: expected {expected}, got {actual}; tolerance {tolerance}").into(),
        );
    }
    Ok(())
}

fn numbers(node: Node<'_, '_>, name: &str) -> Result<Vec<f64>> {
    let value = node
        .attribute(name)
        .ok_or_else(|| format!("missing {} attribute {name}", node.tag_name().name()))?;
    Ok(value
        .split_whitespace()
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()?)
}

fn coordinate(node: Node<'_, '_>, name: &str) -> Result<f64> {
    let values = numbers(node, name)?;
    match values.as_slice() {
        [value] if value.is_finite() => Ok(*value),
        _ => Err(format!("invalid coordinate {name}").into()),
    }
}

fn check_trace(trace: &str, expected: &Expectations) -> Result<()> {
    let document = Document::parse(trace)?;
    let pages: Vec<_> = document
        .descendants()
        .filter(|node| node.has_tag_name("page"))
        .collect();
    if pages.len() != expected.pages
        || expected.frames_by_page.len() != expected.pages
        || expected.required_text_by_page.len() != expected.pages
    {
        return Err(format!(
            "page count mismatch: expected {}, got {}",
            expected.pages,
            pages.len()
        )
        .into());
    }
    for (index, page) in pages.iter().enumerate() {
        let box_values = numbers(*page, "mediabox")?;
        let [left, bottom, right, top] = box_values.as_slice() else {
            return Err("invalid mediabox".into());
        };
        near(*left, 0.0, 0.00001, "mediabox origin x")?;
        near(*bottom, 0.0, 0.00001, "mediabox origin y")?;
        for (actual, wanted) in [right - left, top - bottom]
            .iter()
            .zip(expected.page_size_mm)
        {
            near(
                actual * MM_PER_POINT,
                wanted,
                expected.geometry_tolerance_mm,
                "page size mm",
            )?;
        }

        let mut lines = Vec::new();
        for path in page
            .children()
            .filter(|node| node.has_tag_name("stroke_path"))
        {
            let points: Vec<_> = path.children().filter(Node::is_element).collect();
            if points.len() != 2
                || !points[0].has_tag_name("moveto")
                || !points[1].has_tag_name("lineto")
            {
                continue;
            }
            let transform = numbers(path, "transform")?;
            if transform.as_slice() != [1.0, 0.0, 0.0, -1.0, 0.0, *top] {
                return Err(
                    "unsupported core-v1 stroke transform; not a general native PDF checker".into(),
                );
            }
            lines.push([
                coordinate(points[0], "x")?,
                coordinate(points[0], "y")?,
                coordinate(points[1], "x")?,
                coordinate(points[1], "y")?,
            ]);
        }
        // AlbumEasy emits each frame as vertical, vertical, horizontal, horizontal.
        // Use centrelines rather than the deliberately extended corner endpoints.
        let mut frames = Vec::new();
        let (groups, remainder) = lines.as_chunks::<4>();
        for [a, b, c, d] in groups {
            if a[0] != a[2] || b[0] != b[2] || c[1] != c[3] || d[1] != d[3] {
                return Err("unsupported core-v1 frame segment order".into());
            }
            frames.push([
                a[0].min(b[0]) * MM_PER_POINT,
                c[1].min(d[1]) * MM_PER_POINT,
                (a[0] - b[0]).abs() * MM_PER_POINT,
                (c[1] - d[1]).abs() * MM_PER_POINT,
            ]);
        }
        let valid_rule = index == 1 && remainder.len() == 1 && remainder[0][1] == remainder[0][3];
        if (!remainder.is_empty() && !valid_rule)
            || frames.len() != expected.frames_by_page[index].len()
        {
            return Err(format!("page {} frame count/remaining rule mismatch", index + 1).into());
        }
        for (frame_index, (actual, wanted)) in frames
            .iter()
            .zip(&expected.frames_by_page[index])
            .enumerate()
        {
            for (field, (actual, wanted)) in actual.iter().zip(wanted).enumerate() {
                near(
                    *actual,
                    *wanted,
                    expected.geometry_tolerance_mm,
                    &format!(
                        "page {} frame {} field {field} mm",
                        index + 1,
                        frame_index + 1
                    ),
                )?;
            }
        }
    }
    Ok(())
}

fn check_text(text: &str, required: &[String], page: usize) -> Result<()> {
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    for item in required {
        if !normalized.contains(&item.split_whitespace().collect::<Vec<_>>().join(" ")) {
            return Err(format!("page {page} missing text: {item}").into());
        }
    }
    Ok(())
}

fn tool_output(program: &str, args: &[&OsStr]) -> Result<String> {
    let output = Command::new(program).args(args).output().map_err(|error| {
        format!("cannot run {program} {args:?}: {error}; install tool and put it on PATH")
    })?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() {
        return Err(format!("{program} {args:?} failed ({}): {stderr}", output.status).into());
    }
    if !stderr.is_empty() {
        eprintln!("{program} diagnostics: {stderr}");
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn inspect_text(pdf: &Path, expected: &Expectations) -> Result<()> {
    let document = PdfDocument::open(pdf)
        .map_err(|error| format!("oxidize-pdf cannot open {}: {error}", pdf.display()))?;
    let pages = document
        .extract_text()
        .map_err(|error| format!("oxidize-pdf cannot extract {}: {error}", pdf.display()))?;
    if pages.len() != expected.pages || pages.len() != expected.required_text_by_page.len() {
        return Err(format!(
            "oxidize-pdf text page count mismatch for {}: expected {}, got {}",
            pdf.display(),
            expected.pages,
            pages.len()
        )
        .into());
    }
    for (index, (page, required)) in pages
        .iter()
        .zip(&expected.required_text_by_page)
        .enumerate()
    {
        if page.truncated {
            return Err(
                format!("oxidize-pdf truncated {} page {}", pdf.display(), index + 1).into(),
            );
        }
        check_text(&page.text, required, index + 1)?;
    }
    Ok(())
}

fn inspect(pdf: &Path, expected: &Expectations) -> Result<String> {
    let trace = tool_output(
        "mutool",
        &[
            OsStr::new("draw"),
            OsStr::new("-q"),
            OsStr::new("-F"),
            OsStr::new("trace"),
            pdf.as_os_str(),
        ],
    )?;
    check_trace(&trace, expected)?;
    inspect_text(pdf, expected)?;
    eprintln!(
        "PASS {}: {} pages, 11 frames, 20 required texts",
        pdf.display(),
        expected.pages
    );
    Ok(trace)
}

fn page_traces(trace: &str) -> Result<Vec<String>> {
    let document = Document::parse(trace)?;
    Ok(document
        .descendants()
        .filter(|node| node.has_tag_name("page"))
        .map(|node| trace[node.range()].to_owned())
        .collect())
}

fn check_strict_trace(reference: &str, actual: &str) -> Result<()> {
    if page_traces(reference)? != page_traces(actual)? {
        return Err(
            "strict AlbumEasy vector/text drift (excluding filename and PDF metadata)".into(),
        );
    }
    Ok(())
}

fn check_frozen() -> Result<()> {
    let expected = expectations()?;
    let source = fs::read(fixture().join("album.txt"))?;
    check_hash(&source, &expected.input_sha256, "album.txt")?;
    check_hash(
        &fs::read(fixture().join("reference.pdf"))?,
        PDF_SHA256,
        "reference.pdf",
    )?;
    Ok(())
}

#[test]
fn frozen_identities_and_hash_drift() -> Result<()> {
    check_frozen()?;
    let expected = expectations()?;
    let mut source = fs::read(fixture().join("album.txt"))?;
    source.push(b' ');
    assert!(check_hash(&source, &expected.input_sha256, "modified input").is_err());
    Ok(())
}

#[test]
fn oxidize_reference_text_and_negative_controls() -> Result<()> {
    check_frozen()?;
    let pdf = fixture().join("reference.pdf");
    inspect_text(&pdf, &expectations()?)?;
    let mut missing = expectations()?;
    missing.required_text_by_page[0].push("THIS TEXT IS NOT IN CORE-V1".to_owned());
    assert!(inspect_text(&pdf, &missing).is_err());
    let mut wrong_pages = expectations()?;
    wrong_pages.pages += 1;
    assert!(inspect_text(&pdf, &wrong_pages).is_err());
    let directory = tempfile::tempdir()?;
    let invalid = directory.path().join("invalid.pdf");
    fs::write(&invalid, b"not a PDF")?;
    assert!(inspect_text(&invalid, &expectations()?).is_err());
    Ok(())
}

#[test]
fn tolerance_and_missing_text_are_rejected() -> Result<()> {
    near(25.009, 25.0, 0.01, "inside tolerance")?;
    assert!(near(25.011, 25.0, 0.01, "dimension drift").is_err());
    assert!(near(f64::NAN, 25.0, 0.01, "invalid measurement").is_err());
    let required = vec!["Page 1 of 2".to_owned()];
    check_text("Page  1\nof 2", &required, 1)?;
    assert!(check_text("Page 1 of 3", &required, 1).is_err());
    Ok(())
}

#[test]
fn strict_trace_detects_styles_but_ignores_filename() -> Result<()> {
    let reference = r#"<document filename="reference.pdf"><page number="1"><stroke_path linewidth="1"/></page></document>"#;
    check_strict_trace(
        reference,
        &reference.replace("reference.pdf", "candidate.pdf"),
    )?;
    assert!(check_strict_trace(
        reference,
        &reference.replace("linewidth=\"1\"", "linewidth=\"2\"")
    )
    .is_err());
    Ok(())
}

#[test]
#[ignore = "requires mutool on PATH; run explicitly before accepting baseline changes"]
fn reference_geometry_text_and_negative_controls() -> Result<()> {
    check_frozen()?;
    let expected = expectations()?;
    let trace = inspect(&fixture().join("reference.pdf"), &expected)?;
    let mut drifted = expectations()?;
    drifted.frames_by_page[0][0][0] += 0.02;
    assert!(
        check_trace(&trace, &drifted).is_err(),
        "placement drift must fail"
    );
    drifted = expectations()?;
    drifted.frames_by_page[0][0][2] += 0.02;
    assert!(
        check_trace(&trace, &drifted).is_err(),
        "size drift must fail"
    );
    drifted = expectations()?;
    drifted.pages = 3;
    assert!(
        check_trace(&trace, &drifted).is_err(),
        "page count drift must fail"
    );
    drifted = expectations()?;
    drifted.page_size_mm[0] += 0.02;
    assert!(
        check_trace(&trace, &drifted).is_err(),
        "page size drift must fail"
    );
    assert!(check_trace("<broken", &expected).is_err());
    Ok(())
}

#[test]
#[ignore = "set ALBUMEASY_BASELINE_CANDIDATE to a fresh PDF rendered from the frozen album.txt"]
fn candidate_matches_reference_strictly() -> Result<()> {
    check_frozen()?;
    let candidate = std::env::var_os("ALBUMEASY_BASELINE_CANDIDATE")
        .ok_or("ALBUMEASY_BASELINE_CANDIDATE is required; no silent skip")?;
    let expected = expectations()?;
    let reference = inspect(&fixture().join("reference.pdf"), &expected)?;
    let actual = inspect(Path::new(&candidate), &expected)?;
    check_strict_trace(&reference, &actual)
}
