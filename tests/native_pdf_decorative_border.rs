use sha2::{Digest, Sha256};
use stamp_album_gen2::native_pdf::{
    colour::NamedColour,
    commands::page::album_pages_decorative_border::{
        AlbumPagesDecorativeBorder, DecorativeBorderPolicy,
    },
    config::NativePdfConfig,
    document::Bounds,
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
const POLICY: [f64; 11] = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
const BASE: &str = r#"
color_album_border = "red"
[page]
album_page_width_mm = 100
album_page_height_mm = 150
page_left_margin_mm = 15
page_right_margin_mm = 12
page_top_margin_mm = 16
page_bottom_margin_mm = 18
album_pages_spacing_h = 3
album_pages_spacing_v = 4
page_border_outer = 0
page_border_inner1 = 0
page_border_inner2 = 0
page_border_spacing = 0
[native_pdf.proof]
page_count = 1
guide_enabled = false
guide_width_mm = 0.1
guide_colour = "black"
"#;
const DECOR: &str = r#"
[native_pdf.decorative_border]
enabled = true
definition_file = "corner.def"
image_scale = 1.0
corner_padding_h_mm = 0.0
corner_padding_v_mm = 0.0
line_outer_mm = 0.0
line_inner1_mm = 0.0
line_inner2_mm = 0.0
line_spacing_mm = 0.0
line_padding_h_mm = 0.0
line_padding_v_mm = 0.0
end_offset_h_mm = 0.0
end_offset_v_mm = 0.0
colour = "black"
"#;
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/decorative-border-v1")
}
fn policy() -> Result<DecorativeBorderPolicy> {
    Ok(DecorativeBorderPolicy::parse(
        &POLICY,
        NamedColour::parse("black")?,
    )?)
}
fn near(a: f64, b: f64) {
    assert!(a.is_finite() && (a - b).abs() <= 0.01, "{a} != {b}");
}
fn loaded(name: &str) -> Result<NativePdfConfig> {
    Ok(NativePdfConfig::from_toml_in(
        &format!(
            "{BASE}{}",
            DECOR.replace("corner.def", &format!("{name}.def"))
        ),
        &root(),
    )?)
}

#[test]
fn mirrored_corner_bounds_and_fixed_slot_line_spacing() -> Result<()> {
    let command = AlbumPagesDecorativeBorder::load(&root().join("corner.def"), policy()?)?;
    let margin = Bounds {
        left: 15.0,
        right: 88.0,
        bottom: 18.0,
        top: 134.0,
    };
    let layout = command.layout(margin)?;
    assert_eq!(layout.corners.len(), 4);
    let w = 30.0 * 25.4 / 72.0;
    let h = 18.0 * 25.4 / 72.0;
    for (got, wanted) in layout.corners.iter().zip([
        [15.0, 134.0 - h, w, h],
        [88.0, 134.0 - h, -w, h],
        [15.0, 18.0 + h, w, -h],
        [88.0, 18.0 + h, -w, -h],
    ]) {
        for (a, b) in got.iter().zip(wanted) {
            near(*a, b);
        }
    }
    assert_eq!(layout.lines.len(), 12);
    for (index, line) in layout.lines.iter().enumerate() {
        near(line.width_mm, [0.3, 0.2, 0.1][index / 4]);
    }
    assert_eq!(layout.lines[0].points, [[27.0, 134.0], [76.0, 134.0]]);
    assert_eq!(layout.lines[4].points, [[27.0, 133.0], [76.0, 133.0]]);
    let footprints = layout.footprints();
    assert_eq!(footprints.len(), 16);
    near(footprints[4].left, 27.0);
    near(footprints[4].top, 134.15);
    near(footprints[4].bottom, 133.85);
    let sparse =
        AlbumPagesDecorativeBorder::load(&root().join("sparse.def"), policy()?)?.layout(margin)?;
    assert_eq!(sparse.lines.len(), 8);
    near(sparse.lines[0].points[0][1], 130.5);
    Ok(())
}

