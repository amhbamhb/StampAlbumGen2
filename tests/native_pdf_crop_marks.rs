use sha2::{Digest, Sha256};
use stamp_album_gen2::native_pdf::{
    colour::NamedColour,
    commands::page::{album_pages_crop_marks::AlbumPagesCropMarks, album_pages_size::PageSize},
    config::NativePdfConfig,
    renderer::write_page_geometry_proof,
    units::points_to_mm,
};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const BASE: &str = r#"
color_album_border = "black"
[page]
album_page_width_mm = 100
album_page_height_mm = 150
page_left_margin_mm = 15
page_right_margin_mm = 12
page_top_margin_mm = 15
page_bottom_margin_mm = 15
album_pages_spacing_h = 3
album_pages_spacing_v = 4
page_border_outer = 0.1
page_border_inner1 = 0.5
page_border_inner2 = 0.1
page_border_spacing = 1
[native_pdf.proof]
page_count = 2
guide_enabled = false
guide_width_mm = 0.1
guide_colour = "black"
"#;

const CROP: &str = r#"
[native_pdf.crop_marks]
enabled = true
line_width_mm = 0.1
horizontal_padding_mm = 1
vertical_padding_mm = 1
arms_mm = 5
colour = "black"
"#;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/crop-marks-v1")
}

fn near(actual: f64, expected: f64) {
    assert!(
        actual.is_finite() && (actual - expected).abs() <= 0.01,
        "expected {expected}, got {actual}"
    );
}

fn config(crop: &str) -> Result<NativePdfConfig> {
    Ok(NativePdfConfig::from_toml(&format!("{BASE}{crop}"))?)
}

#[test]
fn explicit_cross_geometry_clipping_and_zero_hairlines() -> Result<()> {
    let size = PageSize::parse(&[100.0, 150.0])?;
    let colour = NamedColour::parse("black")?;
    let layout = AlbumPagesCropMarks::parse(&[0.25, 2.0, 4.0, 5.0], colour)?.layout(size)?;
    assert!(layout.clipped);
    near(layout.line_width_mm, 0.25);
    assert_eq!(layout.marks.len(), 4);
    for (mark, centre) in layout.marks.iter().zip([
        [1.875, 3.875],
        [1.875, 146.125],
        [98.125, 146.125],
        [98.125, 3.875],
    ]) {
        assert_eq!(mark.points.len(), 4);
        for (point, expected) in mark.points.iter().zip([
            [centre[0] - 5.0, centre[1]],
            [centre[0] + 5.0, centre[1]],
            [centre[0], centre[1] + 5.0],
            [centre[0], centre[1] - 5.0],
        ]) {
            near(point[0], expected[0]);
            near(point[1], expected[1]);
        }
    }
    let safe = AlbumPagesCropMarks::parse(&[0.2, 8.0, 9.0, 2.5], colour)?.layout(size)?;
    assert!(!safe.clipped);
    // Butt caps: touching the page edge is not clipping.
    assert!(
        !AlbumPagesCropMarks::parse(&[0.2, 5.1, 5.1, 5.0], colour)?
            .layout(size)?
            .clipped
    );
    assert!(
        AlbumPagesCropMarks::parse(&[0.2, 5.09, 5.1, 5.0], colour)?
            .layout(size)?
            .clipped
    );
    let zero = AlbumPagesCropMarks::parse(&[0.0; 4], colour)?.layout(size)?;
    near(zero.line_width_mm, 0.0); // PDF hairline, not a disabled command.
    assert!(!zero.clipped);
    assert_eq!(zero.marks[0].points, [[0.0, 0.0]; 4]);
    assert!(
        !AlbumPagesCropMarks::parse(&[0.2, 0.0, 0.0, 0.0], colour)?
            .layout(size)?
            .clipped
    );
    Ok(())
}

#[test]
fn arguments_are_explicit_finite_nonnegative_and_pdf_safe() -> Result<()> {
    let colour = NamedColour::parse("black")?;
    for count in [0, 1, 2, 3, 5] {
        assert!(AlbumPagesCropMarks::parse(&vec![1.0; count], colour).is_err());
    }
    for index in 0..4 {
        for invalid in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY, f64::MAX] {
            let mut values = [1.0; 4];
            values[index] = invalid;
            assert!(AlbumPagesCropMarks::parse(&values, colour).is_err());
        }
    }
    // Individually finite point values may overflow after page-size addition.
    let huge = AlbumPagesCropMarks::parse(&[1.0e307, 0.0, 0.0, 0.0], colour)?;
    assert!(huge.layout(PageSize::parse(&[6.0e307, 100.0])?).is_err());
    Ok(())
}

