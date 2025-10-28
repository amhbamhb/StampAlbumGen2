//! # Album Utilities Module
//!
//! This module contains shared utility functions used by both album generation
//! modules (album_globals and stampworld). These functions are identical across
//! both modules and have been consolidated here to eliminate duplication.
//!
//! ## Functions
//!
//! - `clean_text`: Cleans problematic Unicode and formatting from text for AlbumEasy output
//! - `sanitize_filename`: Sanitizes filenames by replacing filesystem-incompatible characters
//! - `write_globals`: Writes global album settings to the output
//! - `calculate_stamp_dimensions`: Calculates appropriate stamp dimensions based on series
//! - `load_stamps_from_db`: Loads stamp data from SQLite database

use regex::Regex;
use std::io::Write;
// Use image utilities to measure actual image dimensions where possible
use crate::flagdb;
use crate::image_utils::{
    calc_mm_per_pixel_embedded, estimate_real_dimensions, get_image_dimensions,
};
use std::process::Command;

/// Cleans problematic Unicode and formatting from text for AlbumEasy output
pub fn clean_text(s: &str) -> String {
    // Remove 'Â' or 'â' before fractions/currency (UTF-8 encoding artifacts)
    let re = Regex::new(r"[Ââ]([½¼¾⅓⅔⅛⅜⅝⅞£€$¥])").unwrap();
    let mut s = re.replace_all(s, "$1").to_string();

    // Replace stray 'Â' or 'â' with nothing
    s = s.replace('â', "");
    s = s.replace('Â', "");

    // REVERSE LOGIC: Convert Unicode fractions BACK to ASCII for AlbumEasy compatibility
    // First, handle fractions that need spacing (e.g., 11⅝ → 11 5/8)
    let fraction_chars = ["½", "¼", "¾", "⅓", "⅔", "⅛", "⅜", "⅝", "⅞"];
    let fraction_replacements = [
        "1/2", "1/4", "3/4", "1/3", "2/3", "1/8", "3/8", "5/8", "7/8",
    ];

    for (fraction_char, replacement) in fraction_chars.iter().zip(fraction_replacements.iter()) {
        // Use regex to add space between number and fraction (e.g., 11⅝ → 11 5/8)
        let pattern = format!(r"(\d)({})", fraction_char);
        let re = Regex::new(&pattern).unwrap();
        s = re
            .replace_all(&s, format!("$1 {}", replacement))
            .to_string();
    }

    // Handle standalone fractions (not preceded by numbers)
    s = s.replace("½", "1/2");
    s = s.replace("¼", "1/4");
    s = s.replace("¾", "3/4");
    s = s.replace("⅓", "1/3");
    s = s.replace("⅔", "2/3");
    s = s.replace("⅛", "1/8");
    s = s.replace("⅜", "3/8");
    s = s.replace("⅝", "5/8");
    s = s.replace("⅞", "7/8");

    // Replace double quotes
    s = s.replace('"', "'");

    // Replace newlines and tabs with spaces
    s = s.replace('\n', " ");
    s = s.replace('\r', " ");
    s = s.replace('\t', " ");

    // Replace multiple spaces with single space
    let re_spaces = Regex::new(r"\s+").unwrap();
    s = re_spaces.replace_all(&s, " ").to_string();

    // Trim whitespace
    s = s.trim().to_string();

    // Replace non-ASCII except allowed symbols with space
    s.chars()
        .map(|c| match c {
            '£' | '€' | '$' | '¥' | '½' | '¼' | '¾' | '⅓' | '⅔' | '⅛' | '⅜' | '⅝' | '⅞' => {
                c
            }
            _ if c.is_ascii() => c,
            _ => ' ',
        })
        .collect()
}

/// Sanitizes a filename by replacing filesystem-incompatible characters
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| {
            match c {
                '*' => 's',  // star -> s
                '?' => 'q',  // question -> q
                '<' => 'l',  // less -> l
                '>' => 'g',  // greater -> g
                ':' => 'c',  // colon -> c
                '"' => 'd',  // double quote -> d
                '|' => 'p',  // pipe -> p
                '\\' => 'b', // backslash -> b
                '/' => 'f',  // forward slash -> f
                _ if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' => c,
                _ => '_', // replace other non-alphanumeric with underscore
            }
        })
        .collect()
}