#[test]
fn explicit_policy_validation_and_definition_overrides() -> Result<()> {
    for count in [0, 10, 12] {
        assert!(
            DecorativeBorderPolicy::parse(&vec![1.0; count], NamedColour::parse("black")?).is_err()
        );
    }
    for index in 0..11 {
        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, f64::MAX] {
            let mut values = POLICY;
            values[index] = bad;
            assert!(DecorativeBorderPolicy::parse(&values, NamedColour::parse("black")?).is_err());
        }
    }
    for index in [0, 3, 4, 5, 6, 9, 10] {
        let mut values = POLICY;
        values[index] = -1.0;
        assert!(DecorativeBorderPolicy::parse(&values, NamedColour::parse("black")?).is_err());
    }
    let mut values = POLICY;
    values[0] = 0.0;
    assert!(DecorativeBorderPolicy::parse(&values, NamedColour::parse("black")?).is_err());
    let omitted = loaded("omitted")?
        .page_settings
        .resolve(1)?
        .decorative_border
        .ok_or("decor")?;
    assert!(omitted.lines.is_empty());
    let changed = DECOR
        .replace("corner.def", "omitted.def")
        .replace("image_scale = 1.0", "image_scale = 2.0");
    let config = NativePdfConfig::from_toml_in(&format!("{BASE}{changed}"), &root())?;
    near(
        config
            .page_settings
            .resolve(1)?
            .decorative_border
            .ok_or("decor")?
            .corners[0][2],
        60.0 * 25.4 / 72.0,
    );
    // Definition settings override the explicit omitted-command policy.
    let config = NativePdfConfig::from_toml_in(
        &format!(
            "{BASE}{}",
            DECOR.replace("image_scale = 1.0", "image_scale = 2.0")
        ),
        &root(),
    )?;
    near(
        config
            .page_settings
            .resolve(1)?
            .decorative_border
            .ok_or("decor")?
            .corners[0][2],
        30.0 * 25.4 / 72.0,
    );
    Ok(())
}

#[test]
fn strict_unsupported_invalid_and_redacted_definition_errors() -> Result<()> {
    let temp = tempfile::tempdir()?;
    fs::copy(
        root().join("ornament.png"),
        temp.path().join("ornament.png"),
    )?;
    let file = temp.path().join("border.def");
    for (source, key) in [
        ("IMAGE_EDGE_LEFT(\"ornament.png\")", "unsupported"),
        ("UNKNOWN(\"secret\")", "unsupported"),
        (
            "IMAGE_SCALE(nan)\nIMAGE_CORNER(\"ornament.png\")",
            "image_scale",
        ),
        (
            "IMAGE_SCALE(0)\nIMAGE_CORNER(\"ornament.png\")",
            "image_scale",
        ),
        (
            "IMAGE_CORNER_PAD(1)\nIMAGE_CORNER(\"ornament.png\")",
            "arguments",
        ),
        (
            "BORDER_LINES(1 2 3)\nIMAGE_CORNER(\"ornament.png\")",
            "arguments",
        ),
        ("IMAGE_CORNER(\"secret", "syntax"),
        ("IMAGE_CORNER(\"ornament.png\") trailing", "syntax"),
        ("BORDER_LINES(0 0 0 0)", "IMAGE_CORNER"),
        ("IMAGE_CORNER(ornament.png)", "quoted"),
        ("IMAGE_CORNER(\"\")", "empty"),
    ] {
        fs::write(&file, source)?;
        let error = AlbumPagesDecorativeBorder::load(&file, policy()?)
            .err()
            .ok_or("invalid accepted")?;
        assert!(error.contains(key), "{source}: {error}");
        assert!(!error.contains("secret"), "{error}");
    }
    fs::write(&file,"# Comment\nIMAGE_SCALE(0.5)\nIMAGE_SCALE(1) # last wins\nIMAGE_CORNER(\"ornament.png\");\n")?;
    let layout = AlbumPagesDecorativeBorder::load(&file, policy()?)?.layout(Bounds {
        left: 10.0,
        bottom: 10.0,
        right: 90.0,
        top: 140.0,
    })?;
    near(layout.corners[0][2], 30.0 * 25.4 / 72.0);
    Ok(())
}