#[test]
fn marks_do_not_resize_shift_or_reserve_content_and_follow_physical_edges() -> Result<()> {
    let plain = config("")?;
    let crop = config(CROP)?;
    assert!(crop.unapplied_keys.is_empty());
    for page in [1, 2] {
        let before = plain.page_settings.resolve(page)?;
        let after = crop.page_settings.resolve(page)?;
        near(after.size.width_mm(), before.size.width_mm());
        near(after.size.height_mm(), before.size.height_mm());
        for (a, b) in [
            (after.margin_bounds, before.margin_bounds),
            (after.content_bounds, before.content_bounds),
        ] {
            for (x, y) in [a.left, a.bottom, a.right, a.top]
                .into_iter()
                .zip([b.left, b.bottom, b.right, b.top])
            {
                near(x, y);
            }
        }
        assert!(before.crop_marks.is_none());
        assert!(after.crop_marks.ok_or("marks")?.clipped);
    }
    let even = config(&format!(
        "{CROP}\n[page.even_margins]\nleft_mm=12\nright_mm=15\ntop_mm=16\nbottom_mm=17\n"
    ))?;
    let first = even.page_settings.resolve(1)?.crop_marks.ok_or("marks")?;
    let second = even.page_settings.resolve(2)?.crop_marks.ok_or("marks")?;
    for (a, b) in first.marks.iter().zip(&second.marks) {
        assert_eq!(a.points, b.points);
    }
    Ok(())
}

#[test]
fn missing_invalid_disabled_keys_redaction_and_unknown_keys() -> Result<()> {
    for key in [
        "enabled",
        "line_width_mm",
        "horizontal_padding_mm",
        "vertical_padding_mm",
        "arms_mm",
        "colour",
    ] {
        let source = CROP
            .lines()
            .filter(|line| !line.starts_with(key))
            .collect::<Vec<_>>()
            .join("\n");
        let error = config(&source)
            .err()
            .ok_or("missing key accepted")?
            .to_string();
        assert!(
            error.contains(&format!("native_pdf.crop_marks.{key}")),
            "{error}"
        );
    }
    for (key, value) in [
        ("enabled", "\"secret\""),
        ("line_width_mm", "-1"),
        ("horizontal_padding_mm", "nan"),
        ("vertical_padding_mm", "inf"),
        ("arms_mm", "-0.1"),
        ("colour", "\"secret\""),
    ] {
        let mut table: toml::Table = toml::from_str(CROP)?;
        let value: toml::Table = toml::from_str(&format!("value = {value}"))?;
        table["native_pdf"]["crop_marks"][key] = value["value"].clone();
        table["native_pdf"]["crop_marks"]["enabled"] = if key == "enabled" {
            table["native_pdf"]["crop_marks"]["enabled"].clone()
        } else {
            toml::Value::Boolean(false)
        };
        let error = config(&toml::to_string(&table)?)
            .err()
            .ok_or("invalid accepted")?
            .to_string();
        assert!(
            error.contains(&format!("native_pdf.crop_marks.{key}")),
            "{error}"
        );
        assert!(!error.contains("secret"), "{error}");
    }
    assert!(
        config(&CROP.replace("enabled = true", "enabled = false"))?
            .page_settings
            .resolve(1)?
            .crop_marks
            .is_none()
    );
    assert_eq!(
        config(&format!("{CROP}\nfuture = 1"))?.unapplied_keys,
        ["native_pdf.crop_marks.future"]
    );
    assert!(
        config(&CROP.replace(
            "[native_pdf.crop_marks]",
            "[native_pdf]\ncrop_marks = 1\n[native_pdf.other]"
        ))
        .is_err()
    );
    Ok(())
}