/// Writes global album settings to the provided writer
pub fn write_globals<W: Write>(
    writer: &mut W,
    album_name: &str,
    conn_opt: Option<&rusqlite::Connection>,
    start_year: Option<i32>,
    end_year: Option<i32>,
) {
    // Get current date/time and user information using chrono (cross-platform)
    // Use UTC for the canonical generated timestamp and Local for the host localtime
    use chrono::{Local, Utc};
    let datetime_str = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let localdatetime = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Get user information (cross-platform)
    let username = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());
    let hostname = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string());

    // Get command line arguments and present them as the full CLI used
    let args: Vec<String> = std::env::args().collect();
    let command_line = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "No arguments".to_string()
    };

    // Load config so we can emit the values used for header formatting toggles
    let cfg = crate::config::load_config("config.toml");
    let cfg_print_flag_cover = cfg.print_flag_cover.unwrap_or(true);
    let cfg_include_design = cfg.include_design.unwrap_or(true);
    let cfg_include_engraving = cfg.include_engraving.unwrap_or(true);
    let cfg_row_start_es = cfg.row_start_es_enabled.unwrap_or(true);

    // Build AlbumEasy header lines dynamically from config so the emitted
    // header matches the layout math used later in the generator.
    let page_w = cfg.album_page_width_mm.unwrap_or(216.0);
    let page_h = cfg.album_page_height_mm.unwrap_or(279.4);
    let top_margin = cfg.page_top_margin_mm.unwrap_or(10.0);
    let bottom_margin = cfg.page_bottom_margin_mm.unwrap_or(10.0);
    let left_margin = cfg.page_left_margin_mm.unwrap_or(10.0);
    let right_margin = cfg.page_right_margin_mm.unwrap_or(10.0);
    let border_outer = cfg.page_border_outer.unwrap_or(0.5);
    let border_inner1 = cfg.page_border_inner1.unwrap_or(0.1);
    let border_inner2 = cfg.page_border_inner2.unwrap_or(0.0);
    let border_spacing = cfg.page_border_spacing.unwrap_or(3.0);
    let pages_spacing_h = cfg.album_pages_spacing_h.unwrap_or(0.0);
    let pages_spacing_v = cfg.album_pages_spacing_v.unwrap_or(0.0);
    let img_new_w = cfg.album_stamp_img_new_setting_w_mm.unwrap_or(0.0);
    let img_new_h = cfg.album_stamp_img_new_setting_h_mm.unwrap_or(0.0);
    let img_preserve = cfg
        .album_stamp_img_new_setting_preserve_aspect
        .unwrap_or(true);
    let stamp_box_adj = cfg.stamp_box_adjust.unwrap_or(3.0);

    let mut lines: Vec<String> = Vec::new();
    lines.push(
        "################################################################################"
            .to_string(),
    );
    lines.push(format!("# {}", album_name));
    lines.push(
        "################################################################################"
            .to_string(),
    );
    lines.push(format!(
        "# Generated: UTC: {} localtime : {}",
        datetime_str, localdatetime
    ));
    lines.push(format!("# User: {}@{}", username, hostname));
    lines.push(format!("# OS: {}", std::env::consts::OS));
    lines.push(format!("# Architecture: {}", std::env::consts::ARCH));
    lines.push(format!(
        "# Working Directory: {}",
        std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .display()
    ));
    lines.push(format!("# Command Line: {}", command_line));
    // Emit a rich set of Config: lines so the generated header captures all
    // config toggles used for this run. This mirrors the legacy header
    // format and makes it easier to trace generation params.
    let cfg_include_wikipedia = cfg
        .include_wikipedia_content
        .unwrap_or(cfg.fetch_wikipedia.unwrap_or(false));
    let cfg_allow_single_large = cfg.allow_single_large_stamp_per_page.unwrap_or(true);
    let cfg_preserve_aspect = cfg.preserve_aspect.unwrap_or(img_preserve);
    let cfg_emit_placeholder_diags = cfg.emit_placeholder_diagnostics.unwrap_or(false);
    let cfg_detailed_page_fit = cfg
        .detailed_page_fit
        .unwrap_or(cfg.detailed_page_fit_diagnostics.unwrap_or(false));

    lines.push(format!(
        "# Config: print_flag_cover={}",
        cfg_print_flag_cover
    ));
    lines.push(format!("# Config: include_design={}", cfg_include_design));
    lines.push(format!(
        "# Config: include_engraving={}",
        cfg_include_engraving
    ));
    lines.push(format!(
        "# Config: include_watermark={}",
        cfg.include_watermark.unwrap_or(true)
    ));
    lines.push(format!(
        "# Config: include_perforation={}",
        cfg.include_perforation.unwrap_or(true)
    ));
    lines.push(format!(
        "# Config: include_catalog_numbers={}",
        cfg.include_catalog_numbers.unwrap_or(true)
    ));
    lines.push(format!(
        "# Config: include_series={}",
        cfg.include_series.unwrap_or(true)
    ));
    lines.push(format!(
        "# Config: include_country={}",
        cfg.include_country.unwrap_or(true)
    ));
    lines.push(format!(
        "# Config: include_date={}",
        cfg.include_date.unwrap_or(true)
    ));
    lines.push(format!(
        "# Config: include_wikipedia_content={}",
        cfg_include_wikipedia
    ));
    lines.push(format!(
        "# Config: row_start_es_enabled={}",
        cfg_row_start_es
    ));
    lines.push(format!(
        "# Config: allow_single_large_stamp={}",
        cfg_allow_single_large
    ));
    lines.push(format!("# Config: preserve_aspect={}", cfg_preserve_aspect));
    lines.push(format!(
        "# Config: emit_placeholder_diagnostics={}",
        cfg_emit_placeholder_diags
    ));
    lines.push(format!(
        "# Config: detailed_page_fit={}",
        cfg_detailed_page_fit
    ));
    lines.push(format!(
        "# Config: stamp_heading_in_row_height={}",
        cfg.stamp_heading_in_row_height.unwrap_or(false)
    ));
    lines.push(format!(
        "# Config: pages_vspace_in_row_height={}",
        cfg.pages_vspace_in_row_height.unwrap_or(false)
    ));
    lines.push(format!(
        "# Config_SAFETY: packing_horizontal_safety_mm={:?} packing_vertical_safety_mm={:?}",
        cfg.packing_horizontal_safety_mm, cfg.packing_vertical_safety_mm
    ));
    lines.push("".to_string());
    lines.push(format!(
        "ALBUM_PAGES_SIZE  ({:.1} {:.1}) # Page size in mm (width height)",
        page_w, page_h
    ));
    lines.push("".to_string());
    // Emit margins as (left right top bottom) to match internal layout assumptions
    lines.push(format!(
        "ALBUM_PAGES_MARGINS  ({:.1} {:.1} {:.1} {:.1} )",
        left_margin, right_margin, top_margin, bottom_margin
    ));
    lines.push("".to_string());
    lines.push(format!(
        "ALBUM_PAGES_BORDER ({:.2} {:.2} {:.2} {:.2})    #outer inner1 inner2 spacing",
        border_outer, border_inner1, border_inner2, border_spacing
    ));
    lines.push("#ALBUM_PAGES_BORDER3  (0.1 0.0 0.0 3.0) #single line border".to_string());
    lines.push("".to_string());
    // Use ALBUM_PAGES_HEADER with configurable font/size/position
    let title_font = cfg
        .album_pages_title_font
        .clone()
        .unwrap_or_else(|| "HN".to_string());
    let title_size = cfg.album_pages_title_size.unwrap_or(10.0);
    let title_pos = cfg
        .album_pages_header_position
        .clone()
        .unwrap_or_else(|| "C".to_string());
    // Build title including optional year range. Logic:
    // - If both start and end provided: if equal -> single year, else show "start - end"
    // - If only start provided: append "start - <max year from DB or current year>"
    // - If only end provided: prepend "<min year from DB or current year> - end"
    // - If neither provided: just album_name
    use chrono::Datelike;
    let mut title_body = album_name.to_string();
    // Helper to query min/max year from DB if available
    let db_min_year = conn_opt.and_then(|c| {
        c.query_row("SELECT MIN(year) FROM stamps", [], |r| {
            r.get::<_, Option<i32>>(0)
        })
        .ok()
        .flatten()
    });
    let db_max_year = conn_opt.and_then(|c| {
        c.query_row("SELECT MAX(year) FROM stamps", [], |r| {
            r.get::<_, Option<i32>>(0)
        })
        .ok()
        .flatten()
    });
    let current_year = chrono::Utc::now().year();
    match (start_year, end_year) {
        (Some(s), Some(e)) => {
            if s == e {
                title_body = format!("{} {}", album_name, s);
            } else {
                title_body = format!("{} - {} - {}", album_name, s, e);
            }
        }
        (Some(s), None) => {
            let maxy = db_max_year.unwrap_or(current_year);
            if s == maxy {
                title_body = format!("{} {}", album_name, s);
            } else {
                title_body = format!("{} - {} - {}", album_name, s, maxy);
            }
        }
        (None, Some(e)) => {
            let miny = db_min_year.unwrap_or(current_year);
            if miny == e {
                title_body = format!("{} {}", album_name, e);
            } else {
                title_body = format!("{} - {} - {}", album_name, miny, e);
            }
        }
        (None, None) => {
            // leave album_name as-is
        }
    }
    lines.push(format!(
        "ALBUM_PAGES_HEADER ({} {} {} \"{}\")",
        title_font, title_size as i32, title_pos, title_body
    ));
    lines.push("".to_string());
    lines.push(format!(
        "ALBUM_PAGES_SPACING ({:.1} {:.1})",
        pages_spacing_h, pages_spacing_v
    ));
    lines.push("".to_string());
    // Emit the per-album image aspect ratio flag based on config
    lines.push(format!(
        "ALBUM_STAMP_IMG_ASPECT_RATIO ({}) ",
        cfg_preserve_aspect
    ));
    // Emit color overrides if present in config
    if let Some(ref c) = cfg.color_album_border {
        lines.push(format!("COLOR_ALBUM_BORDER({})", c));
    }
    if let Some(ref c) = cfg.color_album_footer {
        lines.push(format!("COLOR_ALBUM_FOOTER({})", c));
    }
    if let Some(ref c) = cfg.color_album_header {
        lines.push(format!("COLOR_ALBUM_HEADER({})", c));
    }
    if let Some(ref c) = cfg.color_album_title {
        lines.push(format!("COLOR_ALBUM_TITLE({})", c));
    }
    if let Some(ref c) = cfg.color_album_margin_txt {
        lines.push(format!("COLOR_ALBUM_MARGIN_TXT({})", c));
    }
    // Stamp color overrides
    if let Some(ref c) = cfg.color_stamp_border {
        lines.push(format!("COLOR_STAMP_BORDER({})", c));
    }
    if let Some(ref c) = cfg.color_stamp_heading {
        lines.push(format!("COLOR_STAMP_HEADING({})", c));
    }
    if let Some(ref c) = cfg.color_stamp_text {
        lines.push(format!("COLOR_STAMP_TEXT({})", c));
    }
    if let Some(ref c) = cfg.color_stamp_background {
        lines.push(format!("COLOR_STAMP_BACKGROUND({})", c));
    }
    lines.push("".to_string());
    lines.push("#ALBUM_PAGES_NUMBER (TN 10 R)".to_string());
    // Parameterize footer page number format from config
    if cfg.album_pages_footer_number_enabled.unwrap_or(true) {
        let foot_font = cfg
            .album_pages_footer_number_font
            .clone()
            .unwrap_or_else(|| "HN".to_string());
        let foot_size = cfg.album_pages_footer_number_size.unwrap_or(10.0);
        let foot_pos = cfg
            .album_pages_footer_number_position
            .clone()
            .unwrap_or_else(|| "C".to_string());
        let foot_start = cfg.album_pages_footer_number_start.unwrap_or(1);
        let foot_before = cfg
            .album_pages_footer_number_before
            .clone()
            .unwrap_or_else(|| "Page ".to_string());
        let foot_after = cfg
            .album_pages_footer_number_after
            .clone()
            .unwrap_or_else(|| " of $PAGES$".to_string());
        lines.push(format!(
            "ALBUM_PAGES_FOOTER_NUMBER ({} {} {} {} \"{}\" \"{}\")",
            foot_font, foot_size as i32, foot_pos, foot_start, foot_before, foot_after
        ));
    }
    lines.push("".to_string());
    lines.push(format!(
        "ALBUM_STAMP_IMG_NEW_SETTING ({:.1} {:.1} {})",
        img_new_w, img_new_h, img_preserve
    ));
    lines.push("".to_string());
    lines.push("ROW_ALIGN_MIDDLE".to_string());
    lines.push("".to_string());
    lines.push("# The stamp boxes in this album are sized approximately".to_string());
    lines.push("# 4mm larger than the actual stamps giving a 2mm border".to_string());
    lines.push("# To have the boxes drawn the actual stamp size use".to_string());
    lines.push("#STAMP_BOXES_SIZE_ADJUST(-4.0)".to_string());
    lines.push("# To increase the border, e.g by 1mm use STAMP_BOXES_SIZE_ADJUST(2.0)".to_string());
    lines.push(format!("STAMP_BOXES_SIZE_ADJUST ({:.1})", stamp_box_adj));
    // Emit each global line. Ensure we always insert a PAGE_START before any
    // PAGE_ command (PAGE_VSPACE, PAGE_TEXT_CENTRE, etc.) unless the last
    // emitted line was already a PAGE_START. This guarantees the generated
    // AlbumEasy file always begins page sections correctly.
    let mut last_was_page_start = false;
    for line in lines.iter() {
        let trimmed = line.trim();
        if trimmed.starts_with("PAGE_") {
            if !last_was_page_start {
                let _ = writeln!(writer, "PAGE_START");
                // Ensure page sections begin with a row start when enabled
                if cfg_row_start_es {
                    // Emit ROW_START_ES using config params
                    let rs_font = cfg
                        .row_start_es_font
                        .clone()
                        .unwrap_or_else(|| "HN".to_string());
                    let rs_size = cfg.row_start_es_size.unwrap_or(6.0);
                    let rs_spacing = cfg.row_start_es_spacing.unwrap_or(0.1);
                    let rs_vspace = cfg.row_start_es_vspace.unwrap_or(5.0);
                    let _ = writeln!(
                        writer,
                        "ROW_START_ES ({} {} {:.1} {:.1})",
                        rs_font, rs_size as i32, rs_spacing, rs_vspace
                    );
                }
                last_was_page_start = true;
            }
            let _ = writeln!(writer, "{}", line);
            // keep last_was_page_start = true
            continue;
        }

        // If the line itself is an explicit PAGE_START, write it and set the flag
        if trimmed == "PAGE_START" {
            let _ = writeln!(writer, "{}", line);
            // Follow PAGE_START with a ROW_START_ES so the page has an initialized row
            if cfg_row_start_es {
                let rs_font = cfg
                    .row_start_es_font
                    .clone()
                    .unwrap_or_else(|| "HN".to_string());
                let rs_size = cfg.row_start_es_size.unwrap_or(6.0);
                let rs_spacing = cfg.row_start_es_spacing.unwrap_or(0.1);
                let rs_vspace = cfg.row_start_es_vspace.unwrap_or(5.0);
                let _ = writeln!(
                    writer,
                    "ROW_START_ES ({} {} {:.1} {:.1})",
                    rs_font, rs_size as i32, rs_spacing, rs_vspace
                );
            }
            last_was_page_start = true;
            continue;
        }

        // Non-page lines clear the page-start flag
        let _ = writeln!(writer, "{}", line);
        last_was_page_start = false;
    }

    // Emit an optional cover page containing a large title and (when available)
    // a flag image. We attempt to copy a flag into Albums/{Album}/images/flag_placeholder.png
    // using the `flagdb` helper. If a flag image cannot be found or downloaded,
    // still emit a textual cover so the album begins with a title page.
    if cfg_print_flag_cover {
        // Write a simple cover block after the standard globals so it appears
        // below the STAMP_BOXES_SIZE_ADJUST section as requested.
        let album_images_dir = format!("Albums/{}/images", album_name.replace(' ', "-"));
        // Try to fetch/copy a flag image (size w320). If this fails we ignore the error.
        let mut flag_path_opt: Option<String> = None;
        match flagdb::copy_flag_for_album(album_name, &album_images_dir, "w320") {
            Ok(p) => {
                // convert to relative path under album dir if possible
                if let Some(fname) = std::path::Path::new(&p)
                    .file_name()
                    .and_then(|s| s.to_str())
                {
                    flag_path_opt = Some(format!("images/{}", fname));
                } else {
                    flag_path_opt = Some(p);
                }
            }
            Err(_) => {
                // no flag available; proceed with textual cover only
            }
        }

        // Emit a cover page header
        let _ = writeln!(
            writer,
            "################################################################################"
        );
        let _ = writeln!(writer, "### {} ###", album_name);
        let _ = writeln!(writer, "PAGE_START");
        // Ensure cover pages also begin with a row start so the layout is initialized
        if cfg_row_start_es {
            let rs_font = cfg
                .row_start_es_font
                .clone()
                .unwrap_or_else(|| "HN".to_string());
            let rs_size = cfg.row_start_es_size.unwrap_or(6.0);
            let rs_spacing = cfg.row_start_es_spacing.unwrap_or(0.1);
            let rs_vspace = cfg.row_start_es_vspace.unwrap_or(5.0);
            let _ = writeln!(
                writer,
                "ROW_START_ES ({} {} {:.1} {:.1})",
                rs_font, rs_size as i32, rs_spacing, rs_vspace
            );
        }
        // If we have a flag image, emit it as a large image entry on the cover.
        if let Some(fp) = flag_path_opt.as_ref() {
            // Use flag_cover_* config values if provided, otherwise fall back to
            // the previously hard-coded numbers so behavior is unchanged.
            let top_vspace = cfg.flag_cover_page_vspace_top.unwrap_or(40.0);
            let img_w = cfg.flag_cover_img_w_mm.unwrap_or(160.0);
            let img_h = cfg.flag_cover_img_h_mm.unwrap_or(80.0);
            let after_vspace = cfg.flag_cover_image_vspace_after.unwrap_or(12.0);

            // Emit a large image placeholder (AlbumEasy consumer will scale)
            let _ = writeln!(writer, "PAGE_VSPACE ({:.1})", top_vspace);

            // Use configured ROW_START_ES params for cover image block
            let rs_font = cfg
                .row_start_es_font
                .clone()
                .unwrap_or_else(|| "HN".to_string());
            let rs_size = cfg.row_start_es_size.unwrap_or(6.0);
            let rs_spacing = cfg.row_start_es_spacing.unwrap_or(0.1);
            let rs_vspace = cfg.row_start_es_vspace.unwrap_or(5.0);
            let _ = writeln!(
                writer,
                "ROW_START_ES ({} {} {:.1} {:.1})",
                rs_font, rs_size as i32, rs_spacing, rs_vspace
            );
            let _ = writeln!(writer, "# Cover flag image for album");
            let flag_cover_background_color = cfg
                .flag_cover_background_color
                .clone()
                .unwrap_or_else(|| "white".to_string());
            let _ = writeln!(
                writer,
                "COLOR_STAMP_BACKGROUND({})",
                flag_cover_background_color
            );
            let _ = writeln!(writer, "# Change background for flag if needed");

            let _ = writeln!(
                writer,
                "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"\" \"\" \"\" NO_FRAME)",
                img_w, img_h, fp
            );
            let _ = writeln!(
                writer,
                "# Change the color background back to default after flag"
            );
            let _ = writeln!(
                writer,
                "COLOR_STAMP_BACKGROUND({})",
                cfg.color_stamp_background
                    .clone()
                    .unwrap_or_else(|| "white".to_string())
            );

            let _ = writeln!(writer, "PAGE_VSPACE ({:.1})", after_vspace);

            // Title parameters for image cover
            let title_size = cfg.flag_cover_image_title_size.unwrap_or(32.0);
            // scale is not directly used by AlbumEasy; keep for downstream consumers
            let _title_scale = cfg.flag_cover_image_title_scale.unwrap_or(1.0);
            let _ = writeln!(
                writer,
                "PAGE_TEXT_CENTRE (TB {} \"{}\" 1.0)",
                title_size as i32,
                album_name.replace("\\n", " ")
            );
        } else {
            // Text-only cover: large centered title
            let top_vspace = cfg.flag_cover_text_only_vspace_top.unwrap_or(24.0);
            let after_vspace = cfg.flag_cover_text_only_vspace_after.unwrap_or(18.0);
            let title_size = cfg.flag_cover_text_only_title_size.unwrap_or(48.0);
            let _title_scale = cfg.flag_cover_text_only_title_scale.unwrap_or(1.0);

            let _ = writeln!(writer, "PAGE_VSPACE ({:.1})", top_vspace);
            let rs_font = cfg
                .row_start_es_font
                .clone()
                .unwrap_or_else(|| "HN".to_string());
            let rs_size = cfg.row_start_es_size.unwrap_or(6.0);
            let rs_spacing = cfg.row_start_es_spacing.unwrap_or(0.1);
            let rs_vspace = cfg.row_start_es_vspace.unwrap_or(5.0);
            let _ = writeln!(
                writer,
                "ROW_START_ES ({} {} {:.1} {:.1})",
                rs_font, rs_size as i32, rs_spacing, rs_vspace
            );
            let _ = writeln!(
                writer,
                "PAGE_TEXT_CENTRE (TB {} \"{}\" 1.0)",
                title_size as i32,
                album_name.replace("\\n", " ")
            );
            let _ = writeln!(writer, "PAGE_VSPACE ({:.1})", after_vspace);
        }
        // After cover, emit a page separator so normal globals follow
        let _ = writeln!(
            writer,
            "################################################################################"
        );
    }
}

