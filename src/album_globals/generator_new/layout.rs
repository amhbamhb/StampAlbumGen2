use std::io::Write;

/// Emit the canonical PAGE_START marker.
pub fn write_page_start<W: Write>(out: &mut W) -> std::io::Result<()> {
    writeln!(out, "PAGE_START").map(|_| ())
}

/// Emit a PAGE_VSPACE marker with a floating-point millimetre amount.
/// The formatting follows the generator convention: whole numbers printed
/// without a decimal place, otherwise with one fraction digit.
pub fn write_page_vspace<W: Write>(out: &mut W, mm: f64) -> std::io::Result<()> {
    if mm.fract().abs() < 0.001 {
        writeln!(out, "PAGE_VSPACE ({:.0})", mm).map(|_| ())
    } else {
        writeln!(out, "PAGE_VSPACE ({:.1})", mm).map(|_| ())
    }
}

/// Emit a PAGE_VSPACE marker that carries a custom marker string instead of
/// a numeric millimetre value. The generator uses marker tokens (e.g. "6")
/// during initial emission so the postprocessor can later replace them with
/// concrete millimetre values; keep this behavior available during refactor.
pub fn write_page_vspace_marker<W: Write>(out: &mut W, marker: &str) -> std::io::Result<()> {
    writeln!(out, "PAGE_VSPACE ({})", marker).map(|_| ())
}

/// Return a PAGE_VSPACE string formatted from a millimetre value.
pub fn format_page_vspace_mm(mm: f64) -> String {
    if mm.fract().abs() < 0.001 {
        format!("PAGE_VSPACE ({:.0})", mm)
    } else {
        format!("PAGE_VSPACE ({:.1})", mm)
    }
}

/// Return a PAGE_VSPACE string using a marker token.
pub fn format_page_vspace_marker(marker: &str) -> String {
    format!("PAGE_VSPACE ({})", marker)
}

/// Emit a short PAGE_FIT diagnostic line used during page fit diagnostics.
pub fn write_page_fit_diagnostic<W: Write>(
    out: &mut W,
    series: &str,
    need: f64,
    current: f64,
    usable: f64,
    will_fit: bool,
) -> std::io::Result<()> {
    writeln!(
        out,
        "# PAGE_FIT series=\"{}\" need=\"{:.2}\" current=\"{:.2}\" usable=\"{:.2}\" will_fit=\"{}\"",
        series,
        need,
        current,
        usable,
        will_fit
    )
    .map(|_| ())
}

/// Write the standard stamp-page header for a given page number.
pub fn write_stamp_page_header<W: Write>(
    out: &mut W,
    page_number: usize,
    config: &crate::config::Config,
) -> std::io::Result<()> {
    writeln!(out, "### Page {} - Stamps ###", page_number)?;
    writeln!(out, "PAGE_START")?;
    // Emit the ROW_START_ES marker using the shared helper so formatting stays consistent.
    // Use the local helper inside `generator_new` so the refactor can proceed
    // without touching the original top-level helpers module.
    super::helpers::emit_row_start(out, RowSpacingMode::FS, Some(config))?;
    writeln!(
        out,
        "################################################################################"
    )?;
    Ok(())
}

/// Increment the page counter and write the standard stamp-page header.
/// This mirrors the common pattern in the original generator while keeping
/// the emission logic in one place for easier refactoring.
pub fn start_stamp_page<W: Write>(
    out: &mut W,
    page_counter: &mut usize,
    config: &crate::config::Config,
) -> std::io::Result<()> {
    *page_counter += 1;
    if *page_counter > 1 {
        writeln!(
            out,
            "\n################################################################################"
        )?;
    }
    write_stamp_page_header(out, *page_counter, config)
}

/// Emit the ROW_START_ES marker using the shared helper from `helpers`.
/// This wrapper centralizes where ROW_START_ES comes from so callers
/// can use `generator_new::layout::emit_row_start_es` during refactors.
use crate::album_globals::generator_new::helpers::RowSpacingMode;

pub fn emit_row_start<W: Write>(
    out: &mut W,
    mode: RowSpacingMode,
    config: &crate::config::Config,
) -> std::io::Result<()> {
    // forward to the existing helper to keep formatting consistent
    super::helpers::emit_row_start(out, mode, Some(config))
}

/// Measure the font and spacing from an emitted PAGE_TEXT command.
pub(crate) fn page_text_height(line: &str, fallback: f64) -> f64 {
    let Some((prefix, text)) = line.split_once('"') else {
        return fallback;
    };
    let Some((text, suffix)) = text.rsplit_once('"') else {
        return fallback;
    };
    let size = prefix
        .split_whitespace()
        .last()
        .and_then(|v| v.parse::<f64>().ok());
    let spacing = suffix
        .trim()
        .trim_end_matches(')')
        .trim()
        .parse::<f64>()
        .ok();
    match (size, spacing) {
        (Some(size), Some(spacing)) => {
            let lines = 1 + text.matches("\\n").count() + text.matches('\n').count();
            lines as f64 * size * 25.4 / 72.0 + spacing
        }
        _ => fallback,
    }
}

/// Write explicitly configured spacing without replacing it with marker tokens.
pub fn write_header_block<W: Write>(
    out: &mut W,
    pending_lines: &[(String, f64)],
    v_before: f64,
    v_after: f64,
    _config: &crate::config::Config,
) -> std::io::Result<f64> {
    write_page_vspace(out, v_before)?;

    for (line, _h) in pending_lines.iter() {
        writeln!(out, "{}", line)?;
    }
    let text_height: f64 = pending_lines.iter().map(|(_, h)| *h).sum();
    writeln!(out, "# HEADER_HEIGHT mm={:.4}", text_height)?;

    write_page_vspace(out, v_after)?;

    Ok(v_before + text_height + v_after)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_measurement_matches_font_spacing_and_explicit_breaks() {
        assert!(
            (page_text_height("PAGE_TEXT_CENTRE (HN 8 \"Series\" 1.0)", 0.0)
                - (8.0 * 25.4 / 72.0 + 1.0))
                .abs()
                < 0.001
        );
        assert!(
            (page_text_height("PAGE_TEXT_CENTRE (HN 8 \"A\\nB\" 2.0)", 0.0)
                - (16.0 * 25.4 / 72.0 + 2.0))
                .abs()
                < 0.001
        );
        assert_eq!(page_text_height("# comment", 3.0), 3.0);
    }

    #[test]
    fn header_spacing_is_numeric_even_with_a_legacy_marker() {
        let config = crate::config::Config {
            page_vspace_marker: Some("legacy-marker".to_string()),
            ..Default::default()
        };
        let mut output = Vec::new();
        let height = write_header_block(
            &mut output,
            &[("PAGE_TEXT_CENTRE (HN 8 \"Series\" 1.0)".to_string(), 4.0)],
            1.0,
            2.0,
            &config,
        )
        .expect("write header");
        let output = String::from_utf8(output).expect("UTF-8 output");
        assert_eq!(height, 7.0);
        assert!(output.starts_with("PAGE_VSPACE (1)\n"));
        assert!(output.ends_with("PAGE_VSPACE (2)\n"));
        assert!(output.contains("# HEADER_HEIGHT mm=4.0000"));
        assert!(!output.contains("legacy-marker"));
    }
}