#[test]
fn templates_are_explicit_disabled_and_snapshot_is_independent() -> Result<()> {
    for source in [
        include_str!("../config_template.toml"),
        include_str!("../config_full_example.toml"),
    ] {
        let mut table: toml::Table = toml::from_str(source)?;
        let crop = &table["native_pdf"]["crop_marks"];
        assert_eq!(crop["enabled"].as_bool(), Some(false));
        assert_eq!(crop["colour"].as_str(), Some("black"));
        for (key, value) in [
            ("line_width_mm", 0.1),
            ("horizontal_padding_mm", 1.0),
            ("vertical_padding_mm", 1.0),
            ("arms_mm", 5.0),
        ] {
            near(crop[key].as_float().ok_or("float")?, value);
        }
        let loaded = NativePdfConfig::from_toml(source)?;
        assert!(loaded.page_settings.resolve(1)?.crop_marks.is_none());
        assert!(
            !loaded
                .unapplied_keys
                .iter()
                .any(|key| key.starts_with("native_pdf.crop_marks"))
        );
        table["native_pdf"]["crop_marks"]["enabled"] = toml::Value::Boolean(true);
        assert!(
            NativePdfConfig::from_toml(&toml::to_string(&table)?)?
                .page_settings
                .resolve(1)?
                .crop_marks
                .is_some()
        );
        assert!(loaded.page_settings.resolve(1)?.crop_marks.is_none());
    }
    let temp = tempfile::tempdir()?;
    let path = temp.path().join("crop settings.toml");
    fs::write(&path, format!("{BASE}{CROP}"))?;
    let old = NativePdfConfig::load(&path)?;
    fs::write(
        &path,
        format!(
            "{BASE}{}",
            CROP.replace("arms_mm = 5", "arms_mm = 2")
                .replace("colour = \"black\"", "colour = \"pink\"")
        ),
    )?;
    let fresh = NativePdfConfig::load(&path)?;
    let old = old.page_settings.resolve(1)?.crop_marks.ok_or("marks")?;
    let fresh = fresh.page_settings.resolve(1)?.crop_marks.ok_or("marks")?;
    assert_ne!(old.marks[0].points, fresh.marks[0].points);
    assert_ne!(old.colour.rgb(), fresh.colour.rgb());
    Ok(())
}

#[test]
fn frozen_reference_hashes_and_media_sizes() -> Result<()> {
    let hashes = fs::read_to_string(root().join("sha256.txt"))?;
    assert_eq!(hashes.lines().count(), 18);
    for line in hashes.lines() {
        let (wanted, name) = line.split_once("  ").ok_or("hash entry")?;
        let actual: String = Sha256::digest(fs::read(root().join(name))?)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(actual, wanted, "{name}");
    }
    for (name, width, height, pages) in cases() {
        let pdf = oxidize_pdf::parser::PdfDocument::open(root().join(format!("{name}.pdf")))?;
        assert_eq!(pdf.page_count()?, u32::try_from(pages)?);
        for index in 0..u32::try_from(pages)? {
            let bounds = pdf.get_page(index)?.media_box;
            near(points_to_mm(bounds[2] - bounds[0]), width);
            near(points_to_mm(bounds[3] - bounds[1]), height);
        }
        assert!(pdf.extract_text()?[0].text.contains("CROP ANCHOR"));
    }
    Ok(())
}

#[test]
fn invalid_layout_creates_no_pdf_and_preserves_existing_output() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let new = temp.path().join("new.pdf");
    let old = temp.path().join("old.pdf");
    fs::write(&old, b"existing output")?;
    // Both values fit points separately, but their resolved sum does not.
    let source = BASE.replace("= 100\n", "= 6.0e307\n");
    let mut loaded = NativePdfConfig::from_toml(&source)?;
    loaded
        .page_settings
        .set_crop_marks(AlbumPagesCropMarks::parse(
            &[1.0e307, 0.0, 0.0, 0.0],
            NamedColour::parse("black")?,
        )?);
    for path in [&new, &old] {
        let error = write_page_geometry_proof(&loaded.page_settings, &loaded.proof, path)
            .expect_err("invalid crop coordinate accepted");
        assert!(
            error.contains("page 1") && error.contains("native_pdf.crop_marks"),
            "{error}"
        );
    }
    assert!(!new.exists());
    assert_eq!(fs::read(old)?, b"existing output");
    Ok(())
}

