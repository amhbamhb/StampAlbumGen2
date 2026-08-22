use std::io::{self, Write};
use log::debug;

fn escape_ae_text(v: &str) -> String {
    // AlbumEasy command args are wrapped in double quotes; embedded quotes
    // are normalized to apostrophes for parser safety.
    v.replace('"', "'")
}

fn heading_max_chars_per_line(emit_width: f64, sh_size: i32) -> usize {
    // Rough width model for HN heading text. Keep bounded so extremely
    // narrow or wide stamps still wrap in a stable way.
    let font_size = (sh_size.max(1)) as f64;
    let mm_per_char = (font_size * 0.42).max(1.8);
    let estimated = (emit_width / mm_per_char).floor() as usize;
    estimated.clamp(16, 72)
}

fn wrap_segment_words(segment: &str, max_chars: usize) -> Vec<String> {
    let clean = segment.trim();
    if clean.is_empty() {
        return Vec::new();
    }

    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();
    for word in clean.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= max_chars {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current);
            current = word.to_string();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

#[allow(dead_code)]
fn wrap_preserving_breaks(text: &str, max_chars: usize) -> Vec<String> {
    let normalized = text.replace("\\n", "\n");
    let mut out: Vec<String> = Vec::new();
    for seg in normalized.split('\n') {
        let wrapped = wrap_segment_words(seg, max_chars);
        if wrapped.is_empty() {
            continue;
        }
        out.extend(wrapped);
    }
    out
}

fn wrap_preserving_breaks_keep_empty(text: &str, max_chars: usize) -> Vec<String> {
    let normalized = text.replace("\\n", "\n");
    let mut out: Vec<String> = Vec::new();
    for seg in normalized.split('\n') {
        let wrapped = wrap_segment_words(seg, max_chars);
        if wrapped.is_empty() {
            // Preserve explicit user-entered blank lines (e.g. trailing "\n\n")
            // so STAMP_HEADING can intentionally add vertical spacing.
            out.push(String::new());
        } else {
            out.extend(wrapped);
        }
    }
    out
}

/// Context struct grouping common parameters for stamp emission helpers.
pub struct EmitContext<'a> {
    pub emit_width: f64,
    pub emit_height: f64,
    pub image_path: &'a str,
    pub left: &'a str,
    pub centre: &'a str,
    pub right: &'a str,
    pub sh_font: &'a str,
    pub sh_size: i32,
    pub sh_label: &'a str,
    pub sgnum: &'a str,
    pub stamp_name: Option<&'a str>,
    pub perforation: Option<&'a str>,
    pub include_perforation: bool,
}

fn build_stamp_heading_lines(
    sh_label: &str,
    sgnum: &str,
    stamp_name: Option<&str>,
    perforation: Option<&str>,
    include_perforation: bool,
    emit_width: f64,
    sh_size: i32,
) -> Vec<String> {
    if sh_label.trim().is_empty() {
        return Vec::new();
    }

    let max_chars = heading_max_chars_per_line(emit_width, sh_size);
    let mut lines: Vec<String> = vec![format!("{} : {}", sh_label.trim(), sgnum.trim())];

    if let Some(name) = stamp_name {
        lines.extend(wrap_preserving_breaks_keep_empty(name, max_chars));
    }

    if include_perforation {
        if let Some(perf) = perforation {
            if !perf.trim().is_empty() {
                lines.extend(wrap_preserving_breaks_keep_empty(
                    &format!("Perforation: {}", perf),
                    max_chars,
                ));
            }
        }
    }

    lines
}

fn format_stamp_heading(
    sh_label: &str,
    sgnum: &str,
    stamp_name: Option<&str>,
    perforation: Option<&str>,
    include_perforation: bool,
    emit_width: f64,
    sh_size: i32,
) -> Option<String> {
    let lines = build_stamp_heading_lines(
        sh_label,
        sgnum,
        stamp_name,
        perforation,
        include_perforation,
        emit_width,
        sh_size,
    );
    if lines.is_empty() {
        return None;
    }
    Some(lines.join(" \\n ").replace('"', "'"))
}