#[test]
fn files_resolve_relative_to_config_and_definition_and_are_snapshotted() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let assets = temp.path().join("assets with spaces");
    fs::create_dir(&assets)?;
    fs::copy(root().join("ornament.png"), assets.join("ornament.png"))?;
    fs::copy(root().join("corner.def"), assets.join("corner.def"))?;
    let config = temp.path().join("user config.toml");
    fs::write(
        &config,
        format!(
            "{BASE}{}",
            DECOR.replace("corner.def", "assets with spaces/corner.def")
        ),
    )?;
    let snapshot = NativePdfConfig::load(&config)?;
    fs::remove_file(assets.join("corner.def"))?;
    fs::remove_file(assets.join("ornament.png"))?;
    let output = temp.path().join("snapshot.pdf");
    write_page_geometry_proof(&snapshot.page_settings, &snapshot.proof, &output)?;
    assert!(output.exists());
    assert!(NativePdfConfig::load(&config).is_err());
    assert!(NativePdfConfig::from_toml(&format!("{BASE}{DECOR}")).is_err());
    let absolute = DECOR.replace(
        "corner.def",
        &root()
            .join("corner.def")
            .to_string_lossy()
            .replace('\\', "/"),
    );
    assert!(NativePdfConfig::from_toml(&format!("{BASE}{absolute}"))?
        .page_settings
        .resolve(1)?
        .decorative_border
        .is_some());
    Ok(())
}

#[test]
fn explicit_config_disabled_compatibility_and_templates() -> Result<()> {
    assert!(NativePdfConfig::from_toml(BASE)?
        .page_settings
        .resolve(1)?
        .decorative_border
        .is_none());
    let disabled = DECOR
        .replace("enabled = true", "enabled = false")
        .replace("corner.def", "");
    assert!(NativePdfConfig::from_toml(&format!("{BASE}{disabled}"))?
        .page_settings
        .resolve(1)?
        .decorative_border
        .is_none());
    for line in DECOR.lines().filter(|line| line.contains(" = ")) {
        let key = line.split_once(" = ").ok_or("key")?.0;
        let bad = DECOR
            .lines()
            .filter(|line| !line.starts_with(key))
            .collect::<Vec<_>>()
            .join("\n");
        let error = NativePdfConfig::from_toml_in(&format!("{BASE}{bad}"), &root())
            .err()
            .ok_or("missing accepted")?;
        assert!(
            error.contains(&format!("native_pdf.decorative_border.{key}")),
            "{error}"
        );
    }
    let invalid = disabled.replace("image_scale = 1.0", "image_scale = -1");
    assert!(NativePdfConfig::from_toml(&format!("{BASE}{invalid}")).is_err());
    for (from, to, key) in [
        ("colour = \"black\"", "colour = \"secret\"", "colour"),
        ("enabled = true", "enabled = \"secret\"", "enabled"),
        (
            "image_scale = 1.0",
            "image_scale = \"secret\"",
            "image_scale",
        ),
        (
            "definition_file = \"corner.def\"",
            "definition_file = \"\"",
            "definition_file",
        ),
    ] {
        let error =
            NativePdfConfig::from_toml_in(&format!("{BASE}{}", DECOR.replace(from, to)), &root())
                .err()
                .ok_or("invalid accepted")?;
        assert!(
            error.contains(&format!("native_pdf.decorative_border.{key}")),
            "{error}"
        );
        assert!(!error.contains("secret"), "{error}");
    }
    let unknown = NativePdfConfig::from_toml_in(&format!("{BASE}{DECOR}\nfuture = 1"), &root())?;
    assert_eq!(
        unknown.unapplied_keys,
        ["native_pdf.decorative_border.future"]
    );
    for source in [
        include_str!("../config_template.toml"),
        include_str!("../config_full_example.toml"),
    ] {
        let table: toml::Table = toml::from_str(source)?;
        assert_eq!(
            table["native_pdf"]["decorative_border"]["enabled"].as_bool(),
            Some(false)
        );
        assert_eq!(
            table["native_pdf"]["decorative_border"]["definition_file"].as_str(),
            Some("")
        );
        let loaded = NativePdfConfig::from_toml(source)?;
        assert!(loaded.page_settings.resolve(1)?.decorative_border.is_none());
        assert!(!loaded
            .unapplied_keys
            .iter()
            .any(|key| key.starts_with("native_pdf.decorative_border")));
    }
    Ok(())
}