/// Calculates appropriate stamp box dimensions based on image dimensions
///
/// This function implements a sophisticated stamp sizing algorithm that:
/// 1. Loads the actual image dimensions from disk
/// 2. Uses embedded reference image for consistent mm/px ratios
/// 3. Applies series-specific scaling factors for precise control
/// 4. Applies reasonable bounds to ensure proper album layout
///
/// # Arguments
///
/// * `image_path` - Relative path to the stamp image (e.g., "L-s.jpg")
/// * `album_name` - Name of the album for path construction
///
/// # Returns
///
/// Tuple of (width_mm, height_mm) representing the calculated stamp dimensions
///
/// # Algorithm
///
/// 1. **Image Loading**: Attempts to load actual pixel dimensions from the image file
/// 2. **Reference Calculation**: Uses embedded B4-s.jpg reference for consistent mm/px ratios
/// 3. **Series Scaling**: Applies series-specific scaling factors (L/M/N series)
/// 4. **Bounds Checking**: Ensures dimensions fit within album layout constraints
/// 5. **Fallback**: Uses default dimensions if image loading fails
///
/// # Examples
///
/// ```rust
/// // Calculate dimensions for L-series stamp
/// let cfg = stamp_album_gen2::Config::default();
/// let (width, height) = stamp_album_gen2::album_utils::calculate_stamp_dimensions("L-s.jpg", "Canada", &cfg);
/// // Result: (49.0, 21.0) for L-series with ALBUM_STAMP_IMG_ASPECT_RATIO = false
/// ```
///
/// # Scaling Factors Applied
///
/// - **L-series**: (1.25, 2.45) → 49.0mm x 21.0mm target
/// - **M-series**: (1.15, 1.45) → 31.1mm x 24.1mm target
/// - **N-series**: (1.10, 1.40) → 26.1mm x 21.0mm target
pub fn calculate_stamp_dimensions(
    image_path: &str,
    album_name: &str,
    cfg: &crate::config::Config,
) -> (f64, f64) {
    // Preferred approach: use actual image pixel dimensions converted to mm
    // via the embedded reference image. This eliminates reliance on hard-coded
    // series groups (L/M/N/O) and produces sizes based on the real image.

    // Reference real-world dimensions used with the embedded B4-s.jpg
    // Updated via calibration (see scripts/calibrate_mm_per_px.sh)
    // Suggested values computed from user-supplied measurements on 2025-09-21
    const REF_W_MM: f32 = 23.42;
    const REF_H_MM: f32 = 25.93;

    // Resolve candidate filesystem paths for the image. The DB may contain "images/<file>"
    // or just a filename; images are stored under Albums/<Album-Name>/images/<file>.
    // We'll prefer a local path first (image_path as-is), then try the album images dir.
    let mut resolved_image_path = image_path.to_string();
    let album_images_candidate = format!("Albums/{}/{}", album_name.replace(' ', "-"), image_path);
    // If the straight image_path does not exist, use the album images candidate for measurements.
    if !std::path::Path::new(&resolved_image_path).exists()
        && std::path::Path::new(&album_images_candidate).exists()
    {
        resolved_image_path = album_images_candidate;
    }

    // If a calibration CSV is provided, load it and check for a basename match.
    // CSV format: filename,mm_per_px_w,mm_per_px_h
    fn load_calibration_map(path: &str) -> std::collections::HashMap<String, (f32, f32)> {
        let mut m = std::collections::HashMap::new();
        if let Ok(content) = std::fs::read_to_string(path) {
            for (i, line) in content.lines().enumerate() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                if parts.len() >= 3 {
                    if let (Ok(w), Ok(h)) = (parts[1].parse::<f32>(), parts[2].parse::<f32>()) {
                        m.insert(parts[0].to_string(), (w, h));
                    }
                } else {
                    log::warn!("calibration CSV: bad line {}: {}", i + 1, line);
                }
            }
        }
        m
    }

    // Try to find a calibration match by filename (basename)
    if let Some(cal_csv) = cfg.calibration_csv.as_ref() {
        let calib_map = load_calibration_map(cal_csv);
        if let Some(basename) = std::path::Path::new(&resolved_image_path)
            .file_name()
            .and_then(|s| s.to_str())
        {
            if let Some((mm_per_px_w, mm_per_px_h)) = calib_map.get(basename) {
                if let Some((w_mm, h_mm)) =
                    estimate_real_dimensions(&resolved_image_path, *mm_per_px_w, *mm_per_px_h)
                {
                    let mut width = w_mm as f64;
                    let mut height = h_mm as f64;
                    if let Some(scale) = cfg.global_scale {
                        width *= scale as f64;
                        height *= scale as f64;
                    }
                    width = width.clamp(10.0, 120.0);
                    height = height.clamp(10.0, 120.0);
                    return (width, height);
                }
            }
        }
    }

    // Preferred approach: use embedded-reference sizing first (more consistent)
    if let Some((mm_per_px_w, mm_per_px_h)) =
        crate::image_utils::calc_mm_per_pixel_embedded(REF_W_MM, REF_H_MM)
    {
        if let Some((w_mm, h_mm)) =
            estimate_real_dimensions(&resolved_image_path, mm_per_px_w, mm_per_px_h)
        {
            let mut width = w_mm as f64;
            let mut height = h_mm as f64;

            // Enforce reasonable bounds to avoid absurd sizes from mis-detected images
            if width > 120.0 || height > 120.0 {
                log::warn!(
                    "Embedded-reference measured size is unexpectedly large ({:.1} x {:.1} mm) for image '{}'.",
                    width,
                    height,
                    resolved_image_path
                );
            } else {
                // Apply global_scale from provided config if present
                if let Some(scale) = cfg.global_scale {
                    width *= scale as f64;
                    height *= scale as f64;
                }

                // Clamp to expected practical range (older stamps are often ~22-37mm)
                width = width.clamp(10.0, 120.0);
                height = height.clamp(10.0, 120.0);
                return (width, height);
            }
        }
    }

    // If embedded-reference approach failed, try DPI-based sizing
    if let Ok(output) = Command::new("identify")
        .args(&["-format", "%x %y %U", &resolved_image_path])
        .output()
    {
        if output.status.success() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                let parts: Vec<&str> = s.split_whitespace().collect();
                // Handle possible formats from ImageMagick identify:
                let mut dpi_x_opt: Option<f32> = None;
                let mut dpi_y_opt: Option<f32> = None;
                let mut unit_opt: Option<&str> = None;

                if parts.len() >= 3 {
                    if let (Ok(dx), Ok(dy)) = (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
                        dpi_x_opt = Some(dx);
                        dpi_y_opt = Some(dy);
                        unit_opt = Some(parts[2]);
                    } else if parts[0].contains('x') {
                        if let Some((dxs, dys)) = parts[0].split_once('x') {
                            if let (Ok(dx), Ok(dy)) = (dxs.parse::<f32>(), dys.parse::<f32>()) {
                                dpi_x_opt = Some(dx);
                                dpi_y_opt = Some(dy);
                                unit_opt = Some(parts[1]);
                            }
                        }
                    }
                } else if parts.len() >= 2 {
                    if parts[0].contains('x') {
                        if let Some((dxs, dys)) = parts[0].split_once('x') {
                            if let (Ok(dx), Ok(dy)) = (dxs.parse::<f32>(), dys.parse::<f32>()) {
                                dpi_x_opt = Some(dx);
                                dpi_y_opt = Some(dy);
                                unit_opt = Some(parts[1]);
                            }
                        }
                    }
                }

                if let (Some(dpi_x), Some(dpi_y), Some(unit)) = (dpi_x_opt, dpi_y_opt, unit_opt) {
                    let (mm_per_px_w_dpi, mm_per_px_h_dpi) = if unit.to_lowercase().contains("inch")
                        || unit.to_lowercase().contains("pixelsperinch")
                    {
                        (25.4_f32 / dpi_x, 25.4_f32 / dpi_y)
                    } else if unit.to_lowercase().contains("centimeter")
                        || unit.to_lowercase().contains("pixelspercentimeter")
                    {
                        (10.0_f32 / dpi_x, 10.0_f32 / dpi_y)
                    } else {
                        (0.0_f32, 0.0_f32)
                    };

                    if mm_per_px_w_dpi > 0.0 && mm_per_px_h_dpi > 0.0 {
                        if let Some((wpx, hpx)) = get_image_dimensions(&resolved_image_path) {
                            let mut w_mm = (wpx as f32) * mm_per_px_w_dpi;
                            let mut h_mm = (hpx as f32) * mm_per_px_h_dpi;
                            // Apply global_scale if present
                            if let Some(scale) = cfg.global_scale {
                                w_mm *= scale;
                                h_mm *= scale;
                            }
                            let w_mm = w_mm as f64;
                            let h_mm = h_mm as f64;
                            // Clamp and return
                            return (w_mm.clamp(10.0, 120.0), h_mm.clamp(10.0, 120.0));
                        }
                    }
                }
            }
        }
    }

    // If DPI-based approach failed or unknown, attempt to compute mm-per-pixel from the embedded reference image
    if let Some((mm_per_px_w, mm_per_px_h)) = calc_mm_per_pixel_embedded(REF_W_MM, REF_H_MM) {
        // Try to estimate using the actual image file if it exists
        if let Some((w_mm, h_mm)) =
            estimate_real_dimensions(&resolved_image_path, mm_per_px_w, mm_per_px_h)
        {
            let width = w_mm as f64;
            let height = h_mm as f64;

            // If measurements are absurdly large (likely a full-page scan or wrong reference),
            // attempt a DPI-based fallback (already tried), otherwise fall back to conservative default.
            if width > 120.0 || height > 120.0 {
                log::warn!(
                    "Embedded-reference measured size is unexpectedly large ({:.1} x {:.1} mm) for image '{}'. Falling back to default.",
                    width,
                    height,
                    resolved_image_path
                );
                return (28.0_f64, 31.0_f64);
            }

            // Enforce reasonable bounds to avoid absurd sizes
            let width = width.clamp(18.0, 60.0); // between 18mm and 60mm
            let height = height.clamp(14.0, 60.0); // between 14mm and 60mm

            return (width, height);
        }
    }

    // Fallback: if image-based estimation fails, use a conservative default
    // (previous default was 28.0 x 31.0). Keep a single sensible fallback
    // and remove the L/M/N/O series branching entirely.
    let (width, height) = (28.0_f64, 31.0_f64);

    (width, height)
}