pub fn estimate_stamp_heading_line_count(
    sh_label: &str,
    sgnum: &str,
    stamp_name: Option<&str>,
    perforation: Option<&str>,
    include_perforation: bool,
    emit_width: f64,
    sh_size: i32,
) -> usize {
    build_stamp_heading_lines(
        sh_label,
        sgnum,
        stamp_name,
        perforation,
        include_perforation,
        emit_width,
        sh_size,
    )
    .len()
}

/// Emit a stamp line and its heading into the AlbumEasy output.
///
/// - `is_image`: when true emits `STAMP_ADD_IMG (...)` using `image_path`,
///   otherwise emits `STAMP_ADD (...)` and treats `image_path` as the textual
///   content for the box (e.g. "Image\\nNot\\nAvailable").
pub fn emit_stamp_lines<W: Write>(
    out: &mut W,
    ctx: &EmitContext,
    config: &crate::config::Config,
    is_image: bool,
) -> io::Result<()> {
    if is_image {
        debug!("emit_stamp_lines: emitting image: path='{}' sg='{}' emit_size=({:.1},{:.1})", ctx.image_path, ctx.sgnum, ctx.emit_width, ctx.emit_height);
        let image_path = escape_ae_text(ctx.image_path);
        let left = escape_ae_text(ctx.left);
        let centre = escape_ae_text(ctx.centre);
        let right = escape_ae_text(ctx.right);
        let line = format!(
            "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"{}\" \"{}\" \"{}\")",
            ctx.emit_width, ctx.emit_height, image_path, left, centre, right
        );
        writeln!(out, "{}", line.replace('\r', "").replace('\n', "\\n"))?;
    } else {
        debug!("emit_stamp_lines: emitting placeholder/text image_path='{}' sg='{}'", ctx.image_path, ctx.sgnum);
        let image_path = escape_ae_text(ctx.image_path);
        let left = escape_ae_text(ctx.left);
        let centre = escape_ae_text(ctx.centre);
        let right = escape_ae_text(ctx.right);
        let line = format!(
            "STAMP_ADD ({:.1} {:.1} \"{}\" \"{}\" \"{}\" \"{}\")",
            ctx.emit_width, ctx.emit_height, image_path, left, centre, right
        );
        writeln!(out, "{}", line.replace('\r', "").replace('\n', "\\n"))?;
    }
    let sh_label = if config.include_catalog_numbers.unwrap_or(true) {
        resolve_stamp_heading_label(config, None)
    } else {
        String::new()
    };
    // Only emit STAMP_HEADING if a label is configured (the number alone
    // is from StampWorld's internal system, not a real catalog)
    if !sh_label.is_empty() {
        let sh_font = config
            .stamp_heading_font
            .clone()
            .unwrap_or_else(|| "HN".to_string());
        let sh_size = config.stamp_heading_size.unwrap_or(10.0) as i32;
        writeln!(
            out,
            "STAMP_HEADING({} {} \"{} : {}\")",
            sh_font, sh_size, sh_label, ctx.sgnum
        )?;
    }

    Ok(())
}

pub fn resolve_stamp_heading_label(
    config: &crate::config::Config,
    catalog_code: Option<&str>,
) -> String {
    let configured = config
        .stamp_heading_label
        .as_deref()
        .map(str::trim)
        .unwrap_or("auto");
    if configured.is_empty() {
        return String::new();
    }
    if configured.eq_ignore_ascii_case("auto") {
        let fallback_catalog_code = config.effective_default_catalog_code();
        return crate::catalog::catalog_display_name(
            catalog_code.unwrap_or(fallback_catalog_code.as_str()),
        )
        .to_string();
    }

    // Backward-compatibility: many existing configs pinned
    // `stamp_heading_label` to a known catalog family label (for example
    // legacy "Stanley Gibbons") before per-stamp catalog family edits existed.
    // When a stamp carries an explicit catalog family, prefer that family
    // label for known catalog-name labels, so row-level Review UI edits are
    // reflected in PDF output.
    if let Some(code) = catalog_code {
        let explicit_code = crate::catalog::normalize_catalog_type(code);
        if !explicit_code.is_empty() {
            let configured_norm = configured.to_ascii_lowercase();
            let known_catalog_labels = [
                crate::catalog::catalog_display_name("sn"),
                crate::catalog::catalog_display_name("sg"),
                crate::catalog::catalog_display_name("sw"),
                crate::catalog::catalog_display_name("mi"),
                crate::catalog::catalog_display_name("yt"),
                crate::catalog::catalog_display_name("afa"),
                crate::catalog::catalog_display_name("nvp"),
                crate::catalog::catalog_display_name("un"),
                crate::catalog::catalog_display_name("col"),
                crate::catalog::catalog_display_name("pol"),
                crate::catalog::catalog_display_name("nor"),
                crate::catalog::catalog_display_name("zum"),
                crate::catalog::catalog_display_name("wad"),
                crate::catalog::catalog_display_name(""),
            ];
            let is_legacy_sg_label = configured_norm == "stanley gibbons";
            if is_legacy_sg_label
                || known_catalog_labels
                    .iter()
                    .any(|name| configured_norm == name.to_ascii_lowercase())
            {
                return crate::catalog::catalog_display_name(&explicit_code).to_string();
            }
        }
    }

    configured.to_string()
}

