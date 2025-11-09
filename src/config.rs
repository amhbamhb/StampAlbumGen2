use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HeaderFormatting {
    pub date_format: Option<String>,
    pub label_style: Option<String>,
    pub series_style: Option<String>,
    pub include_currency: Option<bool>,
    // Optional toggles that may be placed inside the [header_formatting]
    // table in older or alternate config layouts. We accept these here
    // and propagate them up into the top-level `Config` so either
    // placement works.
    pub include_design: Option<bool>,
    pub include_engraving: Option<bool>,
    pub include_watermark: Option<bool>,
    pub include_perforation: Option<bool>,
    pub include_catalog_numbers: Option<bool>,
    pub include_series: Option<bool>,
    pub include_country: Option<bool>,
    pub include_date: Option<bool>,
    pub print_flag_cover: Option<bool>,
    pub header_spacing: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Config {
    pub reference_width_mm: Option<f32>,
    pub reference_height_mm: Option<f32>,
    pub stamp_box_height: Option<f32>,
    pub stamp_box_adjust: Option<f32>,
    // Page layout overrides (mm)
    pub album_page_width_mm: Option<f32>,
    pub album_page_height_mm: Option<f32>,
    // Album title/header font and size (AlbumEasy font codes)
    pub album_pages_title_font: Option<String>,
    pub album_pages_title_size: Option<f32>,
    // Header position: L, R, C, LR, RL
    pub album_pages_header_position: Option<String>,
    pub page_top_margin_mm: Option<f32>,
    pub page_bottom_margin_mm: Option<f32>,
    pub page_left_margin_mm: Option<f32>,
    pub page_right_margin_mm: Option<f32>,
    pub page_border_outer: Option<f32>,
    pub page_border_inner1: Option<f32>,
    pub page_border_inner2: Option<f32>,
    pub page_border_spacing: Option<f32>,
    pub album_pages_spacing_h: Option<f32>,
    pub album_pages_spacing_v: Option<f32>,
    pub median_width_cap_3: Option<f32>,
    pub median_width_cap_4: Option<f32>,
    pub row_font_spacing: Option<f32>,
    pub global_scale: Option<f32>,
    pub header_font_spacing: Option<f32>,
    pub header_formatting: Option<HeaderFormatting>,
    // Footer/page number controls
    pub album_pages_footer_number_enabled: Option<bool>,
    pub album_pages_footer_number_font: Option<String>,
    pub album_pages_footer_number_size: Option<f32>,
    pub album_pages_footer_number_position: Option<String>,
    pub album_pages_footer_number_start: Option<i32>,
    pub album_pages_footer_number_before: Option<String>,
    pub album_pages_footer_number_after: Option<String>,
    // When true, emit detailed per-stamp PAGE_FIT diagnostics into generated
    // AlbumEasy text output to help debug page-fit math (disabled by default).
    pub detailed_page_fit_diagnostics: Option<bool>,
    // When set to Some("placeholders"), grouped-range tokens like "1067-1068"
    // encountered on StampWorld pages will be expanded into individual
    // placeholder DB rows (fast). When set to Some("detail_fetch") the
    // collector may attempt to visit individual stamp detail pages to
    // populate full metadata (slower, not implemented here yet).
    pub expand_grouped_ranges: Option<String>,
    // Conservative buffer reserved at bottom of page (mm) to avoid footer bleed
    pub page_fit_conservative_buffer_mm: Option<f32>,
    // Optional path to an offline calibration CSV file. CSV format:
    // filename,mm_per_px_w,mm_per_px_h
    pub calibration_csv: Option<String>,
    // Optional toggles to include certain header fields when true
    pub include_design: Option<bool>,
    pub include_engraving: Option<bool>,
    pub include_watermark: Option<bool>,
    pub include_perforation: Option<bool>,
    pub include_catalog_numbers: Option<bool>,
    pub include_series: Option<bool>,
    pub include_country: Option<bool>,
    pub include_date: Option<bool>,
    // Allow overriding maximum rows per page when header is larger
    pub max_rows_per_page: Option<usize>,
    // When false, don't emit ROW_START_ES automatically after PAGE_START
    // Default: true
    pub row_start_es_enabled: Option<bool>,
    // Configurable ROW_START_ES parameters
    pub row_start_es_font: Option<String>,
    pub row_start_es_size: Option<f32>,
    pub row_start_es_spacing: Option<f32>,
    pub row_start_es_vspace: Option<f32>,
    // Configurable ROW_START_ES parameters (font/size/spacing/vspace)
    // Allow a single very large stamp (or row) to occupy a page if enabled
    // and it exceeds the fraction `single_stamp_threshold_pct` of usable height.
    pub allow_single_large_stamp_per_page: Option<bool>,
    // Fraction (0.0..1.0) above which a single-row page is permitted.
    pub single_stamp_threshold_pct: Option<f32>,
    // Extra padding subtracted from usable height (mm) to avoid bleed at page bottom
    pub usable_height_padding_mm: Option<f32>,
    pub header_vspace_before_mm: Option<f32>,
    pub header_vspace_after_mm: Option<f32>,
    pub text_line_height_mm: Option<f32>,
    pub image_spacing_mm: Option<f32>,
    pub max_stamps_per_row: Option<u32>,
    pub album_stamp_img_new_setting_w_mm: Option<f32>,
    pub album_stamp_img_new_setting_h_mm: Option<f32>,
    pub album_stamp_img_new_setting_preserve_aspect: Option<bool>,
    // If false, the generator will skip emitting the large flag/title cover page
    // Default: true (when None or not present)
    pub print_flag_cover: Option<bool>,
    // If true, fetch a short Wikipedia summary for the album/site and save
    // it under the album directory (wiki_summary.txt). Default: false
    pub fetch_wikipedia: Option<bool>,
    // Legacy / alternate name: include_wikipedia_content
    pub include_wikipedia_content: Option<bool>,
    // Keep-alive alias for preserve aspect ratio flag used in some configs
    pub preserve_aspect: Option<bool>,
    // Alternative alias for aspect ratio preservation (used in some config files)
    pub album_stamp_img_aspect_ratio: Option<bool>,

    // Emit extra diagnostics related to placeholder expansion
    pub emit_placeholder_diagnostics: Option<bool>,

    // Backwards-compatible alias for detailed_page_fit_diagnostics
    pub detailed_page_fit: Option<bool>,

    // When true, include the stamp heading height in the row height calculation
    pub stamp_heading_in_row_height: Option<bool>,
    // When true, include pages vertical vspace in row height calculations
    pub pages_vspace_in_row_height: Option<bool>,

    // Packing safety margins (mm) - used to add small horizontal/vertical
    // safety paddings during packing to avoid visual overlap.
    pub packing_horizontal_safety_mm: Option<f32>,
    pub packing_vertical_safety_mm: Option<f32>,

    // Color customization keys (string names understood by the generator)
    pub color_album_border: Option<String>,
    pub color_album_footer: Option<String>,
    pub color_album_header: Option<String>,
    pub color_album_title: Option<String>,
    pub color_album_margin_txt: Option<String>,
    pub color_stamp_border: Option<String>,
    pub color_stamp_heading: Option<String>,
    pub color_stamp_text: Option<String>,
    pub color_stamp_background: Option<String>,
    // Flag cover specific settings (unique names prefixed with `flag_cover_`)
    pub flag_cover_img_w_mm: Option<f32>,
    pub flag_cover_img_h_mm: Option<f32>,
    pub flag_cover_page_vspace_top: Option<f32>,
    pub flag_cover_image_vspace_after: Option<f32>,
    pub flag_cover_image_title_font: Option<String>,
    pub flag_cover_image_title_size: Option<f32>,
    pub flag_cover_image_title_scale: Option<f32>,
    pub flag_cover_text_only_vspace_top: Option<f32>,
    pub flag_cover_text_only_title_font: Option<String>,
    pub flag_cover_text_only_title_size: Option<f32>,
    pub flag_cover_text_only_title_scale: Option<f32>,
    pub flag_cover_text_only_vspace_after: Option<f32>,
    pub flag_cover_background_color: Option<String>,
    // How series are ordered in generated albums. Options:
    // - Some("first_date"): order series by the date of their first stamp (default behavior)
    // - Some("first_sg"): order series by the first stamp's SG number
    // - Some("alpha"): alphabetical series order
    pub series_order: Option<String>,
    // Custom string markers emitted around headers and row starts so tests
    // and downstream tools can reliably parse and identify sections.
    // Default values are chosen to reproduce historic output.
    pub page_vspace_marker: Option<String>,
    pub row_start_es_marker: Option<String>,
    // Optional templates for PAGE_TEXT output. Each template must include
    // a single `{}` placeholder which will be replaced with the text to
    // emit. If omitted the generator falls back to historical literals
    // such as `PAGE_TEXT_CENTRE (TN 8 "{}" 1.0)`.
    pub page_text_format: Option<String>,
    pub page_text_centre_format: Option<String>,
    pub page_text_right_format: Option<String>,
    // Per-field overrides for PAGE_TEXT templates. If present these fields
    // are used to synthesize a template string and provide convenient tuning
    // for font, size and optional vspace without having to provide a full
    // template string. Each of these are optional and defaults are used
    // when omitted.
    pub page_text_font: Option<String>,
    pub page_text_size: Option<f32>,
    pub page_text_vspace: Option<f32>,
    pub page_text_centre_font: Option<String>,
    pub page_text_centre_size: Option<f32>,
    pub page_text_centre_vspace: Option<f32>,
    pub page_text_right_font: Option<String>,
    pub page_text_right_size: Option<f32>,
    pub page_text_right_vspace: Option<f32>,
    // Stamp heading formatting: font, size and label used in STAMP_HEADING
    pub stamp_heading_font: Option<String>,
    pub stamp_heading_size: Option<f32>,
    pub stamp_heading_label: Option<String>,
    // Optional override: force a DPI to be used when converting pixels -> mm.
    // If present the generator will compute mm_per_pixel = 25.4 / dpi and
    // use that instead of the embedded-reference ratio when converting
    // stored pixel dimensions to millimetres.
    pub force_dpi: Option<f32>,
    // When true, prefer the DPI-derived/DB-stored real-world image sizes and
    // do NOT scale images down to `stamp_box_height` during emission.
    // This allows emitted STAMP_ADD_IMG boxes to reflect real-world mm
    // dimensions (useful when `force_dpi` is set and you want physical sizes).
    pub preserve_real_image_size: Option<bool>,

    // Progress indicator settings
    // When true, show spinning progress indicator with current operation status
    pub show_progress_indicator: Option<bool>,

    // Wikipedia content configuration
    // Path to Wikipedia cache database
    pub wikipedia_cache_db_path: Option<String>,
    // Wikipedia content font settings
    pub content_wiki_font: Option<String>,
    pub content_wiki_font_size: Option<u32>,
    pub content_wiki_max_lines_per_page: Option<u32>,
    pub content_wiki_estimated_chars_per_line: Option<usize>,
    // Wikipedia paragraph formatting
    pub wikipedia_paragraph_font: Option<String>,
    pub wikipedia_paragraph_font_size: Option<u32>,
    pub wikipedia_paragraph_alignment: Option<String>,
    pub wikipedia_paragraph_padding: Option<f32>,
    pub wikipedia_paragraph_vspace: Option<f32>,
    // Multiplier used to compute paragraph vspace when explicit vspace is not set
    pub wikipedia_paragraph_vspace_multiplier: Option<f32>,
    pub wikipedia_paragraph_right_padding: Option<f32>,
    // Pagination strategy for Wikipedia content: "balanced" | "conservative" | "no_balance"
    pub wikipedia_pagination_strategy: Option<String>,
    // If true, distribute leftover vertical space as top padding to center content on page
    pub wikipedia_center_content: Option<bool>,
    // Conservative pagination fraction (used by conservative strategy). e.g., 0.75
    pub wikipedia_pagination_conservative_fraction: Option<f32>,
    // Wikipedia section title formatting
    pub wikipedia_section_title_font: Option<String>,
    pub wikipedia_section_title_font_size: Option<u32>,
}

impl Config {
    /// Returns a Config with all default values set (used for auto-fixing missing config fields)
    pub fn with_defaults() -> Self {
        Config {
            reference_width_mm: Some(24.0),
            reference_height_mm: Some(27.0),
            stamp_box_height: Some(30.0),
            stamp_box_adjust: Some(-3.5),
            album_page_width_mm: Some(216.0),
            album_page_height_mm: Some(279.4),
            album_pages_title_font: Some("HN".to_string()),
            album_pages_title_size: Some(10.0),
            album_pages_header_position: Some("C".to_string()),
            page_top_margin_mm: Some(10.0),
            page_bottom_margin_mm: Some(10.0),
            page_left_margin_mm: Some(10.0),
            page_right_margin_mm: Some(10.0),
            page_border_outer: Some(0.1),
            page_border_inner1: Some(0.5),
            page_border_inner2: Some(0.1),
            page_border_spacing: Some(1.0),
            album_pages_spacing_h: Some(3.0),
            album_pages_spacing_v: Some(3.0),
            row_font_spacing: Some(0.1),
            header_font_spacing: Some(5.0),
            album_pages_footer_number_enabled: Some(true),
            album_pages_footer_number_font: Some("HN".to_string()),
            album_pages_footer_number_size: Some(8.0),
            album_pages_footer_number_position: Some("C".to_string()),
            album_pages_footer_number_start: Some(1),
            album_pages_footer_number_before: Some("Page ".to_string()),
            album_pages_footer_number_after: Some(" of $PAGES$".to_string()),
            detailed_page_fit_diagnostics: Some(false),
            expand_grouped_ranges: Some("placeholders".to_string()),
            page_fit_conservative_buffer_mm: Some(2.0),
            include_design: Some(true),
            include_engraving: Some(true),
            include_watermark: Some(true),
            include_perforation: Some(true),
            include_catalog_numbers: Some(true),
            include_series: Some(true),
            include_country: Some(true),
            include_date: Some(true),
            max_rows_per_page: Some(5),
            row_start_es_enabled: Some(true),
            row_start_es_font: Some("HN".to_string()),
            row_start_es_size: Some(6.0),
            row_start_es_spacing: Some(0.1),
            row_start_es_vspace: Some(5.0),
            allow_single_large_stamp_per_page: Some(true),
            single_stamp_threshold_pct: Some(0.7),
            usable_height_padding_mm: Some(1.0),
            header_vspace_before_mm: Some(10.0),
            header_vspace_after_mm: Some(1.0),
            text_line_height_mm: Some(2.6),
            image_spacing_mm: Some(5.0),
            max_stamps_per_row: Some(5),
            album_stamp_img_new_setting_w_mm: Some(6.0),
            album_stamp_img_new_setting_h_mm: Some(6.0),
            album_stamp_img_new_setting_preserve_aspect: Some(true),
            print_flag_cover: Some(true),
            include_wikipedia_content: Some(true),
            album_stamp_img_aspect_ratio: Some(true),
            packing_horizontal_safety_mm: Some(2.0),
            packing_vertical_safety_mm: Some(2.0),
            color_album_border: Some("darkblue".to_string()),
            color_album_footer: Some("darkblue".to_string()),
            color_album_header: Some("darkblue".to_string()),
            color_album_title: Some("darkblue".to_string()),
            color_album_margin_txt: Some("darkblue".to_string()),
            color_stamp_border: Some("black".to_string()),
            color_stamp_heading: Some("black".to_string()),
            color_stamp_text: Some("black".to_string()),
            color_stamp_background: Some("black".to_string()),
            flag_cover_img_w_mm: Some(160.0),
            flag_cover_img_h_mm: Some(80.0),
            flag_cover_page_vspace_top: Some(40.0),
            flag_cover_image_vspace_after: Some(12.0),
            flag_cover_image_title_font: Some("HN".to_string()),
            flag_cover_image_title_size: Some(32.0),
            flag_cover_image_title_scale: Some(1.0),
            flag_cover_text_only_vspace_top: Some(24.0),
            flag_cover_text_only_title_font: Some("HN".to_string()),
            flag_cover_text_only_title_size: Some(48.0),
            flag_cover_text_only_title_scale: Some(1.0),
            flag_cover_text_only_vspace_after: Some(18.0),
            series_order: Some("first_sg".to_string()),
            page_vspace_marker: Some("6".to_string()),
            row_start_es_marker: Some("ROW_START_ES".to_string()),
            page_text_format: Some(r#"PAGE_TEXT (HN 10 "{}" 1.0)"#.to_string()),
            page_text_centre_format: Some(r#"PAGE_TEXT_CENTRE (TN 8 "{}" 1.0)"#.to_string()),
            page_text_right_format: Some(r#"PAGE_TEXT_RIGHT (HN 10 "{}" 1.0)"#.to_string()),
            page_text_font: Some("HN".to_string()),
            page_text_size: Some(10.0),
            page_text_vspace: Some(1.0),
            page_text_centre_font: Some("HN".to_string()),
            page_text_centre_size: Some(8.0),
            page_text_centre_vspace: Some(1.0),
            page_text_right_font: Some("HN".to_string()),
            page_text_right_size: Some(10.0),
            page_text_right_vspace: Some(1.0),
            stamp_heading_font: Some("HN".to_string()),
            stamp_heading_size: Some(6.0),
            stamp_heading_label: Some("SG".to_string()),
            force_dpi: Some(96.0),
            preserve_real_image_size: Some(true),
            show_progress_indicator: Some(true),
            wikipedia_cache_db_path: Some("wikipedia_cache.db".to_string()),
            content_wiki_font: Some("HN".to_string()),
            content_wiki_font_size: Some(12),
            // Prefer automatic estimation by default; allow users to override
            // with explicit caps only when necessary.
            content_wiki_max_lines_per_page: None,
            content_wiki_estimated_chars_per_line: None,
            wikipedia_paragraph_font: Some("HN".to_string()),
            wikipedia_paragraph_font_size: Some(12),
            wikipedia_paragraph_alignment: Some("JUSTIFIED".to_string()),
            wikipedia_paragraph_padding: Some(10.0),
            wikipedia_paragraph_vspace: Some(5.0),
            wikipedia_paragraph_vspace_multiplier: Some(0.25),
            wikipedia_paragraph_right_padding: Some(10.0),
            wikipedia_pagination_strategy: Some("balanced".to_string()),
            wikipedia_center_content: Some(true),
            wikipedia_pagination_conservative_fraction: Some(0.75),
            wikipedia_section_title_font: Some("HB".to_string()),
            wikipedia_section_title_font_size: Some(14),
            // All other fields remain None (not set in defaults)
            ..Default::default()
        }
    }
}

/// Detects duplicate keys in TOML content by parsing line by line
/// Returns a vector of duplicate key names if any are found
fn detect_duplicate_keys(content: &str) -> Option<Vec<String>> {
    use std::collections::HashMap;

    let mut key_counts: HashMap<String, usize> = HashMap::new();
    let mut duplicates: Vec<String> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip comments and empty lines
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }

        // Look for key = value patterns (basic detection)
        if let Some(key_end) = trimmed.find('=') {
            let key = trimmed[..key_end].trim();

            // Skip if key contains brackets (array/table syntax)
            if key.contains('[') || key.contains(']') {
                continue;
            }

            // Remove quotes if present
            let clean_key = key.trim_matches('"').trim_matches('\'');

            *key_counts.entry(clean_key.to_string()).or_insert(0) += 1;
        }
    }

    // Collect keys that appear more than once
    for (key, count) in key_counts {
        if count > 1 {
            duplicates.push(key);
        }
    }

    if duplicates.is_empty() {
        None
    } else {
        duplicates.sort();
        Some(duplicates)
    }
}

pub fn load_config(path: &str) -> Config {
    // Try to find the config file starting at the current working directory
    // and searching upward through parent directories. This ensures that
    // running the binary from a subdirectory (e.g. Albums/Canada) will still
    // pick up the project's top-level `config.toml`.
    let try_paths = {
        let mut paths = Vec::new();

        // First, try paths relative to current working directory
        if let Ok(mut dir) = std::env::current_dir() {
            for _ in 0..8 {
                paths.push(dir.join(path));
                if !dir.pop() {
                    break;
                }
            }
        }

        // Also try paths relative to the executable's location
        // This helps when running the executable from a different directory
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(mut exe_dir) = exe_path.parent() {
                for _ in 0..8 {
                    paths.push(exe_dir.join(path));
                    if let Some(parent) = exe_dir.parent() {
                        exe_dir = parent;
                    } else {
                        break;
                    }
                }
            }
        }

        // Also try the literal path as a last resort
        paths.push(std::path::PathBuf::from(path));
        paths
    };

    for candidate in try_paths.iter() {
        debug!("Trying config path: {}", candidate.display());
        if let Ok(content) = fs::read_to_string(candidate) {
            info!("Found config file at: {}", candidate.display());
            debug!(
                "Config file content (first 200 chars): {}",
                content.chars().take(200).collect::<String>()
            );

            // Check for duplicate keys in the TOML content
            if let Some(duplicates) = detect_duplicate_keys(&content) {
                eprintln!(
                    "WARNING: Config file '{}' contains duplicate keys:",
                    candidate.display()
                );
                for key in duplicates {
                    eprintln!("  - '{}' appears multiple times", key);
                }
                eprintln!("The last value for each duplicate key will be used.");
                eprintln!("Consider removing the duplicate entries to clean up your config file.");
            }

            // First parse into a generic toml::Value to inspect raw mapping
            if let Ok(_value) = toml::from_str::<toml::Value>(&content) {
                // Successfully parsed into toml::Value; suppress noisy stdout debug prints
                // and use the log crate for structured logging instead.
                debug!("Successfully parsed as TOML");
            } else {
                // Provide user-friendly error message for TOML parsing failures
                eprintln!(
                    "ERROR: Config file '{}' contains invalid TOML syntax.",
                    candidate.display()
                );
                eprintln!("Common issues:");
                eprintln!("- Duplicate keys (same setting appears multiple times)");
                eprintln!("- Invalid value format (e.g., missing quotes around strings)");
                eprintln!("- Syntax errors (unmatched brackets, invalid characters)");
                eprintln!("Please check the file and fix any syntax errors.");
                eprintln!("File content preview (first 10 lines):");
                for (i, line) in content.lines().enumerate().take(10) {
                    eprintln!("  {}: {}", i + 1, line);
                }
                continue;
            }

            // Parse the content into a toml::Value so we can detect keys that
            // may have been placed inside `[header_formatting]` by mistake.
            match toml::from_str::<toml::Value>(&content) {
                Ok(mut value) => {
                    debug!(
                        "Successfully parsed TOML structure from {}",
                        candidate.display()
                    );
                    // If keys are present inside the header_formatting table but
                    // not at top-level, promote them to top-level so deserialization
                    // into `Config` picks them up. This handles malformed or
                    // legacy configs that put page/layout keys inside
                    // `[header_formatting]`.
                    if let Some(hf_table) = value
                        .get("header_formatting")
                        .and_then(|v| v.as_table())
                        .cloned()
                    {
                        // Only promote keys that are missing from top-level. We cloned
                        // the header_formatting table above so we don't hold an immutable
                        // borrow on `value` while mutating it.
                        if let Some(tbl) = value.as_table_mut() {
                            for (k, v) in hf_table.iter() {
                                if tbl.get(k).is_none() {
                                    tbl.insert(k.clone(), v.clone());
                                }
                            }
                        }
                    }

                    // Now serialize the possibly-modified `Value` back to TOML
                    // and deserialize into the strongly-typed `Config`.
                    match toml::to_string(&value) {
                        Ok(s) => match toml::from_str::<Config>(&s) {
                            Ok(mut cfg) => {
                                // Auto-fix missing fields by adding defaults
                                let defaults = Config::with_defaults();
                                let mut has_missing = false;

                                // Check and set missing fields
                                if cfg.reference_width_mm.is_none() {
                                    cfg.reference_width_mm = defaults.reference_width_mm;
                                    has_missing = true;
                                }
                                if cfg.reference_height_mm.is_none() {
                                    cfg.reference_height_mm = defaults.reference_height_mm;
                                    has_missing = true;
                                }
                                if cfg.stamp_box_height.is_none() {
                                    cfg.stamp_box_height = defaults.stamp_box_height;
                                    has_missing = true;
                                }
                                if cfg.album_page_width_mm.is_none() {
                                    cfg.album_page_width_mm = defaults.album_page_width_mm;
                                    has_missing = true;
                                }
                                if cfg.album_page_height_mm.is_none() {
                                    cfg.album_page_height_mm = defaults.album_page_height_mm;
                                    has_missing = true;
                                }
                                if cfg.album_pages_title_font.is_none() {
                                    cfg.album_pages_title_font = defaults.album_pages_title_font;
                                    has_missing = true;
                                }
                                if cfg.album_pages_title_size.is_none() {
                                    cfg.album_pages_title_size = defaults.album_pages_title_size;
                                    has_missing = true;
                                }
                                if cfg.album_pages_header_position.is_none() {
                                    cfg.album_pages_header_position =
                                        defaults.album_pages_header_position;
                                    has_missing = true;
                                }
                                if cfg.page_top_margin_mm.is_none() {
                                    cfg.page_top_margin_mm = defaults.page_top_margin_mm;
                                    has_missing = true;
                                }
                                if cfg.page_bottom_margin_mm.is_none() {
                                    cfg.page_bottom_margin_mm = defaults.page_bottom_margin_mm;
                                    has_missing = true;
                                }
                                if cfg.page_left_margin_mm.is_none() {
                                    cfg.page_left_margin_mm = defaults.page_left_margin_mm;
                                    has_missing = true;
                                }
                                if cfg.page_right_margin_mm.is_none() {
                                    cfg.page_right_margin_mm = defaults.page_right_margin_mm;
                                    has_missing = true;
                                }
                                if cfg.page_border_outer.is_none() {
                                    cfg.page_border_outer = defaults.page_border_outer;
                                    has_missing = true;
                                }
                                if cfg.page_border_inner1.is_none() {
                                    cfg.page_border_inner1 = defaults.page_border_inner1;
                                    has_missing = true;
                                }
                                if cfg.page_border_inner2.is_none() {
                                    cfg.page_border_inner2 = defaults.page_border_inner2;
                                    has_missing = true;
                                }
                                if cfg.page_border_spacing.is_none() {
                                    cfg.page_border_spacing = defaults.page_border_spacing;
                                    has_missing = true;
                                }
                                if cfg.album_pages_spacing_h.is_none() {
                                    cfg.album_pages_spacing_h = defaults.album_pages_spacing_h;
                                    has_missing = true;
                                }
                                if cfg.album_pages_spacing_v.is_none() {
                                    cfg.album_pages_spacing_v = defaults.album_pages_spacing_v;
                                    has_missing = true;
                                }
                                if cfg.row_font_spacing.is_none() {
                                    cfg.row_font_spacing = defaults.row_font_spacing;
                                    has_missing = true;
                                }
                                if cfg.header_font_spacing.is_none() {
                                    cfg.header_font_spacing = defaults.header_font_spacing;
                                    has_missing = true;
                                }
                                if cfg.album_pages_footer_number_enabled.is_none() {
                                    cfg.album_pages_footer_number_enabled =
                                        defaults.album_pages_footer_number_enabled;
                                    has_missing = true;
                                }
                                if cfg.album_pages_footer_number_font.is_none() {
                                    cfg.album_pages_footer_number_font =
                                        defaults.album_pages_footer_number_font;
                                    has_missing = true;
                                }
                                if cfg.album_pages_footer_number_size.is_none() {
                                    cfg.album_pages_footer_number_size =
                                        defaults.album_pages_footer_number_size;
                                    has_missing = true;
                                }
                                if cfg.album_pages_footer_number_position.is_none() {
                                    cfg.album_pages_footer_number_position =
                                        defaults.album_pages_footer_number_position;
                                    has_missing = true;
                                }
                                if cfg.album_pages_footer_number_start.is_none() {
                                    cfg.album_pages_footer_number_start =
                                        defaults.album_pages_footer_number_start;
                                    has_missing = true;
                                }
                                if cfg.album_pages_footer_number_before.is_none() {
                                    cfg.album_pages_footer_number_before =
                                        defaults.album_pages_footer_number_before;
                                    has_missing = true;
                                }
                                if cfg.album_pages_footer_number_after.is_none() {
                                    cfg.album_pages_footer_number_after =
                                        defaults.album_pages_footer_number_after;
                                    has_missing = true;
                                }
                                if cfg.include_design.is_none() {
                                    cfg.include_design = defaults.include_design;
                                    has_missing = true;
                                }
                                if cfg.include_engraving.is_none() {
                                    cfg.include_engraving = defaults.include_engraving;
                                    has_missing = true;
                                }
                                if cfg.include_watermark.is_none() {
                                    cfg.include_watermark = defaults.include_watermark;
                                    has_missing = true;
                                }
                                if cfg.include_perforation.is_none() {
                                    cfg.include_perforation = defaults.include_perforation;
                                    has_missing = true;
                                }
                                if cfg.include_catalog_numbers.is_none() {
                                    cfg.include_catalog_numbers = defaults.include_catalog_numbers;
                                    has_missing = true;
                                }
                                if cfg.include_series.is_none() {
                                    cfg.include_series = defaults.include_series;
                                    has_missing = true;
                                }
                                if cfg.include_country.is_none() {
                                    cfg.include_country = defaults.include_country;
                                    has_missing = true;
                                }
                                if cfg.include_date.is_none() {
                                    cfg.include_date = defaults.include_date;
                                    has_missing = true;
                                }
                                if cfg.max_rows_per_page.is_none() {
                                    cfg.max_rows_per_page = defaults.max_rows_per_page;
                                    has_missing = true;
                                }
                                if cfg.row_start_es_enabled.is_none() {
                                    cfg.row_start_es_enabled = defaults.row_start_es_enabled;
                                    has_missing = true;
                                }
                                if cfg.row_start_es_font.is_none() {
                                    cfg.row_start_es_font = defaults.row_start_es_font;
                                    has_missing = true;
                                }
                                if cfg.row_start_es_size.is_none() {
                                    cfg.row_start_es_size = defaults.row_start_es_size;
                                    has_missing = true;
                                }
                                if cfg.row_start_es_spacing.is_none() {
                                    cfg.row_start_es_spacing = defaults.row_start_es_spacing;
                                    has_missing = true;
                                }
                                if cfg.row_start_es_vspace.is_none() {
                                    cfg.row_start_es_vspace = defaults.row_start_es_vspace;
                                    has_missing = true;
                                }
                                if cfg.print_flag_cover.is_none() {
                                    cfg.print_flag_cover = defaults.print_flag_cover;
                                    has_missing = true;
                                }
                                if cfg.include_wikipedia_content.is_none() {
                                    cfg.include_wikipedia_content =
                                        defaults.include_wikipedia_content;
                                    has_missing = true;
                                }
                                if cfg.album_stamp_img_aspect_ratio.is_none() {
                                    cfg.album_stamp_img_aspect_ratio =
                                        defaults.album_stamp_img_aspect_ratio;
                                    has_missing = true;
                                }
                                if cfg.color_album_border.is_none() {
                                    cfg.color_album_border = defaults.color_album_border;
                                    has_missing = true;
                                }
                                if cfg.color_album_footer.is_none() {
                                    cfg.color_album_footer = defaults.color_album_footer;
                                    has_missing = true;
                                }
                                if cfg.color_album_header.is_none() {
                                    cfg.color_album_header = defaults.color_album_header;
                                    has_missing = true;
                                }
                                if cfg.color_album_title.is_none() {
                                    cfg.color_album_title = defaults.color_album_title;
                                    has_missing = true;
                                }
                                if cfg.color_album_margin_txt.is_none() {
                                    cfg.color_album_margin_txt = defaults.color_album_margin_txt;
                                    has_missing = true;
                                }
                                if cfg.color_stamp_border.is_none() {
                                    cfg.color_stamp_border = defaults.color_stamp_border;
                                    has_missing = true;
                                }
                                if cfg.color_stamp_heading.is_none() {
                                    cfg.color_stamp_heading = defaults.color_stamp_heading;
                                    has_missing = true;
                                }
                                if cfg.color_stamp_text.is_none() {
                                    cfg.color_stamp_text = defaults.color_stamp_text;
                                    has_missing = true;
                                }
                                if cfg.color_stamp_background.is_none() {
                                    cfg.color_stamp_background = defaults.color_stamp_background;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_img_w_mm.is_none() {
                                    cfg.flag_cover_img_w_mm = defaults.flag_cover_img_w_mm;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_img_h_mm.is_none() {
                                    cfg.flag_cover_img_h_mm = defaults.flag_cover_img_h_mm;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_page_vspace_top.is_none() {
                                    cfg.flag_cover_page_vspace_top =
                                        defaults.flag_cover_page_vspace_top;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_image_vspace_after.is_none() {
                                    cfg.flag_cover_image_vspace_after =
                                        defaults.flag_cover_image_vspace_after;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_image_title_font.is_none() {
                                    cfg.flag_cover_image_title_font =
                                        defaults.flag_cover_image_title_font;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_image_title_size.is_none() {
                                    cfg.flag_cover_image_title_size =
                                        defaults.flag_cover_image_title_size;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_image_title_scale.is_none() {
                                    cfg.flag_cover_image_title_scale =
                                        defaults.flag_cover_image_title_scale;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_text_only_vspace_top.is_none() {
                                    cfg.flag_cover_text_only_vspace_top =
                                        defaults.flag_cover_text_only_vspace_top;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_text_only_title_font.is_none() {
                                    cfg.flag_cover_text_only_title_font =
                                        defaults.flag_cover_text_only_title_font;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_text_only_title_size.is_none() {
                                    cfg.flag_cover_text_only_title_size =
                                        defaults.flag_cover_text_only_title_size;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_text_only_title_scale.is_none() {
                                    cfg.flag_cover_text_only_title_scale =
                                        defaults.flag_cover_text_only_title_scale;
                                    has_missing = true;
                                }
                                if cfg.flag_cover_text_only_vspace_after.is_none() {
                                    cfg.flag_cover_text_only_vspace_after =
                                        defaults.flag_cover_text_only_vspace_after;
                                    has_missing = true;
                                }
                                if cfg.series_order.is_none() {
                                    cfg.series_order = defaults.series_order;
                                    has_missing = true;
                                }
                                if cfg.page_text_format.is_none() {
                                    cfg.page_text_format = defaults.page_text_format;
                                    has_missing = true;
                                }
                                if cfg.page_text_centre_format.is_none() {
                                    cfg.page_text_centre_format = defaults.page_text_centre_format;
                                    has_missing = true;
                                }
                                if cfg.page_text_right_format.is_none() {
                                    cfg.page_text_right_format = defaults.page_text_right_format;
                                    has_missing = true;
                                }
                                if cfg.page_text_font.is_none() {
                                    cfg.page_text_font = defaults.page_text_font;
                                    has_missing = true;
                                }
                                if cfg.page_text_size.is_none() {
                                    cfg.page_text_size = defaults.page_text_size;
                                    has_missing = true;
                                }
                                if cfg.page_text_vspace.is_none() {
                                    cfg.page_text_vspace = defaults.page_text_vspace;
                                    has_missing = true;
                                }
                                if cfg.page_text_centre_font.is_none() {
                                    cfg.page_text_centre_font = defaults.page_text_centre_font;
                                    has_missing = true;
                                }
                                if cfg.page_text_centre_size.is_none() {
                                    cfg.page_text_centre_size = defaults.page_text_centre_size;
                                    has_missing = true;
                                }
                                if cfg.page_text_centre_vspace.is_none() {
                                    cfg.page_text_centre_vspace = defaults.page_text_centre_vspace;
                                    has_missing = true;
                                }
                                if cfg.page_text_right_font.is_none() {
                                    cfg.page_text_right_font = defaults.page_text_right_font;
                                    has_missing = true;
                                }
                                if cfg.page_text_right_size.is_none() {
                                    cfg.page_text_right_size = defaults.page_text_right_size;
                                    has_missing = true;
                                }
                                if cfg.page_text_right_vspace.is_none() {
                                    cfg.page_text_right_vspace = defaults.page_text_right_vspace;
                                    has_missing = true;
                                }
                                if cfg.stamp_heading_font.is_none() {
                                    cfg.stamp_heading_font = defaults.stamp_heading_font;
                                    has_missing = true;
                                }
                                if cfg.stamp_heading_size.is_none() {
                                    cfg.stamp_heading_size = defaults.stamp_heading_size;
                                    has_missing = true;
                                }
                                if cfg.stamp_heading_label.is_none() {
                                    cfg.stamp_heading_label = defaults.stamp_heading_label;
                                    has_missing = true;
                                }
                                if cfg.force_dpi.is_none() {
                                    cfg.force_dpi = defaults.force_dpi;
                                    has_missing = true;
                                }
                                if cfg.preserve_real_image_size.is_none() {
                                    cfg.preserve_real_image_size =
                                        defaults.preserve_real_image_size;
                                    has_missing = true;
                                }
                                if cfg.show_progress_indicator.is_none() {
                                    cfg.show_progress_indicator = defaults.show_progress_indicator;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_cache_db_path.is_none() {
                                    cfg.wikipedia_cache_db_path = defaults.wikipedia_cache_db_path;
                                    has_missing = true;
                                }
                                if cfg.content_wiki_font.is_none() {
                                    cfg.content_wiki_font = defaults.content_wiki_font;
                                    has_missing = true;
                                }
                                if cfg.content_wiki_font_size.is_none() {
                                    cfg.content_wiki_font_size = defaults.content_wiki_font_size;
                                    has_missing = true;
                                }
                                if cfg.content_wiki_max_lines_per_page.is_none() {
                                    cfg.content_wiki_max_lines_per_page =
                                        defaults.content_wiki_max_lines_per_page;
                                    has_missing = true;
                                }
                                if cfg.content_wiki_estimated_chars_per_line.is_none() {
                                    cfg.content_wiki_estimated_chars_per_line =
                                        defaults.content_wiki_estimated_chars_per_line;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_paragraph_font.is_none() {
                                    cfg.wikipedia_paragraph_font =
                                        defaults.wikipedia_paragraph_font;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_paragraph_font_size.is_none() {
                                    cfg.wikipedia_paragraph_font_size =
                                        defaults.wikipedia_paragraph_font_size;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_paragraph_alignment.is_none() {
                                    cfg.wikipedia_paragraph_alignment =
                                        defaults.wikipedia_paragraph_alignment;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_paragraph_padding.is_none() {
                                    cfg.wikipedia_paragraph_padding =
                                        defaults.wikipedia_paragraph_padding;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_paragraph_vspace.is_none() {
                                    cfg.wikipedia_paragraph_vspace =
                                        defaults.wikipedia_paragraph_vspace;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_paragraph_right_padding.is_none() {
                                    cfg.wikipedia_paragraph_right_padding =
                                        defaults.wikipedia_paragraph_right_padding;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_section_title_font.is_none() {
                                    cfg.wikipedia_section_title_font =
                                        defaults.wikipedia_section_title_font;
                                    has_missing = true;
                                }
                                if cfg.wikipedia_section_title_font_size.is_none() {
                                    cfg.wikipedia_section_title_font_size =
                                        defaults.wikipedia_section_title_font_size;
                                    has_missing = true;
                                }

                                if has_missing {
                                    // Write back the updated config with defaults added
                                    match toml::to_string(&cfg) {
                                        Ok(updated_content) => {
                                            if let Err(e) = fs::write(candidate, updated_content) {
                                                log::warn!("Failed to write updated config with defaults to {}: {}", candidate.display(), e);
                                            } else {
                                                log::info!("Added missing config fields with default values to {}", candidate.display());
                                            }
                                        }
                                        Err(e) => {
                                            log::warn!("Failed to serialize updated config: {}", e);
                                        }
                                    }
                                }

                                // If the file places the include_* toggles inside the
                                // `[header_formatting]` table (common), ensure those
                                // values are propagated into the top-level `Config` fields
                                // when the top-level ones are not present.
                                if let Some(hf) = cfg.header_formatting.clone() {
                                    cfg.include_design = cfg.include_design.or(hf.include_design);
                                    cfg.include_engraving =
                                        cfg.include_engraving.or(hf.include_engraving);
                                    cfg.include_watermark =
                                        cfg.include_watermark.or(hf.include_watermark);
                                    cfg.include_perforation =
                                        cfg.include_perforation.or(hf.include_perforation);
                                    cfg.include_catalog_numbers =
                                        cfg.include_catalog_numbers.or(hf.include_catalog_numbers);
                                    cfg.include_series = cfg.include_series.or(hf.include_series);
                                    cfg.include_country =
                                        cfg.include_country.or(hf.include_country);
                                    cfg.include_date = cfg.include_date.or(hf.include_date);
                                    cfg.print_flag_cover =
                                        cfg.print_flag_cover.or(hf.print_flag_cover);
                                    // header_formatting currently does not include the detailed
                                    // diagnostics toggle; leave it as-is when present at top-level.
                                }

                                info!("Loaded config from {}", candidate.display());
                                debug!("Parsed Config struct: {:?}", cfg);

                                // Debug log color configuration values to help troubleshoot
                                debug!("Color config values:");
                                debug!("  color_album_border: {:?}", cfg.color_album_border);
                                debug!(
                                    "  color_stamp_background: {:?}",
                                    cfg.color_stamp_background
                                );
                                debug!(
                                    "  album_stamp_img_aspect_ratio: {:?}",
                                    cfg.album_stamp_img_aspect_ratio
                                );

                                return cfg;
                            }
                            Err(e) => {
                                eprintln!(
                                    "ERROR: Failed to deserialize config from '{}': {}",
                                    candidate.display(),
                                    e
                                );
                                eprintln!("This usually means there are unknown or invalid configuration keys.");
                                eprintln!("Please check that all setting names are correct and values are in the right format.");
                                break;
                            }
                        },
                        Err(e) => {
                            eprintln!(
                                "ERROR: Failed to process config file '{}': {}",
                                candidate.display(),
                                e
                            );
                            eprintln!(
                                "This is an internal error while processing the configuration."
                            );
                            break;
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "ERROR: Config file '{}' has invalid TOML structure: {}",
                        candidate.display(),
                        e
                    );
                    eprintln!("File content preview (first 5 lines):");
                    for line in content.lines().take(5) {
                        eprintln!("  {}", line);
                    }
                    continue;
                }
            }
        } else {
            debug!("Could not read file: {}", candidate.display());
        }
    }

    // Log all the paths that were tried to help with debugging
    debug!("Config file '{}' not found. Searched paths:", path);
    for p in try_paths.iter() {
        debug!("  - {}", p.display());
    }

    // If no config file was found, create one with defaults
    if let Ok(cwd) = std::env::current_dir() {
        let default_config_path = cwd.join(path);
        let defaults = Config::with_defaults();
        match toml::to_string(&defaults) {
            Ok(content) => {
                if let Err(e) = fs::write(&default_config_path, content) {
                    log::warn!(
                        "Failed to create default config file at {}: {}",
                        default_config_path.display(),
                        e
                    );
                } else {
                    log::info!(
                        "Created new config file with default values at {}",
                        default_config_path.display()
                    );
                }
            }
            Err(e) => {
                log::warn!("Failed to serialize default config: {}", e);
            }
        }
    }

    log::info!("config.toml not found in parent dirs or failed to parse, using default values");
    Config::default()
}

use std::collections::HashMap;

/// Load a simple mapping file that allows users to override site_name -> canonical_slug
/// Example TOML:
/// [slug_overrides]
/// "British-ColumbiaIsland" = "British-Columbia!Vancouver-Island"
pub fn load_slug_overrides(path: &str) -> HashMap<String, String> {
    // Search upward from cwd like load_config
    let try_paths = {
        let mut paths = Vec::new();

        // First, try paths relative to current working directory
        if let Ok(mut dir) = std::env::current_dir() {
            for _ in 0..8 {
                paths.push(dir.join(path));
                if !dir.pop() {
                    break;
                }
            }
        }

        // Also try paths relative to the executable's location
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(mut exe_dir) = exe_path.parent() {
                for _ in 0..8 {
                    paths.push(exe_dir.join(path));
                    if let Some(parent) = exe_dir.parent() {
                        exe_dir = parent;
                    } else {
                        break;
                    }
                }
            }
        }

        paths.push(std::path::PathBuf::from(path));
        paths
    };

    for candidate in try_paths.iter() {
        if let Ok(content) = fs::read_to_string(candidate) {
            if let Ok(value) = toml::from_str::<toml::Value>(&content) {
                if let Some(table) = value.get("slug_overrides") {
                    if let Some(tbl) = table.as_table() {
                        let mut out = HashMap::new();
                        for (k, v) in tbl.iter() {
                            if let Some(s) = v.as_str() {
                                out.insert(k.clone(), s.to_string());
                            }
                        }
                        log::info!(
                            "Loaded slug_overrides from {} ({} entries)",
                            candidate.display(),
                            out.len()
                        );
                        return out;
                    }
                }
            }
        }
    }
    HashMap::new()
}
