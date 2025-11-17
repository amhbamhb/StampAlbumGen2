use crate::wikipedia::ContentSource;
use log::info;
use std::io::Write;

/// Writes global AlbumEasy header information to the output stream
pub fn write_globals<W: Write>(
    writer: &mut W,
    album_name: &str,
    conn_opt: Option<&rusqlite::Connection>,
    start_year: Option<i32>,
    end_year: Option<i32>,
) -> usize {
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
    let img_new_w = cfg.album_stamp_img_new_setting_w_mm.unwrap_or(6.0);
    let img_new_h = cfg.album_stamp_img_new_setting_h_mm.unwrap_or(6.0);
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

    // Emit a short, unconditional marker so automated runs always have a
    // greppable line indicating the wiki-metrics subsystem was considered for
    // this album generation. This avoids missing markers when pagination
    // isn't executed (e.g., no wiki content found).
    info!(
        "WIKI_PAGE_METRICS_RUN album={} include_wikipedia={}",
        album_name, cfg_include_wikipedia
    );

    // If requested, create/initialize a compact per-run metrics file in /tmp.
    // This ensures automation always has a small file to read even when
    // pagination is not executed (no wiki content). The file is overwritten
    // here with a baseline run marker and a no-wikipedia summary if applicable.
    if std::env::var("FORCE_WIKI_PAGE_METRICS").is_ok() {
        let safe_album = album_name.replace(' ', "_");
        let metrics_path = format!("/tmp/forcemetrics_{}.runlog", safe_album);
        if let Ok(mut mf) = std::fs::File::create(&metrics_path) {
            let _ = writeln!(
                mf,
                "FORCE_WIKI_PAGE_METRICS_RUN album={} include_wikipedia={}",
                album_name, cfg_include_wikipedia
            );
            if !cfg_include_wikipedia {
                let _ = writeln!(mf, "WIKI_PAGE_PAGINATION_SUMMARY album={} sections=0 pages=0 total_height=0.0 usable_height=0.0 strategy=none reason=no_wikipedia", album_name);
            } else {
                // Placeholder summary to be replaced/augmented later by the
                // pagination code when real content is available.
                let _ = writeln!(
                    mf,
                    "WIKI_PAGE_PAGINATION_SUMMARY_PENDING album={} note=awaiting_pagination",
                    album_name
                );
            }
        }
    }

    // If Wikipedia content is not enabled for this run, emit a concise
    // pagination summary showing zero sections/pages so automation can
    // rely on a single greppable line even when no pagination occurs.
    if !cfg_include_wikipedia {
        info!(
            "WIKI_PAGE_PAGINATION_SUMMARY album={} sections=0 pages=0 total_height=0.0 usable_height=0.0 strategy=none reason=no_wikipedia",
            album_name
        );
    }

    // Track page count for sequential numbering
    let mut page_counter = 0usize;
    let cfg_allow_single_large = cfg.allow_single_large_stamp_per_page.unwrap_or(true);
    let cfg_preserve_aspect = cfg
        .preserve_aspect
        .or(cfg.album_stamp_img_aspect_ratio)
        .unwrap_or(img_preserve);
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
        page_counter += 1; // Flag cover is a page

        // Add descriptive page header for flag cover
        let _ = writeln!(
            writer,
            "################################################################################"
        );
        let _ = writeln!(writer, "### Page {} ###", page_counter);
        let _ = writeln!(writer, "### {} Flag ###", album_name);
        let _ = writeln!(writer, "PAGE_START");
        // Write a simple cover block after the standard globals so it appears
        // below the STAMP_BOXES_SIZE_ADJUST section as requested.
        let album_images_dir = format!("Albums/{}/images", album_name.replace(' ', "-"));
        // Try to fetch/copy a flag image (size w320) and coat of arms. If these fail we ignore the error.
        let mut flag_path_opt: Option<String> = None;
        let mut coat_path_opt: Option<String> = None;
        match crate::flagdb::copy_flag_for_album(album_name, &album_images_dir, "w320") {
            Ok(p) => {
                if let Some(fname) = std::path::Path::new(&p)
                    .file_name()
                    .and_then(|s| s.to_str())
                {
                    flag_path_opt = Some(format!("images/{}", fname));
                } else {
                    flag_path_opt = Some(p);
                }
            }
            Err(_) => {}
        }
        match crate::flagdb::copy_coa_for_album(album_name, &album_images_dir) {
            Ok(p) => {
                if let Some(fname) = std::path::Path::new(&p)
                    .file_name()
                    .and_then(|s| s.to_str())
                {
                    coat_path_opt = Some(format!("images/{}", fname));
                } else {
                    coat_path_opt = Some(p);
                }
            }
            Err(_) => {}
        }

        // If flag and coat refer to the same image path, avoid showing both
        // on the cover (this can happen if a flag isn't available and the
        // coat file was used as a fallback). Prefer showing the coat only.
        if let (Some(coat_p), Some(flag_p)) = (coat_path_opt.as_ref(), flag_path_opt.as_ref()) {
            if coat_p == flag_p {
                flag_path_opt = None;
            }
        }

        // Row starts before specific cover elements (coat, flag, name)
        // are emitted inside each element's handling below so we don't
        // need to emit a blanket ROW_START_ES here.

        // Decide element order. Support cfg.flag_cover_elements as an ordered list of values: "coat", "flag", "name".
        let raw_elements: Vec<String> = cfg
            .flag_cover_elements
            .clone()
            .unwrap_or_else(|| vec!["coat".to_string(), "flag".to_string(), "name".to_string()]);

        // Deduplicate while preserving order so repeated entries in the config
        // (or accidental duplicates) don't cause duplicate output on the cover.
        use std::collections::HashSet;
        let mut seen: HashSet<String> = HashSet::new();
        let mut elements: Vec<String> = Vec::new();
        for e in raw_elements.into_iter() {
            if !seen.contains(&e) {
                seen.insert(e.clone());
                elements.push(e);
            }
        }

        // Did we obtain any images to show?
        let any_present = coat_path_opt.is_some() || flag_path_opt.is_some();
        // Track whether the configured element order already includes a name entry
        let name_included = elements.iter().any(|e| e.as_str() == "name");
        let top_vspace = cfg.flag_cover_page_vspace_top.unwrap_or(40.0);
        let after_vspace_default = cfg.flag_cover_image_vspace_after.unwrap_or(12.0);

        if any_present {
            let _ = writeln!(writer, "PAGE_VSPACE ({:.1})", top_vspace);
        }

        for elem in elements.iter() {
            match elem.as_str() {
                "coat" => {
                    if let Some(cp) = coat_path_opt.as_ref() {
                        // Emit a row start before the coat (large vertical element)
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
                        // White background for the coat area
                        let _ = writeln!(writer, "COLOR_STAMP_BACKGROUND(white)");
                        // Use configurable coat dimensions from config, default 80x100
                        let coat_w = cfg.flag_cover_coat_img_w_mm.unwrap_or(80.0);
                        let coat_h = cfg.flag_cover_coat_img_h_mm.unwrap_or(100.0);
                        let _ = writeln!(writer, "# Coat of arms for album (if available)");
                        let _ = writeln!(
                            writer,
                            "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"\" \"\" \"\" NO_FRAME)",
                            coat_w, coat_h, cp
                        );
                        // Small vertical gap after coat
                        let _ = writeln!(writer, "PAGE_VSPACE ({:.1})", 6.0);
                    }
                }
                "flag" => {
                    if let Some(fp) = flag_path_opt.as_ref() {
                        // Emit a small-row_start before the flag (HN 1 as requested)
                        if cfg_row_start_es {
                            let rs_font = cfg
                                .row_start_es_font
                                .clone()
                                .unwrap_or_else(|| "HN".to_string());
                            // Use a tiny font size 1 as requested
                            let rs_size = 1.0;
                            let rs_spacing = cfg.row_start_es_spacing.unwrap_or(0.1);
                            let rs_vspace = cfg.row_start_es_vspace.unwrap_or(5.0);
                            let _ = writeln!(
                                writer,
                                "ROW_START_ES ({} {} {:.1} {:.1})",
                                rs_font, rs_size as i32, rs_spacing, rs_vspace
                            );
                        }
                        let _ = writeln!(writer, "# Cover flag image for album");
                        let img_w = cfg.flag_cover_img_w_mm.unwrap_or(160.0);
                        let img_h = cfg.flag_cover_img_h_mm.unwrap_or(80.0);
                        let _ = writeln!(
                            writer,
                            "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"\" \"\" \"\" NO_FRAME)",
                            img_w, img_h, fp
                        );
                        // Set stamp background back to black as requested after the flag
                        let _ = writeln!(writer, "COLOR_STAMP_BACKGROUND(black)");
                        // Vertical gap after flag controlled by config
                        let _ = writeln!(writer, "PAGE_VSPACE ({:.1})", after_vspace_default);
                    }
                }
                "name" => {
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
                    let title_size = cfg.flag_cover_image_title_size.unwrap_or(32.0);
                    let _ = writeln!(
                        writer,
                        "PAGE_TEXT_CENTRE (TB {} \"{}\" 1.0)",
                        title_size as i32,
                        album_name.replace("\\n", " ")
                    );
                }
                _ => {}
            }
        }

        // If nothing was present, fall back to the previous text-only cover behavior
        if !any_present {
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
        } else {
            // Images were present. Only emit a centered title if the
            // configured element order did not already include a "name"
            // element. This avoids printing the name twice.
            if !name_included {
                let top_vspace = cfg.flag_cover_text_only_vspace_top.unwrap_or(24.0);
                let after_vspace = cfg.flag_cover_text_only_vspace_after.unwrap_or(18.0);
                let title_size = cfg.flag_cover_text_only_title_size.unwrap_or(48.0);

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
        }

        // After cover, emit a page separator so normal globals follow
        let _ = writeln!(
            writer,
            "################################################################################"
        );

        // Add Wikipedia content section if enabled
        if cfg_include_wikipedia {
            page_counter = add_wikipedia_section(writer, album_name, &cfg, page_counter);
        }
    }

    page_counter // Return the total number of pages created
}