#[test]
fn missing_corrupt_and_non_alpha_png_assets_fail_before_output() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let file = temp.path().join("border.def");
    fs::write(&file, "IMAGE_CORNER(\"missing.png\")")?;
    assert!(AlbumPagesDecorativeBorder::load(&file, policy()?).is_err());
    fs::write(temp.path().join("missing.png"), b"not png")?;
    assert!(AlbumPagesDecorativeBorder::load(&file, policy()?).is_err());
    image::RgbImage::new(4, 4).save(temp.path().join("missing.png"))?;
    let error = AlbumPagesDecorativeBorder::load(&file, policy()?)
        .err()
        .ok_or("no alpha accepted")?;
    assert!(error.contains("alpha"), "{error}");
    Ok(())
}

#[test]
fn numeric_content_is_unchanged_and_invalid_geometry_preserves_output() -> Result<()> {
    let configured = loaded("corner")?;
    let plain = NativePdfConfig::from_toml(BASE)?.page_settings.resolve(1)?;
    let decorated = configured.page_settings.resolve(1)?;
    for (a, b) in [
        decorated.content_bounds.left,
        decorated.content_bounds.right,
        decorated.content_bounds.top,
        decorated.content_bounds.bottom,
    ]
    .into_iter()
    .zip([
        plain.content_bounds.left,
        plain.content_bounds.right,
        plain.content_bounds.top,
        plain.content_bounds.bottom,
    ]) {
        near(a, b);
    }
    let temp = tempfile::tempdir()?;
    let existing = temp.path().join("existing.pdf");
    fs::write(&existing, b"keep me")?;
    let mut bad = configured;
    let mut values = POLICY;
    values[0] = 6e307;
    let command = AlbumPagesDecorativeBorder::load(
        &root().join("omitted.def"),
        DecorativeBorderPolicy::parse(&values, NamedColour::parse("black")?)?,
    )?;
    bad.page_settings.set_decorative_border(command);
    for path in [&existing, &temp.path().join("new.pdf")] {
        assert!(write_page_geometry_proof(&bad.page_settings, &bad.proof, path).is_err());
    }
    let mut late = loaded("corner")?;
    late.proof.page_count = 2;
    late.page_settings.set_even_margins(
        stamp_album_gen2::native_pdf::commands::page::album_pages_marginse::EvenMargins::parse(&[
            45.0, 40.0, 16.0, 18.0,
        ])?,
    );
    assert!(late.page_settings.resolve(1).is_ok());
    assert!(late.page_settings.resolve(2).is_err());
    for path in [&existing, &temp.path().join("new.pdf")] {
        let error = write_page_geometry_proof(&late.page_settings, &late.proof, path)
            .expect_err("page two accepted");
        assert!(error.contains("page 2"), "{error}");
    }
    assert_eq!(fs::read(existing)?, b"keep me");
    assert!(!temp.path().join("new.pdf").exists());
    Ok(())
}

#[test]
fn frozen_reference_hashes_and_text() -> Result<()> {
    let source = fs::read_to_string(root().join("sha256.txt"))?;
    assert_eq!(source.lines().count(), 16);
    for line in source.lines() {
        let (hash, file) = line.split_once("  ").ok_or("hash")?;
        let actual: String = Sha256::digest(fs::read(root().join(file))?)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(actual, hash, "{file}");
    }
    for name in ["corner", "padded", "sparse", "image-only", "omitted"] {
        let pdf = oxidize_pdf::parser::PdfDocument::open(root().join(format!("{name}.pdf")))?;
        assert_eq!(pdf.page_count()?, if name == "padded" { 2 } else { 1 });
        assert!(pdf.extract_text()?[0].text.contains("DECOR ANCHOR"));
    }
    Ok(())
}