#[test]
fn crop_marks_leave_all_measured_text_and_reservations_unchanged() -> Result<()> {
    let mut table: toml::Table = toml::from_str(include_str!("../config_template.toml"))?;
    table["footer"]["album_pages_footer_number_enabled"] = toml::Value::Boolean(true);
    table["footer"]["album_pages_footer_number_position"] = toml::Value::String("R".into());
    for component in [
        "header",
        "footer",
        "header_number",
        "footer_number",
        "header_date",
        "footer_date",
        "title",
    ] {
        table["native_pdf"][component]["enabled"] = toml::Value::Boolean(true);
    }
    for component in ["header_date", "footer_date"] {
        table["native_pdf"][component]["date_source"] = toml::Value::String("fixed".into());
    }
    let plain = NativePdfConfig::from_toml(&toml::to_string(&table)?)?;
    table["native_pdf"]["crop_marks"]["enabled"] = toml::Value::Boolean(true);
    let crop = NativePdfConfig::from_toml(&toml::to_string(&table)?)?;
    for page in 1..=3 {
        let before = plain.page_settings.resolve_with_page_count(page, 3)?;
        let after = crop.page_settings.resolve_with_page_count(page, 3)?;
        for (x, y) in [
            before.content_bounds.left,
            before.content_bounds.bottom,
            before.content_bounds.right,
            before.content_bounds.top,
        ]
        .into_iter()
        .zip([
            after.content_bounds.left,
            after.content_bounds.bottom,
            after.content_bounds.right,
            after.content_bounds.top,
        ]) {
            near(x, y);
        }
        for (a, b) in [
            (
                before.header.as_ref().map(|x| &x.lines),
                after.header.as_ref().map(|x| &x.lines),
            ),
            (
                before.footer.as_ref().map(|x| &x.lines),
                after.footer.as_ref().map(|x| &x.lines),
            ),
            (
                before.header_date.as_ref().map(|x| &x.lines),
                after.header_date.as_ref().map(|x| &x.lines),
            ),
            (
                before.footer_date.as_ref().map(|x| &x.lines),
                after.footer_date.as_ref().map(|x| &x.lines),
            ),
            (
                before.header_number.as_ref().map(|x| &x.lines),
                after.header_number.as_ref().map(|x| &x.lines),
            ),
            (
                before.footer_number.as_ref().map(|x| &x.lines),
                after.footer_number.as_ref().map(|x| &x.lines),
            ),
            (
                before.title.as_ref().map(|x| &x.lines),
                after.title.as_ref().map(|x| &x.lines),
            ),
        ] {
            let a = a.ok_or("missing text")?;
            let b = b.ok_or("missing text")?;
            assert_eq!(a.len(), b.len());
            for (a, b) in a.iter().zip(b) {
                assert_eq!(a.text, b.text);
                near(a.left_mm, b.left_mm);
                near(a.baseline_mm, b.baseline_mm);
                near(a.width_mm, b.width_mm);
            }
        }
    }
    Ok(())
}

fn cases() -> Vec<(&'static str, f64, f64, usize)> {
    [
        "crop-default",
        "crop-custom",
        "crop-disabled",
        "crop-one",
        "crop-two",
        "crop-three",
        "crop-landscape",
        "crop-zero",
        "crop-colour",
    ]
    .into_iter()
    .map(|name| {
        if name == "crop-landscape" {
            (name, 150.5, 100.25, 2)
        } else {
            (
                name,
                100.0,
                150.0,
                if name == "crop-default" { 2 } else { 1 },
            )
        }
    })
    .collect()
}

fn trace(path: &Path) -> Result<String> {
    let output = Command::new("mutool")
        .args(["draw", "-q", "-F", "trace"])
        .arg(path)
        .output()?;
    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
    assert!(output.status.success());
    Ok(String::from_utf8(output.stdout)?)
}

// Normalize PDF affine transforms, including AlbumEasy's flipped y axis.
fn strokes(xml: &str) -> Result<Vec<Vec<f64>>> {
    let doc = roxmltree::Document::parse(xml)?;
    let mut result = Vec::new();
    for stroke in doc.descendants().filter(|n| n.has_tag_name("stroke_path")) {
        let operations: Vec<_> = stroke
            .children()
            .filter(|n| n.is_element())
            .map(|n| n.tag_name().name())
            .collect();
        if stroke.children().any(|n| n.has_tag_name("closepath")) {
            assert_eq!(
                operations,
                ["moveto", "lineto", "lineto", "lineto", "closepath"]
            );
        } else {
            assert_eq!(operations, ["moveto", "lineto", "moveto", "lineto"]);
        }
        let matrix = stroke
            .attribute("transform")
            .ok_or("transform")?
            .split_whitespace()
            .map(str::parse::<f64>)
            .collect::<std::result::Result<Vec<_>, _>>()?;
        assert_eq!(matrix.len(), 6);
        assert_eq!([matrix[0], matrix[1], matrix[2]], [1.0, 0.0, 0.0]);
        assert!((matrix[3].abs() - 1.0).abs() < 0.00001);
        assert_eq!(stroke.attribute("linecap"), Some("0,0,0"));
        let mut values = vec![points_to_mm(
            stroke.attribute("linewidth").ok_or("width")?.parse()?,
        )];
        values.extend(
            stroke
                .attribute("color")
                .ok_or("colour")?
                .split_whitespace()
                .map(str::parse::<f64>)
                .collect::<std::result::Result<Vec<_>, _>>()?,
        );
        for point in stroke
            .children()
            .filter(|n| n.has_tag_name("moveto") || n.has_tag_name("lineto"))
        {
            let x: f64 = point.attribute("x").ok_or("x")?.parse()?;
            let y: f64 = point.attribute("y").ok_or("y")?.parse()?;
            values.extend([
                points_to_mm(matrix[0] * x + matrix[2] * y + matrix[4]),
                points_to_mm(matrix[1] * x + matrix[3] * y + matrix[5]),
            ]);
        }
        result.push(values);
    }
    Ok(result)
}

