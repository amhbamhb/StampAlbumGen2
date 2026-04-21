/// Commemorative Album Generator
/// Generates complete commemorative album output matching the sample format
/// Works for any year by defining commemorative ranges
use crate::album_globals::structs::Stamp;
use std::io::Write;

/// Define commemorative stamp ranges for each year
fn get_commemorative_range(year: i32) -> Option<(u32, u32)> {
    match year {
        1955 => Some((1064, 1072)), // 1955 Commemoratives: Fort Duquesne, Pennsylvania Academy, etc.
        1954 => Some((1026, 1053)),
        1956 => Some((1075, 1096)),
        1957 => Some((1100, 1121)),
        1958 => Some((1141, 1160)),
        1959 => Some((1183, 1204)),
        1960 => Some((1227, 1248)),
        1930 => Some((690, 700)),
        _ => None,
    }
}

/// Check if a stamp is commemorative for the given year
pub fn is_commemorative_for_year(stamp: &Stamp, year: i32) -> bool {
    if let Some((min, max)) = get_commemorative_range(year) {
        if let Some(sg_str) = stamp.attrs.get("sg_number") {
            let trimmed = sg_str.trim();
            if let Ok(num) = trimmed
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse::<u32>()
            {
                return num >= min && num <= max;
            }
        }
    }
    false
}

/// Alternative: check if this is from a commemorative-focused series based on name
pub fn is_commemorative_series(series_name: &str) -> bool {
    let lower = series_name.to_lowercase();
    lower.contains("commemorative") || 
    lower.contains("commemoratives") ||
    lower == "1955" ||  // The mixed series for 1955
    lower == "1957" ||
    lower == "1930"
}

/// Check if a stamp is a "special" stamp (Certified Mail, Special Handling, etc.)
pub fn is_special_stamp(stamp: &Stamp) -> bool {
    if let Some(sg) = stamp.attrs.get("sg_number") {
        let sg_lower = sg.to_lowercase();
        sg_lower.starts_with("fa")
            || sg_lower.starts_with("qe")
            || sg_lower.starts_with("jx")
            || sg_lower.starts_with("mx")
    } else {
        false
    }
}