/// Loads stamp data from SQLite database
pub fn load_stamps_from_db(db_path: &str) -> Vec<crate::db_utils::StampStub> {
    let conn = crate::db_utils::init_db(db_path).expect("Failed to open database");

    // Load all stamps directly instead of filtering by series
    let mut stmt = conn
        .prepare(
            "SELECT year, month, day, w_number, type, denomination, color, month_num, description, stamps_issued, image_path, remote_image_url, watermark, design, engraving, perforation, series, currency, context FROM stamps ORDER BY sg_number"
        )
        .expect("Failed to prepare statement");

    let stamp_iter = stmt
        .query_map([], |row| {
            // Debug: print raw row data
            // let sg_result: Result<String, _> = row.get(3);
            // let desc_result: Result<String, _> = row.get(8);
            // println!("[DEBUG] Raw row data: sg_number={:?}, description={:?}", sg_result, desc_result);

            Ok(crate::db_utils::StampStub {
                year: row.get::<_, i32>(0).unwrap_or(0),
                month: row.get(1).unwrap_or(String::new()),
                month_num: row.get::<_, i32>(7).unwrap_or(0),
                day: row.get(2).unwrap_or(String::new()),
                sg_number: row.get(3).unwrap_or(String::new()),
                r#type: row.get(4).unwrap_or(String::new()),
                stamp_type: row.get(4).unwrap_or(String::new()),
                denomination: row.get(5).unwrap_or(String::new()),
                color: row.get(6).unwrap_or(String::new()),
                description: row.get(8).unwrap_or(String::new()),
                desc: row.get(8).unwrap_or(String::new()), // description is at index 8
                catalog_num: row.get(3).unwrap_or(String::new()), // Use sg_number for catalog_num
                stamps_issued: crate::db_utils::format_stamps_issued(
                    &row.get::<_, String>(9).unwrap_or(String::new()),
                ), // stamps_issued at index 9
                mint_condition: String::new(),
                unused: String::new(),
                used: String::new(),
                letter_fdc: String::new(),
                currency: row.get(17).unwrap_or(String::new()), // currency at index 17
                image_path: row.get(10).unwrap_or(String::new()), // image_path at index 10
                remote_image_url: row.get(11).unwrap_or(String::new()), // remote_image_url at index 11
                original_image_url: String::new(),
                image_width_px: row
                    .get::<_, Option<i64>>("image_width_px")
                    .ok()
                    .flatten()
                    .map(|v| v as i32),
                image_height_px: row
                    .get::<_, Option<i64>>("image_height_px")
                    .ok()
                    .flatten()
                    .map(|v| v as i32),
                watermark: row.get(12).unwrap_or(String::new()), // watermark at index 12
                design: row.get(13).unwrap_or(String::new()),    // design at index 13
                engraving: row.get(14).unwrap_or(String::new()), // engraving at index 14
                perforation: row.get(15).unwrap_or(String::new()), // perforation at index 15
                series: row.get(16).unwrap_or(String::new()),    // series at index 16
                context: row.get(18).unwrap_or(String::new()),   // context at index 18
                multi_perf: false,
            })
        })
        .expect("Failed to query stamps");

    let mut all_stamps = Vec::new();
    for stamp_result in stamp_iter {
        match stamp_result {
            Ok(stamp) => {
                // println!("[DEBUG] Created stamp: sg={}, desc='{}'", stamp.sg_number, stamp.desc);
                all_stamps.push(stamp);
            }
            Err(_e) => {
                // println!("[DEBUG] Error creating stamp: {}", e);
            }
        }
    }
    all_stamps
}