#[test]
#[ignore = "requires mutool on PATH; compares actual native and AlbumEasy strokes"]
fn actual_native_paths_match_all_repeated_reference_cases() -> Result<()> {
    let temp = tempfile::tempdir()?;
    for (name, width, height, pages) in cases() {
        let source = fs::read_to_string(root().join(format!("{name}.txt")))?;
        let config_source = BASE
            .replace("= 100\n", &format!("= {width}\n"))
            .replace("= 150\n", &format!("= {height}\n"))
            .replace("page_count = 2", &format!("page_count = {pages}"));
        let mut loaded = NativePdfConfig::from_toml(&config_source)?;
        if name != "crop-disabled" {
            // Fixture-only optional-argument expansion from explicit reference defaults.
            let mut values = [0.1, 1.0, 1.0, 5.0];
            if let Some(line) = source
                .lines()
                .find(|line| line.starts_with("ALBUM_PAGES_CROP_MARKS ("))
            {
                let inner = line.split_once('(').ok_or("open")?.1.trim_end_matches(')');
                for (i, value) in inner.split_whitespace().enumerate() {
                    values[i] = value.parse()?;
                }
            }
            loaded
                .page_settings
                .set_crop_marks(AlbumPagesCropMarks::parse(
                    &values,
                    NamedColour::parse("black")?,
                )?);
        }
        if name == "crop-landscape" {
            loaded.page_settings.set_even_margins(stamp_album_gen2::native_pdf::commands::page::album_pages_marginse::EvenMargins::parse(&[12.0,15.0,16.0,17.0])?);
        }
        if name == "crop-colour" {
            loaded.page_settings.set_border_colour(stamp_album_gen2::native_pdf::commands::colour::colour_album_border::AlbumBorderColour::parse(&["pink"])?);
        }
        let path = temp.path().join(format!("{name}.pdf"));
        write_page_geometry_proof(&loaded.page_settings, &loaded.proof, &path)?;
        let native = strokes(&trace(&path)?)?;
        let reference = strokes(&trace(&root().join(format!("{name}.pdf")))?)?;
        assert_eq!(native.len(), reference.len(), "{name}");
        for (a, b) in native.iter().zip(&reference) {
            assert_eq!(a.len(), b.len(), "{name}");
            for (index, (actual, wanted)) in a.iter().zip(b).enumerate() {
                if (1..=3).contains(&index) {
                    assert!((actual - wanted).abs() <= 0.00051);
                } else {
                    near(*actual, *wanted);
                }
            }
        }
        let media = oxidize_pdf::parser::PdfDocument::open(&path)?
            .get_page(0)?
            .media_box;
        near(points_to_mm(media[2] - media[0]), width);
        near(points_to_mm(media[3] - media[1]), height);
    }
    Ok(())
}

#[test]
#[ignore = "requires mutool on PATH; verifies actual native crop colour override"]
fn rendered_native_colour_is_independent_of_album_border() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let loaded = config(&CROP.replace("colour = \"black\"", "colour = \"pink\""))?;
    let output = temp.path().join("pink.pdf");
    write_page_geometry_proof(&loaded.page_settings, &loaded.proof, &output)?;
    let paths = strokes(&trace(&output)?)?;
    assert_eq!(paths.len(), 14);
    let (pages, remainder) = paths.as_chunks::<7>();
    assert!(remainder.is_empty());
    for page in pages {
        for border in &page[..3] {
            assert_eq!(border[1..4], [0.0; 3]);
        }
        for crop in &page[3..] {
            for (actual, wanted) in crop[1..4].iter().zip(NamedColour::parse("pink")?.rgb()) {
                assert!((actual - wanted).abs() <= 0.00051);
            }
        }
    }
    Ok(())
}
