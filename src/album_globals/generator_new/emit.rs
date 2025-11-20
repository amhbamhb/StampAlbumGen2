use std::io::{self, Write};

/// Emit a stamp line and its heading into the AlbumEasy output.
///
/// - `is_image`: when true emits `STAMP_ADD_IMG (...)` using `image_path`,
///   otherwise emits `STAMP_ADD (...)` and treats `image_path` as the textual
///   content for the box (e.g. "Image\\nNot\\nAvailable").
pub fn emit_stamp_lines<W: Write>(
    out: &mut W,
    emit_width: f64,
    emit_height: f64,
    image_path: &str,
    left: &str,
    centre: &str,
    right: &str,
    config: &crate::config::Config,
    sgnum: &str,
    is_image: bool,
) -> io::Result<()> {
    if is_image {
        let line = format!(
            "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"{}\" \"{}\" \"{}\")",
            emit_width, emit_height, image_path, left, centre, right
        );
        writeln!(out, "{}", line.replace('\r', "").replace('\n', "\\n"))?;
    } else {
        let line = format!(
            "STAMP_ADD ({:.1} {:.1} \"{}\" \"{}\" \"{}\" \"{}\")",
            emit_width, emit_height, image_path, left, centre, right
        );
        writeln!(out, "{}", line.replace('\r', "").replace('\n', "\\n"))?;
    }

    let sh_font = config
        .stamp_heading_font
        .clone()
        .unwrap_or_else(|| "HN".to_string());
    let sh_size = config.stamp_heading_size.unwrap_or(10.0) as i32;
    let sh_label = config
        .stamp_heading_label
        .clone()
        .unwrap_or_else(|| "SG".to_string());
    writeln!(
        out,
        "STAMP_HEADING({} {} \"{} : {}\")",
        sh_font, sh_size, sh_label, sgnum
    )?;

    Ok(())
}

/// Emit a stamp block (either with an image or a textual placeholder)
///
/// This centralizes the formatting of `STAMP_ADD_IMG` / `STAMP_ADD` and
/// `STAMP_HEADING` so call sites can focus on layout and placeholder
/// generation logic.
pub fn emit_stamp<W: Write>(
    out: &mut W,
    has_image: bool,
    image_path: &str,
    emit_width: f64,
    emit_height: f64,
    left: &str,
    centre: &str,
    right: &str,
    sh_font: &str,
    sh_size: i32,
    sh_label: &str,
    sgnum: &str,
) -> std::io::Result<()> {
    if has_image {
        let line = format!(
            "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"{}\" \"{}\" \"{}\")",
            emit_width, emit_height, image_path, left, centre, right
        );
        writeln!(out, "{}", line.replace("\r", "").replace("\n", "\\n"))?;
        writeln!(
            out,
            "STAMP_HEADING({} {} \"{} : {}\")",
            sh_font, sh_size, sh_label, sgnum
        )?;
    } else {
        let line = format!(
            "STAMP_ADD ({:.1} {:.1} \"Image\\nNot\\nAvailable\" \"{}\" \"{}\" \"{}\")",
            emit_width, emit_height, left, centre, right
        );
        writeln!(out, "{}", line.replace("\r", "").replace("\n", "\\n"))?;
        writeln!(
            out,
            "STAMP_HEADING({} {} \"{} : {}\")",
            sh_font, sh_size, sh_label, sgnum
        )?;
    }
    Ok(())
}