/// Adds a Wikipedia content section to the album
/// Returns the updated page counter
fn add_wikipedia_section<W: Write>(
    writer: &mut W,
    album_name: &str,
    cfg: &crate::config::Config,
    mut page_counter: usize,
) -> usize {
    let cache_db_path = cfg
        .wikipedia_cache_db_path
        .as_deref()
        .unwrap_or("wikipedia_cache.db");

    // Precompute usable height so we can emit deterministic metrics even
    // when cached content is missing.
    let usable_height = crate::album_globals::compute_usable_height(cfg);

    match crate::wikipedia::get_cached_content(album_name, cache_db_path) {
        Ok(Some((content, source))) => {
            // usable_height already computed above

            // We'll compute a per-page target height based on the total estimated
            // content height so pages are balanced and we avoid an arbitrary large
            // safety margin that wastes space (previously a fixed 25% margin).
            info!("Wikipedia pagination: usable_height={:.1}", usable_height);

            // Start first page
            page_counter += 1;

            // Start section with descriptive page header indicating source
            let source_name = match source {
                crate::wikipedia::ContentSource::Grokipedia => "Grokipedia",
                crate::wikipedia::ContentSource::Wikipedia => "Wikipedia",
            };

            let _ = writeln!(writer, "### Page {} ###", page_counter);
            let _ = writeln!(writer, "### {} Content ###", source_name);
            let _ = writeln!(writer, "PAGE_START");

            // Add row start if configured and account for its height
            let _row_start_height = if cfg.row_start_es_enabled.unwrap_or(true) {
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
                let _ = writeln!(writer, "################################################################################");

                // Estimate height of row start (rough approximation)
                rs_vspace as f64 + 5.0 // Base height + spacing
            } else {
                0.0
            };

            // Parse content into sections and paragraphs for pagination
            let content_sections = parse_content_into_sections(&content);

            // Measure heights of all sections/paragraphs
            let heights: Vec<f64> = content_sections
                .iter()
                .map(|section| match section {
                    ContentSection::Header(header_text) => {
                        crate::wikipedia::estimate_header_height(header_text, cfg)
                    }
                    ContentSection::Paragraph(paragraph_text) => {
                        crate::wikipedia::estimate_text_height(paragraph_text, cfg)
                    }
                })
                .collect();

            // Compute total estimated content height
            let total_content_height: f64 = heights.iter().sum();

            // Choose pagination strategy (balanced, conservative, no_balance)
            let pagination_strategy = cfg
                .wikipedia_pagination_strategy
                .as_deref()
                .unwrap_or("balanced");

            // Determine per-page pagination target and page division according to strategy
            let balanced_pages = match pagination_strategy {
                "conservative" => {
                    // Use a conservative fraction of usable height (defaults to 0.75)
                    let frac = cfg
                        .wikipedia_pagination_conservative_fraction
                        .unwrap_or(0.75_f32) as f64;
                    let page_target = (usable_height * frac).max(10.0);
                    // Greedy pagination using the conservative target
                    create_greedy_pages(&content_sections, &heights, page_target)
                }
                "no_balance" => {
                    // Pure greedy filling up to usable_height; do not attempt balancing
                    create_greedy_pages(&content_sections, &heights, usable_height)
                }
                _ => {
                    // Default: balanced distribution based on total content height
                    let pages_needed = if total_content_height <= 0.0 {
                        1
                    } else {
                        (total_content_height / usable_height).ceil() as usize
                    };

                    // Per-page target: distribute content evenly but never exceed usable_height
                    let mut pagination_height = if pages_needed == 0 {
                        usable_height
                    } else {
                        (total_content_height / (pages_needed as f64)).min(usable_height)
                    };

                    // If content is small compared to a page, use usable_height so
                    // centering/flow look natural.
                    if total_content_height < usable_height * 0.9 {
                        pagination_height = usable_height;
                    }

                    create_balanced_pages(&content_sections, &heights, pagination_height, cfg)
                }
            };

            // Debug logging
            info!("Wikipedia content pagination: {} sections, {} pages, total_height={:.1}, usable_height={:.1}, strategy={}",
            content_sections.len(), balanced_pages.len(), total_content_height, usable_height, pagination_strategy);

            // Emit a concise, guaranteed one-line pagination summary for automation
            // This is intentionally small and at INFO level so it appears in rotated
            // Logs without enabling full debug logging.
            info!(
            "WIKI_PAGE_PAGINATION_SUMMARY album={} sections={} pages={} total_content_height={:.1} usable_height={:.1} strategy={}",
            album_name,
            content_sections.len(),
            balanced_pages.len(),
            total_content_height,
            usable_height,
            pagination_strategy
        );

            // Unconditional metrics emission (temporary): emit a single log line
            // so automated runs always have at least one easily-greppable marker
            // indicating pagination was executed and reporting totals. This helps
            // verify the code path is reached even when per-page metrics are
            // not emitted for some reason.
            info!(
                "WIKI_PAGE_METRICS_UNCOND pages={} usable_height={:.1} total_content_height={:.1} strategy={}",
                balanced_pages.len(),
                usable_height,
                total_content_height,
                pagination_strategy
            );

            // Guaranteed concise pagination summary for automation (single-line).
            // This avoids enabling debug everywhere while still exposing
            // sections/pages/height info in an easy-to-grep format.
            info!(
                "WIKI_PAGE_PAGINATION_SUMMARY album={} sections={} pages={} total_height={:.1} usable_height={:.1} strategy={}",
                album_name,
                content_sections.len(),
                balanced_pages.len(),
                total_content_height,
                usable_height,
                pagination_strategy
            );

            // If requested, emit per-page metrics to stdout for external tuning tools.
            // Controlled by environment variable WIKI_PAGE_DEBUG.
            // Emit per-page metrics when requested. Historically this was
            // controlled by WIKI_PAGE_DEBUG; also allow FORCE_WIKI_PAGE_METRICS
            // environment variable so we can reliably enable metrics during
            // automated runs without changing other debug settings.
            if std::env::var("WIKI_PAGE_DEBUG").is_ok()
                || std::env::var("FORCE_WIKI_PAGE_METRICS").is_ok()
            {
                for (pi, page_secs) in balanced_pages.iter().enumerate() {
                    let content_height: f64 = page_secs.iter().map(|&idx| heights[idx]).sum();
                    let percent = if usable_height > 0.0 {
                        (content_height / usable_height) * 100.0
                    } else {
                        0.0
                    };
                    info!(
                        "WIKI_PAGE_METRICS page={} usable_height={:.1} content_height={:.1} percent_used={:.1} strategy={}",
                        pi + 1,
                        usable_height,
                        content_height,
                        percent,
                        pagination_strategy
                    );
                }
            }

            // For deterministic testing in environments where Wikipedia
            // content may be missing, allow emitting a small set of synthetic
            // per-page metrics when FORCE_WIKI_PAGE_METRICS_DEMO is set. This
            // is temporary and intended only for verification of log parsing
            // and automation; it does not change generation behavior.
            if std::env::var("FORCE_WIKI_PAGE_METRICS_DEMO").is_ok() {
                for demo_page in 1..=3 {
                    info!(
                        "WIKI_PAGE_METRICS_DEMO page={} usable_height={:.1} content_height={:.1} percent_used={:.1} strategy={}",
                        demo_page,
                        usable_height,
                        usable_height * 0.5,
                        50.0,
                        pagination_strategy
                    );
                }
            }

            // If requested, also emit a compact per-run metrics file to /tmp so
            // automation can pick up pagination and per-page metrics without
            // having to parse rotated logs. This file is intentionally small
            // (one summary line + per-page lines) and is guarded by
            // FORCE_WIKI_PAGE_METRICS to avoid accidental writes.
            if std::env::var("FORCE_WIKI_PAGE_METRICS").is_ok() {
                let safe_album = album_name.replace(' ', "_");
                let metrics_path = format!("/tmp/forcemetrics_{}.runlog", safe_album);
                if let Ok(mut mf) = std::fs::File::create(&metrics_path) {
                    use std::io::Write;
                    let _ = writeln!(mf, "WIKI_PAGE_PAGINATION_SUMMARY album={} sections={} pages={} total_content_height={:.1} usable_height={:.1} strategy={}",
                        album_name, content_sections.len(), balanced_pages.len(), total_content_height, usable_height, pagination_strategy);
                    if balanced_pages.is_empty() {
                        // Emit a single explicit per-page metric (page=1 with zeros)
                        // so parsers can always expect a page line and not special-case
                        // the absence of pages.
                        let _ = writeln!(mf, "WIKI_PAGE_METRICS page=1 usable_height={:.1} content_height=0.0 percent_used=0.0 strategy={}",
                            usable_height, pagination_strategy);
                    } else {
                        for (pi, page_secs) in balanced_pages.iter().enumerate() {
                            let content_height: f64 =
                                page_secs.iter().map(|&idx| heights[idx]).sum();
                            let percent = if usable_height > 0.0 {
                                (content_height / usable_height) * 100.0
                            } else {
                                0.0
                            };
                            let _ = writeln!(mf, "WIKI_PAGE_METRICS page={} usable_height={:.1} content_height={:.1} percent_used={:.1} strategy={}",
                                pi + 1, usable_height, content_height, percent, pagination_strategy);
                        }
                    }
                }
            }

            for (page_index, page_sections) in balanced_pages.iter().enumerate() {
                if page_index > 0 {
                    // Add page separator between pages
                    let _ = writeln!(
                        writer,
                        "################################################################################"
                    );
                    // Start new page
                    let _ = writeln!(writer, "PAGE_START");
                    page_counter += 1;
                    let _ = writeln!(writer, "### Page {} ###", page_counter);
                    let _ = writeln!(writer, "### {} Content (continued) ###", source_name);
                }

                // Optionally center content vertically on the page by distributing
                // leftover space as top padding. Controlled by config.wikipedia_center_content.
                let page_content_height: f64 = page_sections.iter().map(|&idx| heights[idx]).sum();
                let mut final_centering_vspace = 0.0;
                if cfg.wikipedia_center_content.unwrap_or(true) {
                    let leftover = usable_height - page_content_height;
                    if leftover > 0.5 {
                        final_centering_vspace = leftover / 2.0;
                    }
                }

                info!(
                    "Wikipedia page {}: content_height={:.1}, centering_space={:.1}",
                    page_index, page_content_height, final_centering_vspace
                );

                // Emit a PAGE_VSPACE to provide top padding if centering is requested
                if final_centering_vspace > 0.0 {
                    let _ = writeln!(writer, "PAGE_VSPACE ({:.1})", final_centering_vspace);
                }

                // Emit each section in the balanced page
                for &section_index in page_sections {
                    match &content_sections[section_index] {
                        ContentSection::Header(header_text) => {
                            let section_title_font =
                                cfg.wikipedia_section_title_font.as_deref().unwrap_or("HB");
                            let section_title_font_size =
                                cfg.wikipedia_section_title_font_size.unwrap_or(14);
                            let _ = writeln!(
                                writer,
                                "PAGE_TEXT ({} {} \"{}\" 3.0)",
                                section_title_font, section_title_font_size, header_text
                            );
                        }
                        ContentSection::Paragraph(paragraph_text) => {
                            let paragraph_font =
                                cfg.wikipedia_paragraph_font.as_deref().unwrap_or("HN");
                            let paragraph_font_size =
                                cfg.wikipedia_paragraph_font_size.unwrap_or(12);
                            let paragraph_alignment = cfg
                                .wikipedia_paragraph_alignment
                                .as_deref()
                                .unwrap_or("JUSTIFIED");
                            let paragraph_padding = cfg.wikipedia_paragraph_padding.unwrap_or(10.0);
                            let paragraph_vspace = cfg.wikipedia_paragraph_vspace.unwrap_or(5.0);
                            let paragraph_right_padding =
                                cfg.wikipedia_paragraph_right_padding.unwrap_or(10.0);

                            let _ = writeln!(
                                writer,
                                "PAGE_TEXT_PARAGRAPH_START({} {} {} {:.1} {:.1} {:.1})",
                                paragraph_font,
                                paragraph_font_size,
                                paragraph_alignment,
                                paragraph_padding,
                                paragraph_vspace,
                                paragraph_right_padding
                            );
                            let escaped_content = paragraph_text.replace('"', "\\\"");
                            let _ = writeln!(writer, "{}", escaped_content);
                            let _ = writeln!(writer, "PAGE_TEXT_PARAGRAPH_END");
                        }
                    }
                }
            }

            // Add separator after content section
            let _ = writeln!(
                writer,
                "################################################################################"
            );
        }
        Ok(None) => {
            // No cached content available. If automation requested a compact
            // metrics file, write a clear summary indicating zero sections/pages
            // due to missing cached content so consumers have a deterministic
            // result to parse.
            if std::env::var("FORCE_WIKI_PAGE_METRICS").is_ok() {
                let safe_album = album_name.replace(' ', "_");
                let metrics_path = format!("/tmp/forcemetrics_{}.runlog", safe_album);
                if let Ok(mut mf) = std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&metrics_path)
                {
                    let _ = writeln!(mf, "WIKI_PAGE_PAGINATION_SUMMARY album={} sections=0 pages=0 total_height=0.0 usable_height={:.1} strategy=none reason=no_cached_content", album_name, usable_height);
                    // Emit an explicit per-page metric (page=1 with zeros) so parsers
                    // don't need to special-case missing per-page entries.
                    let _ = writeln!(mf, "WIKI_PAGE_METRICS page=1 usable_height={:.1} content_height=0.0 percent_used=0.0 strategy=none", usable_height);
                }
            }
        }
        Err(e) => {
            info!("Error retrieving cached content for {}: {}", album_name, e);
        }
    }

    page_counter // Return the updated page counter
}