#[test]
#[ignore = "requires mutool; compares mirrored PNG RGB and transparency against AlbumEasy rasters"]
fn actual_corner_pixels_and_transparency_match() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let loaded = loaded("image-only")?;
    let native = temp.path().join("native.pdf");
    write_page_geometry_proof(&loaded.page_settings, &loaded.proof, &native)?;
    let mut rasters = Vec::new();
    for (name, path) in [
        ("native", native),
        ("reference", root().join("image-only.pdf")),
    ] {
        let output = temp.path().join(format!("{name}.png"));
        let result = Command::new("mutool")
            .args(["draw", "-q", "-r", "144", "-o"])
            .arg(&output)
            .arg(path)
            .output()?;
        eprint!("{}", String::from_utf8_lossy(&result.stderr));
        assert!(result.status.success());
        rasters.push(image::open(output)?.to_rgb8());
    }
    assert_eq!(rasters[0].dimensions(), rasters[1].dimensions());
    let corners = loaded
        .page_settings
        .resolve(1)?
        .decorative_border
        .ok_or("decor")?
        .corners;
    for (index, [x, y, w, h]) in corners.into_iter().enumerate() {
        for (u, v) in [(0.05, 0.1), (0.3, 0.4), (0.7, 0.65), (0.9, 0.9)] {
            // The reference intentionally overlays body text on the top-left ornament.
            if index == 0 && v > 0.15 {
                continue;
            }
            let px = ((x + u * w) * 72.0 / 25.4 * 2.0).floor() as u32;
            let py = ((150.0 - y - (1.0 - v) * h) * 72.0 / 25.4 * 2.0).floor() as u32;
            let native = rasters[0].get_pixel(px, py);
            let reference = rasters[1].get_pixel(px, py);
            for (a, b) in native.0.into_iter().zip(reference.0) {
                assert!(
                    a.abs_diff(b) <= 2,
                    "corner {index} ({u},{v}) native {native:?} reference {reference:?}"
                );
            }
        }
    }
    Ok(())
}

fn trace(path: &Path) -> Result<String> {
    let result = Command::new("mutool")
        .args(["draw", "-q", "-F", "trace"])
        .arg(path)
        .output()?;
    eprint!("{}", String::from_utf8_lossy(&result.stderr));
    assert!(result.status.success());
    Ok(String::from_utf8(result.stdout)?)
}
fn nums(text: &str) -> Result<Vec<f64>> {
    Ok(text
        .split_whitespace()
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()?)
}

