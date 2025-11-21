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
    super::helpers::emit_row_start_es(out, Some(config))?;
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
pub fn emit_row_start_es<W: Write>(
    out: &mut W,
    config: &crate::config::Config,
) -> std::io::Result<()> {
    // forward to the existing helper to keep formatting consistent
    super::helpers::emit_row_start_es(out, Some(config))
}

/// Write a buffered header block: vspace before, the header lines, and vspace after.
/// Returns the total vertical millimetre height consumed by the block.
pub fn write_header_block<W: Write>(
    out: &mut W,
    pending_lines: &[(String, f64)],
    v_before: f64,
    v_after: f64,
    _config: &crate::config::Config,
) -> std::io::Result<f64> {
    // Emit the leading vspace, header text lines, then trailing vspace.
    // If the config requests a page_vspace_marker token, emit that
    // instead of numeric PAGE_VSPACE values so the postprocessor can
    // replace tokens with concrete values later.
    if let Some(marker) = _config.page_vspace_marker.as_ref() {
        write_page_vspace_marker(out, marker.as_str())?;
    } else {
        write_page_vspace(out, v_before)?;
    }

    for (line, _h) in pending_lines.iter() {
        writeln!(out, "{}", line)?;
    }

    if let Some(marker) = _config.page_vspace_marker.as_ref() {
        write_page_vspace_marker(out, marker.as_str())?;
    } else {
        write_page_vspace(out, v_after)?;
    }

    let text_height: f64 = pending_lines.iter().map(|(_, h)| *h).sum();
    Ok(v_before + text_height + v_after)
}