/// Parse content into sections (headers and paragraphs) for pagination
#[derive(Debug)]
enum ContentSection {
    Header(String),
    Paragraph(String),
}

fn parse_content_into_sections(content: &str) -> Vec<ContentSection> {
    let mut sections = Vec::new();
    let content_lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < content_lines.len() {
        let line = content_lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        // Check if this is a section header (starts with #)
        if line.starts_with('#') {
            let header_text = line.trim_start_matches('#').trim().to_string();
            if !header_text.is_empty() {
                sections.push(ContentSection::Header(header_text));
            }
        } else {
            // This is a paragraph - collect all consecutive non-empty lines
            let mut paragraph_lines = Vec::new();
            while i < content_lines.len() && !content_lines[i].trim().is_empty() {
                paragraph_lines.push(content_lines[i].trim());
                i += 1;
            }

            if !paragraph_lines.is_empty() {
                let paragraph_text = paragraph_lines.join(" ");
                sections.push(ContentSection::Paragraph(paragraph_text));
            }
        }

        i += 1;
    }

    sections
}

/// Create balanced pages by distributing content more evenly across pages
/// This prevents orphaned content and optimizes page utilization
fn create_balanced_pages(
    sections: &[ContentSection],
    heights: &[f64],
    usable_height: f64,
    _cfg: &crate::config::Config,
) -> Vec<Vec<usize>> {
    if sections.is_empty() {
        return Vec::new();
    }

    // Start with greedy approach but enforce usable_height constraint strictly
    let mut pages = Vec::new();
    let mut current_page = Vec::new();
    let mut current_page_height = 0.0;
    let mut section_index = 0;

    while section_index < sections.len() {
        let section_height = heights[section_index];

        // If adding this section would exceed the page height, start a new page
        if current_page_height + section_height > usable_height {
            // If current page has content, finish it and start new page
            if !current_page.is_empty() {
                pages.push(current_page);
                current_page = Vec::new();
                current_page_height = 0.0;
            } else {
                // Edge case: single section is too tall for a page
                // This shouldn't happen with normal content, but handle it
                current_page.push(section_index);
                pages.push(current_page);
                current_page = Vec::new();
                section_index += 1;
                continue;
            }
        }

        // Add section to current page
        current_page.push(section_index);
        current_page_height += section_height;
        section_index += 1;
    }

    // Add final page if it has content
    if !current_page.is_empty() {
        pages.push(current_page);
    }

    // Now try to balance the pages better by moving content between pages
    // This is optional optimization - only if we have multiple pages
    if pages.len() > 1 {
        balance_pages(&mut pages, heights, usable_height);
    }

    info!(
        "Created {} pages for content with total height {:.1}",
        pages.len(),
        heights.iter().sum::<f64>()
    );

    pages
}