#[test]
#[ignore = "requires mutool; compares actual native and AlbumEasy paths and mirrored PNG transforms"]
fn actual_native_strokes_images_alpha_and_content_match_references() -> Result<()> {
    let temp = tempfile::tempdir()?;
    for name in ["corner", "padded", "sparse", "image-only", "omitted"] {
        let mut source = format!(
            "{BASE}{}",
            DECOR.replace("corner.def", &format!("{name}.def"))
        );
        if name == "sparse" {
            source = source
                .replace("= 100\n", "= 150.25\n")
                .replace("= 150\n", "= 100.5\n");
        }
        if name == "padded" {
            source = source
                .replace("page_count = 1", "page_count = 2")
                .replace("page_border_outer = 0\n", "page_border_outer = 0.2\n");
            source.push_str(
                "\n[page.even_margins]\nleft_mm=12\nright_mm=15\ntop_mm=18\nbottom_mm=20\n",
            );
        }
        let loaded = NativePdfConfig::from_toml_in(&source, &root())?;
        let output = temp.path().join(format!("{name}.pdf"));
        write_page_geometry_proof(&loaded.page_settings, &loaded.proof, &output)?;
        let native = trace(&output)?;
        let reference = trace(&root().join(format!("{name}.pdf")))?;
        let native = roxmltree::Document::parse(&native)?;
        let reference = roxmltree::Document::parse(&reference)?;
        let native_pages: Vec<_> = native
            .descendants()
            .filter(|n| n.has_tag_name("page"))
            .collect();
        let reference_pages: Vec<_> = reference
            .descendants()
            .filter(|n| n.has_tag_name("page"))
            .collect();
        assert_eq!(native_pages.len(), reference_pages.len());
        for (index, (a, b)) in native_pages.iter().zip(reference_pages).enumerate() {
            for (x, y) in nums(a.attribute("mediabox").ok_or("media")?)?
                .into_iter()
                .zip(nums(b.attribute("mediabox").ok_or("media")?)?)
            {
                near(x, y);
            }
            for tag in ["fill_image", "clip_image_mask", "stroke_path"] {
                let a: Vec<_> = a.descendants().filter(|n| n.has_tag_name(tag)).collect();
                let b: Vec<_> = b.descendants().filter(|n| n.has_tag_name(tag)).collect();
                assert_eq!(a.len(), b.len(), "{name} {tag}");
                for (a, b) in a.into_iter().zip(b) {
                    if tag == "stroke_path" {
                        assert_eq!(a.attribute("linecap"), Some("0,0,0"));
                        near(
                            a.attribute("linewidth").ok_or("width")?.parse()?,
                            b.attribute("linewidth").ok_or("width")?.parse()?,
                        );
                        assert_eq!(a.attribute("color"), b.attribute("color"));
                        let ma = nums(a.attribute("transform").ok_or("matrix")?)?;
                        let mb = nums(b.attribute("transform").ok_or("matrix")?)?;
                        let pa: Vec<_> = a.children().filter(|n| n.is_element()).collect();
                        let pb: Vec<_> = b.children().filter(|n| n.is_element()).collect();
                        assert_eq!(pa.len(), pb.len());
                        for (a, b) in pa.into_iter().zip(pb) {
                            assert_eq!(a.tag_name().name(), b.tag_name().name());
                            if a.has_tag_name("closepath") {
                                continue;
                            }
                            let ax: f64 = a.attribute("x").ok_or("x")?.parse()?;
                            let ay: f64 = a.attribute("y").ok_or("y")?.parse()?;
                            let bx: f64 = b.attribute("x").ok_or("x")?.parse()?;
                            let by: f64 = b.attribute("y").ok_or("y")?.parse()?;
                            near(ma[0] * ax + ma[4], mb[0] * bx + mb[4]);
                            near(ma[3] * ay + ma[5], mb[3] * by + mb[5]);
                        }
                    } else {
                        for (x, y) in nums(a.attribute("transform").ok_or("matrix")?)?
                            .into_iter()
                            .zip(nums(b.attribute("transform").ok_or("matrix")?)?)
                        {
                            near(x, y);
                        }
                        assert_eq!(a.attribute("width"), b.attribute("width"));
                        assert_eq!(a.attribute("height"), b.attribute("height"));
                    }
                }
            }
            let layout = loaded.page_settings.resolve(index + 1)?;
            let first = b
                .descendants()
                .find(|n| n.has_tag_name("g"))
                .ok_or("glyph")?;
            near(
                layout.content_bounds.left,
                points_to_mm(first.attribute("x").ok_or("x")?.parse()?),
            );
            let y: f64 = first.attribute("y").ok_or("y")?.parse()?;
            near(
                layout.content_bounds.top - points_to_mm(10.0),
                points_to_mm(y),
            );
        }
    }
    let loaded = NativePdfConfig::from_toml_in(
        &format!(
            "{BASE}{}",
            DECOR.replace("colour = \"black\"", "colour = \"pink\"")
        ),
        &root(),
    )?;
    let output = temp.path().join("pink.pdf");
    write_page_geometry_proof(&loaded.page_settings, &loaded.proof, &output)?;
    let xml = trace(&output)?;
    let doc = roxmltree::Document::parse(&xml)?;
    let paths: Vec<_> = doc
        .descendants()
        .filter(|n| n.has_tag_name("stroke_path"))
        .collect();
    assert_eq!(paths.len(), 12);
    for path in paths {
        let rgb = nums(path.attribute("color").ok_or("colour")?)?;
        for (a, b) in rgb.into_iter().zip(NamedColour::parse("pink")?.rgb()) {
            assert!((a - b).abs() <= 0.00051);
        }
    }
    Ok(())
}
