use crate::album_globals::structs::Stamp;
/// Commemorative Layout Module
///
/// Handles specialized layout for commemorative stamp albums.
/// Provides grid-based formatting with proper background colors,
/// aligned images, and structured description boxes.
use std::io::Write;

/// Helper function to clean up text for single-line AlbumEasy commands
/// Removes newlines and excess whitespace
fn clean_for_output(text: &str) -> String {
    text.lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Detect if a stamp is commemorative based on Scott catalog number and year
/// US commemoratives:
/// - 1955: 1064-1072 (9 stamps)
/// - More years can be added as ranges
pub fn is_commemorative(stamp: &Stamp, _year: Option<i32>) -> bool {
    if let Some(sg) = stamp.attrs.get("sg_number") {
        // Parse the scott number (may be "1064", "1064a", etc.)
        if let Ok(num) = sg
            .chars()
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u32>()
        {
            // US 1955 commemoratives are Scott 1064-1072
            if (1064..=1072).contains(&num) {
                return true;
            }
        }
    }
    false
}

/// Emit the commemorative page header with proper formatting
pub fn emit_commemorative_page_start<W: Write>(
    out: &mut W,
    _page_number: usize,
    _title: &str,
) -> std::io::Result<()> {
    writeln!(out, "PAGE_START")?;
    writeln!(out, "PAGE_VSPACE (3.0)")?;
    writeln!(out, "ROW_START_ES (HN 6 0.2)")?;
    writeln!(out)?;
    writeln!(out, "COLOR_STAMP_BORDER(black)")?;
    writeln!(out, "COLOR_STAMP_BACKGROUND(black)")?;
    writeln!(out)?;
    writeln!(out, "ROW_ALIGN_TOP")?;
    Ok(())
}

/// Emit a grid row with two stamps (images + headings)
pub fn emit_stamp_grid_row<W: Write>(
    out: &mut W,
    stamp1: &Stamp,
    stamp2: Option<&Stamp>,
) -> std::io::Result<()> {
    // First stamp
    if let (Some(w), Some(h)) = (stamp1.est_width_mm, stamp1.est_height_mm) {
        let title1 = if let Some(desc) = stamp1.attrs.get("description") {
            if !desc.trim().is_empty() {
                clean_for_output(desc)
            } else {
                stamp1
                    .attrs
                    .get("sg_number")
                    .map(|s| format!("Scott {}", s))
                    .unwrap_or_else(|| "Stamp".to_string())
            }
        } else {
            stamp1
                .attrs
                .get("sg_number")
                .map(|s| format!("Scott {}", s))
                .unwrap_or_else(|| "Stamp".to_string())
        };
        let denomination = stamp1
            .attrs
            .get("denomination")
            .map(|s| s.as_str())
            .unwrap_or("");
        writeln!(
            out,
            "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"{}\" \"Perf 10½ x 11\" \"Horizontal\")",
            w, h, stamp1.image_path, denomination
        )?;
        writeln!(out, "STAMP_HEADING (HB 9 \"{}\")", title1)?;
    }

    // Second stamp (if provided)
    if let Some(s2) = stamp2 {
        if let (Some(w), Some(h)) = (s2.est_width_mm, s2.est_height_mm) {
            let title2 = if let Some(desc) = s2.attrs.get("description") {
                if !desc.trim().is_empty() {
                    clean_for_output(desc)
                } else {
                    s2.attrs
                        .get("sg_number")
                        .map(|s| format!("Scott {}", s))
                        .unwrap_or_else(|| "Stamp".to_string())
                }
            } else {
                s2.attrs
                    .get("sg_number")
                    .map(|s| format!("Scott {}", s))
                    .unwrap_or_else(|| "Stamp".to_string())
            };
            let denomination = s2
                .attrs
                .get("denomination")
                .map(|s| s.as_str())
                .unwrap_or("");
            writeln!(
                out,
                "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"{}\" \"Perf 10½ x 11\" \"Horizontal\")",
                w, h, s2.image_path, denomination
            )?;
            writeln!(out, "STAMP_HEADING (HB 9 \"{}\")", title2)?;
        }
    }

    Ok(())
}

/// Emit description boxes for the grid row
pub fn emit_description_boxes<W: Write>(
    out: &mut W,
    stamp1: &Stamp,
    stamp2: Option<&Stamp>,
) -> std::io::Result<()> {
    writeln!(out, "PAGE_VSPACE (-5.0)")?;
    writeln!(out, "ROW_START_ES (HN 6 0.2)")?;
    writeln!(out, "ROW_ALIGN_TOP")?;
    writeln!(out, "COLOR_STAMP_BORDER(white)")?;
    writeln!(out, "COLOR_STAMP_BACKGROUND(white)")?;

    // Build description for first stamp
    let desc1 = build_stamp_description(stamp1);
    writeln!(
        out,
        "STAMP_ADD (28.0 32.0 \"{}\" \"\" \"\" \"\" NO_FRAME)",
        desc1
    )?;
    writeln!(out, "COLOR_STAMP_BORDER(black)")?;

    // If second stamp exists, add its description
    if let Some(s2) = stamp2 {
        writeln!(out, "COLOR_STAMP_BORDER(white)")?;
        let desc2 = build_stamp_description(s2);
        writeln!(
            out,
            "STAMP_ADD (28.0 32.0 \"{}\" \"\" \"\" \"\" NO_FRAME)",
            desc2
        )?;
        writeln!(out, "COLOR_STAMP_BORDER(black)")?;
    }

    writeln!(out, "COLOR_STAMP_BACKGROUND(black)")?;
    Ok(())
}

/// Build a formatted description for a stamp
fn build_stamp_description(stamp: &Stamp) -> String {
    let mut parts = Vec::new();

    // Title line
    if let Some(desc) = stamp.attrs.get("description") {
        parts.push(clean_for_output(desc));
    }

    // Issued date
    if let Some(issued) = stamp.attrs.get("issue_date") {
        if !issued.is_empty() {
            parts.push(format!("Issued: {}", issued));
        }
    }

    // Color
    if let Some(color) = stamp.attrs.get("color") {
        if !color.is_empty() {
            parts.push(format!("Color: {}", color));
        }
    }

    // Quantity
    if let Some(qty) = stamp.attrs.get("stamps_issued") {
        if !qty.is_empty() {
            parts.push(format!("Quantity: {}", qty));
        }
    }

    // Notes (if available)
    if let Some(notes) = stamp.attrs.get("notes") {
        if !notes.is_empty() {
            parts.push(notes.to_string());
        }
    }

    // Join with newlines, escape for output
    parts.join("\\n")
}

/// Close commemorative page formatting
pub fn emit_commemorative_page_end<W: Write>(out: &mut W) -> std::io::Result<()> {
    writeln!(out, "COLOR_STAMP_BACKGROUND(black)")?;
    Ok(())
}

/// Emit a full commemorative page with stamps grouped in a grid
pub fn emit_commemorative_page<W: Write>(
    out: &mut W,
    page_number: usize,
    title: &str,
    stamps: &[Stamp],
) -> std::io::Result<()> {
    emit_commemorative_page_start(out, page_number, title)?;

    // Group stamps into pairs (2 per row)
    let mut i = 0;
    while i < stamps.len() {
        let stamp1 = &stamps[i];
        let stamp2 = if i + 1 < stamps.len() {
            Some(&stamps[i + 1])
        } else {
            None
        };

        emit_stamp_grid_row(out, stamp1, stamp2)?;
        emit_description_boxes(out, stamp1, stamp2)?;

        if stamp2.is_some() {
            writeln!(out, "ROW_START_ES (HN 6 0.2)")?;
            writeln!(out, "ROW_ALIGN_TOP")?;
            i += 2;
        } else {
            i += 1;
        }
    }

    emit_commemorative_page_end(out)?;
    Ok(())
}