/// Create pages greedily without attempting to rebalance across pages.
fn create_greedy_pages(
    sections: &[ContentSection],
    heights: &[f64],
    usable_height: f64,
) -> Vec<Vec<usize>> {
    let mut pages = Vec::new();
    let mut current_page = Vec::new();
    let mut current_page_height = 0.0;
    let mut idx = 0;

    while idx < sections.len() {
        let h = heights[idx];
        if current_page_height + h > usable_height {
            if !current_page.is_empty() {
                pages.push(current_page);
                current_page = Vec::new();
                current_page_height = 0.0;
            } else {
                // single section larger than page: place it alone
                pages.push(vec![idx]);
                idx += 1;
                continue;
            }
        }
        current_page.push(idx);
        current_page_height += h;
        idx += 1;
    }
    if !current_page.is_empty() {
        pages.push(current_page);
    }
    pages
}

/// Helper function to balance content between existing pages
/// This tries to move content from over-full pages to under-full pages
fn balance_pages(pages: &mut Vec<Vec<usize>>, heights: &[f64], usable_height: f64) {
    // Calculate current page heights
    let mut page_heights: Vec<f64> = pages
        .iter()
        .map(|page| page.iter().map(|&idx| heights[idx]).sum())
        .collect();

    // Find pages that are significantly under-utilized
    let _avg_height = page_heights.iter().sum::<f64>() / page_heights.len() as f64;

    // Try to move content from the last page to earlier under-utilized pages
    let mut i = 0;
    while i < pages.len() - 1 {
        let current_height = page_heights[i];

        // If this page has room and the next page exists
        if current_height < usable_height * 0.9 && i + 1 < pages.len() {
            let next_page_idx = i + 1;
            let next_height = page_heights[next_page_idx];

            // Try to move the first item from next page to current page
            if !pages[next_page_idx].is_empty() {
                let item_to_move = pages[next_page_idx][0];
                let item_height = heights[item_to_move];

                // Check if moving this item would keep both pages under usable_height
                if current_height + item_height <= usable_height
                    && next_height - item_height <= usable_height
                {
                    // Move the item
                    let moved_item = pages[next_page_idx].remove(0);
                    pages[i].push(moved_item);

                    // Update heights
                    page_heights[i] += item_height;
                    page_heights[next_page_idx] -= item_height;

                    // If we moved an item, stay on this page to see if we can move more
                    continue;
                }
            }
        }

        i += 1;
    }
}