/// Emit a stamp block (either with an image or a textual placeholder)
///
/// This centralizes the formatting of `STAMP_ADD_IMG` / `STAMP_ADD` and
/// `STAMP_HEADING` so call sites can focus on layout and placeholder
/// generation logic.
pub fn emit_stamp<W: Write>(out: &mut W, has_image: bool, ctx: &EmitContext) -> std::io::Result<()> {
    if has_image {
        let image_path = escape_ae_text(ctx.image_path);
        let left = escape_ae_text(ctx.left);
        let centre = escape_ae_text(ctx.centre);
        let right = escape_ae_text(ctx.right);
        let line = format!(
            "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"{}\" \"{}\" \"{}\")",
            ctx.emit_width, ctx.emit_height, image_path, left, centre, right
        );
        writeln!(out, "{}", line.replace("\r", "").replace("\n", "\\n"))?;
        // Only emit STAMP_HEADING if a label is configured (the number alone
        // is from StampWorld's internal system, not a real catalog)
        if !ctx.sh_label.is_empty() {
            if let Some(heading) = format_stamp_heading(
                ctx.sh_label,
                ctx.sgnum,
                ctx.stamp_name,
                ctx.perforation,
                ctx.include_perforation,
                ctx.emit_width,
                ctx.sh_size,
            ) {
                writeln!(out, "STAMP_HEADING({} {} \"{}\")", ctx.sh_font, ctx.sh_size, heading)?;
            }
        }
    } else {
        let left = escape_ae_text(ctx.left);
        let centre = escape_ae_text(ctx.centre);
        let right = escape_ae_text(ctx.right);
        let line = format!(
            "STAMP_ADD ({:.1} {:.1} \"Image\\nNot\\nAvailable\" \"{}\" \"{}\" \"{}\")",
            ctx.emit_width, ctx.emit_height, left, centre, right
        );
        writeln!(out, "{}", line.replace("\r", "").replace("\n", "\\n"))?;
        // Only emit STAMP_HEADING if a label is configured
        if !ctx.sh_label.is_empty() {
            if let Some(heading) = format_stamp_heading(
                ctx.sh_label,
                ctx.sgnum,
                ctx.stamp_name,
                ctx.perforation,
                ctx.include_perforation,
                ctx.emit_width,
                ctx.sh_size,
            ) {
                writeln!(out, "STAMP_HEADING({} {} \"{}\")", ctx.sh_font, ctx.sh_size, heading)?;
            }
        }
    }
    Ok(())
}

/// Emit only an image stamp line (no `STAMP_HEADING`) with optional extra
/// options appended (e.g. `NO_FRAME`). This is useful for cover/flag images
/// where a heading should not be emitted.
pub fn emit_image_only<W: Write>(out: &mut W, ctx: &EmitContext, options: Option<&str>) -> std::io::Result<()> {
    let image_path = escape_ae_text(ctx.image_path);
    let left = escape_ae_text(ctx.left);
    let centre = escape_ae_text(ctx.centre);
    let right = escape_ae_text(ctx.right);
    let mut line = format!(
        "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"{}\" \"{}\" \"{}\"",
        ctx.emit_width, ctx.emit_height, image_path, left, centre, right
    );
    if let Some(opts) = options {
        // append a space then the options token(s) and close paren
        line.push(' ');
        line.push_str(opts);
        line.push(')');
    } else {
        line.push(')');
    }
    writeln!(out, "{}", line.replace("\r", "").replace("\n", "\\n"))?;
    Ok(())
}