/// Helper to clean text for single-line output
fn clean_text(text: &str) -> String {
    text.lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Emit the header pages with government information
pub fn emit_header_pages<W: Write>(out: &mut W, year: i32) -> std::io::Result<()> {
    // Now emit the informational pages
    writeln!(out, "PAGE_START")?;
    writeln!(
        out,
        "PAGE_TEXT_CENTRE (HB 14 \"United States Postage Stamps of {}\")",
        year
    )?;
    writeln!(out, "PAGE_TEXT_CENTRE (HN 11 \"Commemorative Issues\")")?;
    writeln!(out)?;
    writeln!(out, "PAGE_VSPACE (8.7)")?;
    writeln!(
        out,
        "PAGE_TEXT_CENTRE (HB 10 \"Government Administration\")"
    )?;
    writeln!(
        out,
        "PAGE_TEXT_CENTRE (HN 9 \"  President : Dwight David Eisenhower\")"
    )?;
    writeln!(
        out,
        "PAGE_TEXT_CENTRE (HN 9 \"  Postmaster General : Arthur E. Summerfield\")"
    )?;
    writeln!(out, "PAGE_VSPACE (2.0)")?;
    writeln!(out, "PAGE_TEXT_CENTRE (HB 10 \"Postal Rates ({})\")", year)?;
    writeln!(
        out,
        "PAGE_TEXT_CENTRE (HN 9 \"  Domestic Letter Rate : 3¢ per oz.\")"
    )?;
    writeln!(out, "PAGE_TEXT_CENTRE (HN 9 \"  Postcard Rate : 2¢\")")?;
    writeln!(out, "PAGE_TEXT_CENTRE (HN 9 \"  Airmail Rate : 2¢\")")?;
    writeln!(out, "PAGE_VSPACE (2.0)")?;
    writeln!(out, "PAGE_TEXT_CENTRE (HB 10 \"Technical Specifications\")")?;
    writeln!(
        out,
        "PAGE_TEXT_CENTRE (HN 9 \"  Printing Method : Rotary Press\")"
    )?;
    writeln!(
        out,
        "PAGE_TEXT_CENTRE (HN 9 \"  Electric Eye Plates : Standard\")"
    )?;
    writeln!(
        out,
        "PAGE_TEXT_CENTRE (HN 9 \"  Pane Configuration : 100 stamps per pane\")"
    )?;
    writeln!(out, "PAGE_VSPACE (5.0)")?;

    Ok(())
}

/// Emit commemorative stamp pages with grid layout
pub fn emit_commemorative_pages<W: Write>(out: &mut W, stamps: &[Stamp]) -> std::io::Result<()> {
    let mut _page_num = 2;
    let mut i = 0;

    while i < stamps.len() {
        writeln!(out, "PAGE_START")?;
        writeln!(out, "PAGE_VSPACE (3.0)")?;
        writeln!(out, "ROW_START_ES (HN 6 0.2)")?;
        writeln!(out)?;
        writeln!(out, "COLOR_STAMP_BORDER(black)")?;
        writeln!(out, "COLOR_STAMP_BACKGROUND(black)")?;
        writeln!(out)?;
        writeln!(out, "ROW_ALIGN_TOP")?;

        // Process up to 2 stamps per image row
        for _ in 0..2 {
            if i >= stamps.len() {
                break;
            }
            let stamp = &stamps[i];

            if let (Some(w), Some(h)) = (stamp.est_width_mm, stamp.est_height_mm) {
                let title = if let Some(desc) = stamp.attrs.get("description") {
                    if !desc.trim().is_empty() {
                        clean_text(desc)
                    } else {
                        stamp
                            .attrs
                            .get("sg_number")
                            .map(|s| format!("Scott {}", s))
                            .unwrap_or_else(|| "Stamp".to_string())
                    }
                } else {
                    stamp
                        .attrs
                        .get("sg_number")
                        .map(|s| format!("Scott {}", s))
                        .unwrap_or_else(|| "Stamp".to_string())
                };

                let denom = stamp
                    .attrs
                    .get("denomination")
                    .map(|s| s.as_str())
                    .unwrap_or("3");
                writeln!(
                    out,
                    "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"{}\" \"Perf 10½ x 11\" \"Horizontal\")",
                    w, h, stamp.image_path, denom
                )?;
                writeln!(out, "STAMP_HEADING (HB 9 \"{}\")", title)?;
            }
            i += 1;
        }

        // Description row
        writeln!(out, "PAGE_VSPACE (-5.0)")?;
        writeln!(out, "ROW_START_ES (HN 6 0.2)")?;
        writeln!(out, "ROW_ALIGN_TOP")?;
        writeln!(out, "COLOR_STAMP_BORDER(white)")?;
        writeln!(out, "COLOR_STAMP_BACKGROUND(white)")?;

        // Add descriptions for the stamps we just added
        i -= if i >= 2 { 2 } else { i };
        for j in 0..2 {
            if i + j >= stamps.len() {
                break;
            }
            let stamp = &stamps[i + j];

            // Build detailed description with color, quantity, location, and description
            let mut details = String::new();

            if let Some(desc) = stamp.attrs.get("description") {
                let cleaned = clean_text(desc);
                details.push_str(&format!("{}\n", cleaned));
            }

            if let Some(color) = stamp.attrs.get("color") {
                if !color.is_empty() {
                    details.push_str(&format!("Color: {}\n", color));
                }
            }

            if let Some(qty) = stamp.attrs.get("quantity_printed") {
                if !qty.is_empty() {
                    details.push_str(&format!("Quantity: {}\n", qty));
                }
            }

            if let Some(loc) = stamp.attrs.get("issue_location") {
                if !loc.is_empty() {
                    details.push_str(&format!("Issued: {}", loc));
                }
            }

            if !details.is_empty() {
                let cleaned = clean_text(&details);
                writeln!(
                    out,
                    "STAMP_ADD (28.0 32.0 \"{}\" \"\" \"\" \"\" NO_FRAME)",
                    cleaned
                )?;
            }

            if j == 0 {
                writeln!(out, "COLOR_STAMP_BORDER(black)")?;
                writeln!(out, "COLOR_STAMP_BORDER(white)")?;
            }
        }
        writeln!(out, "COLOR_STAMP_BORDER(black)")?;
        writeln!(out, "COLOR_STAMP_BACKGROUND(black)")?;
        writeln!(out)?;

        i += 2;
    }

    Ok(())
}

/// Emit special stamps page (Certified Mail, Special Handling, etc.)
pub fn emit_special_stamps_page<W: Write>(out: &mut W, stamps: &[Stamp]) -> std::io::Result<()> {
    writeln!(out, "PAGE_START")?;
    writeln!(out, "PAGE_VSPACE (3.0)")?;
    writeln!(out, "ROW_START_ES (HN 6 0.2)")?;
    writeln!(out)?;
    writeln!(out, "COLOR_STAMP_BORDER(black)")?;
    writeln!(out, "COLOR_STAMP_BACKGROUND(black)")?;
    writeln!(out)?;
    writeln!(out, "ROW_ALIGN_TOP")?;

    // Process special stamps (2 per image row)
    for (idx, stamp) in stamps.iter().enumerate() {
        if idx % 2 == 0 && idx > 0 {
            // Move to description row
            writeln!(out, "PAGE_VSPACE (-5.0)")?;
            writeln!(out, "ROW_START_ES (HN 6 0.2)")?;
            writeln!(out, "ROW_ALIGN_TOP")?;
            writeln!(out, "COLOR_STAMP_BORDER(white)")?;
            writeln!(out, "COLOR_STAMP_BACKGROUND(white)")?;

            // Description text
            writeln!(out, "COLOR_STAMP_BORDER(black)")?;
            writeln!(out, "COLOR_STAMP_BACKGROUND(black)")?;
            writeln!(out, "ROW_START_ES (HN 6 0.2)")?;
            writeln!(out, "ROW_ALIGN_TOP")?;
        }

        if let (Some(w), Some(h)) = (stamp.est_width_mm, stamp.est_height_mm) {
            let title = stamp
                .attrs
                .get("sg_number")
                .map(|s| format!("Scott {}", s))
                .unwrap_or_else(|| "Special Stamp".to_string());
            let denom = stamp
                .attrs
                .get("denomination")
                .map(|s| s.as_str())
                .unwrap_or("var");
            writeln!(
                out,
                "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"{}\" \"Perf 10½ x 11\" \"Horizontal\")",
                w, h, stamp.image_path, denom
            )?;
            writeln!(out, "STAMP_HEADING (HB 9 \"{}\")", title)?;
        }
    }

    writeln!(out, "PAGE_VSPACE (-5.0)")?;
    writeln!(out, "ROW_START_ES (HN 6 0.2)")?;
    writeln!(out, "ROW_ALIGN_TOP")?;
    writeln!(out, "COLOR_STAMP_BORDER(white)")?;
    writeln!(out, "COLOR_STAMP_BACKGROUND(white)")?;

    for stamp in stamps {
        let mut details = String::new();

        if let Some(desc) = stamp.attrs.get("description") {
            let cleaned = clean_text(desc);
            details.push_str(&format!("{}\n", cleaned));
        }

        if let Some(color) = stamp.attrs.get("color") {
            if !color.is_empty() {
                details.push_str(&format!("Color: {}\n", color));
            }
        }

        if let Some(qty) = stamp.attrs.get("quantity_printed") {
            if !qty.is_empty() {
                details.push_str(&format!("Quantity: {}\n", qty));
            }
        }

        if let Some(loc) = stamp.attrs.get("issue_location") {
            if !loc.is_empty() {
                details.push_str(&format!("Issued: {}", loc));
            }
        }

        if !details.is_empty() {
            let cleaned = clean_text(&details);
            writeln!(
                out,
                "STAMP_ADD (28.0 32.0 \"{}\" \"\" \"\" \"\" NO_FRAME)",
                cleaned
            )?;
        }
    }
    writeln!(out, "COLOR_STAMP_BORDER(black)")?;
    writeln!(out, "COLOR_STAMP_BACKGROUND(black)")?;
    writeln!(out)?;

    Ok(())
}

/// Emit summary page with collecting information
pub fn emit_summary_page<W: Write>(
    out: &mut W,
    year: i32,
    stamps: &[Stamp],
) -> std::io::Result<()> {
    writeln!(out, "PAGE_START")?;
    writeln!(out, "PAGE_VSPACE (5.0)")?;
    writeln!(
        out,
        "PAGE_TEXT_Centre (HB 12 \"{} Commemorative Issues - Summary\")",
        year
    )?;
    writeln!(out, "PAGE_VSPACE (8.0)")?;
    writeln!(out, "PAGE_TEXT (HB 10 \"Quick Reference\")")?;
    writeln!(out, "PAGE_RULE_H (0.5 0.0 5.0)")?;
    writeln!(out, "PAGE_TEXT (HN 8 \"  Scott #  Subject\")")?;
    writeln!(out, "PAGE_RULE_H (0.5 0.0 5.0)")?;

    for stamp in stamps {
        if let Some(sg) = stamp.attrs.get("sg_number") {
            let subject = stamp
                .attrs
                .get("description")
                .map(|s| clean_text(s))
                .unwrap_or_else(|| "Commemorative".to_string());
            // Truncate long subjects
            let subject = if subject.len() > 40 {
                format!("{}...", &subject[..37])
            } else {
                subject
            };
            writeln!(out, "PAGE_TEXT (HN 8 \"  {}   {}\")", sg, subject)?;
        }
    }

    writeln!(out, "PAGE_RULE_H (0.5 0.0 5.0)")?;
    writeln!(out, "PAGE_VSPACE (5.0)")?;
    writeln!(out, "PAGE_TEXT (HB 10 \"Collecting Notes:\")")?;
    writeln!(
        out,
        "PAGE_TEXT (HN 9 \"• All {} commemoratives are printed by Rotary Press\")",
        year
    )?;
    writeln!(out, "PAGE_TEXT (HN 9 \"• No fluorescent tagging\")")?;
    writeln!(out, "PAGE_TEXT (HN 9 \"• Standard perforation 10½ x 11\")")?;
    writeln!(
        out,
        "PAGE_TEXT (HN 9 \"• High print runs suggest good availability\")"
    )?;
    writeln!(
        out,
        "PAGE_TEXT (HN 9 \"• Mint condition sets very collectible\")"
    )?;

    Ok(())
}
