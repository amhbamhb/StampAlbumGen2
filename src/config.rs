use serde::Deserialize;
use std::fs;
use log::{info, debug};

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
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
    // Token to use as the ROW_START_ES type (e.g. "TN"). When combined with
    // `row_start_es_font`, `row_start_es_size` and `row_start_es_vspace` this
    // forms a parameterized ROW_START_ES marker like: `TN 6 0.1 5.0`.
    pub row_start_es_token: Option<String>,
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
}

impl Default for Config {
    fn default() -> Self {
        Config {
            album_page_width_mm: None,
            album_page_height_mm: None,
            page_top_margin_mm: None,
                        page_left_margin_mm: None,
                        page_right_margin_mm: None,
            page_bottom_margin_mm: None,
            page_border_outer: None,
            page_border_inner1: None,
            page_border_inner2: None,
            page_border_spacing: None,
            reference_width_mm: None,
            reference_height_mm: None,
            stamp_box_height: None,
            stamp_box_adjust: None,
                        album_pages_spacing_h: None,
                        album_pages_spacing_v: None,
            median_width_cap_3: None,
            median_width_cap_4: None,
            global_scale: None,
            row_font_spacing: None,
            header_font_spacing: None,
            header_formatting: None,
            album_pages_title_font: None,
            album_pages_title_size: None,
            album_pages_header_position: None,
            album_pages_footer_number_enabled: None,
            album_pages_footer_number_font: None,
            album_pages_footer_number_size: None,
            album_pages_footer_number_position: None,
            album_pages_footer_number_start: None,
            album_pages_footer_number_before: None,
            album_pages_footer_number_after: None,
            detailed_page_fit_diagnostics: None,
            expand_grouped_ranges: None,
            page_fit_conservative_buffer_mm: None,
            include_design: None,
            include_engraving: None,
            include_watermark: None,
            include_perforation: None,
            include_catalog_numbers: None,
            include_series: None,
            include_country: None,
            include_date: None,
            calibration_csv: None,
            max_rows_per_page: None,
            print_flag_cover: None,
            fetch_wikipedia: None,
            include_wikipedia_content: None,
            preserve_aspect: None,

            emit_placeholder_diagnostics: None,
            detailed_page_fit: None,

            stamp_heading_in_row_height: None,
            pages_vspace_in_row_height: None,

            packing_horizontal_safety_mm: None,
            packing_vertical_safety_mm: None,

            color_album_border: None,
            color_album_footer: None,
            color_album_header: None,
            color_album_title: None,
            color_album_margin_txt: None,
            color_stamp_border: None,
            color_stamp_heading: None,
            color_stamp_text: None,
            color_stamp_background: None,
            flag_cover_img_w_mm: None,
            flag_cover_img_h_mm: None,
            flag_cover_page_vspace_top: None,
            flag_cover_image_vspace_after: None,
            flag_cover_image_title_font: None,
            flag_cover_image_title_size: None,
            flag_cover_image_title_scale: None,
            flag_cover_text_only_vspace_top: None,
            flag_cover_text_only_title_font: None,
            flag_cover_text_only_title_size: None,
            flag_cover_text_only_title_scale: None,
            flag_cover_text_only_vspace_after: None,
            flag_cover_background_color: None,
            series_order: None,
            page_vspace_marker: None,
            row_start_es_marker: None,
            row_start_es_enabled: None,
            row_start_es_font: None,
            row_start_es_size: None,
            row_start_es_spacing: None,
            row_start_es_vspace: None,
            allow_single_large_stamp_per_page: None,
              single_stamp_threshold_pct: None,
              usable_height_padding_mm: None,
              header_vspace_before_mm: None,
              header_vspace_after_mm: None,
              text_line_height_mm: None,
              image_spacing_mm: None,
                            album_stamp_img_new_setting_w_mm: None,
                            album_stamp_img_new_setting_h_mm: None,
                            album_stamp_img_new_setting_preserve_aspect: None,
        }
    }
}

pub fn load_config(path: &str) -> Config {
    // Try to find the config file starting at the current working directory
    // and searching upward through parent directories. This ensures that
    // running the binary from a subdirectory (e.g. Albums/Canada) will still
    // pick up the project's top-level `config.toml`.
    let try_paths = {
        let mut paths = Vec::new();
        if let Ok(mut dir) = std::env::current_dir() {
            for _ in 0..8 {
                paths.push(dir.join(path));
                if !dir.pop() { break; }
            }
        }
        // Also try the literal path as a last resort
        paths.push(std::path::PathBuf::from(path));
        paths
    };

    for candidate in try_paths.iter() {
        if let Ok(content) = fs::read_to_string(candidate) {
            // First parse into a generic toml::Value to inspect raw mapping
            if let Ok(_value) = toml::from_str::<toml::Value>(&content) {
                // Successfully parsed into toml::Value; suppress noisy stdout debug prints
                // and use the log crate for structured logging instead.
            } else {
                debug!("Failed to parse content into toml::Value while inspecting config");
            }

            // Parse the content into a toml::Value so we can detect keys that
            // may have been placed inside `[header_formatting]` by mistake.
            match toml::from_str::<toml::Value>(&content) {
                Ok(mut value) => {
                    // If keys are present inside the header_formatting table but
                    // not at top-level, promote them to top-level so deserialization
                    // into `Config` picks them up. This handles malformed or
                    // legacy configs that put page/layout keys inside
                    // `[header_formatting]`.
                    if let Some(hf_table) = value.get("header_formatting").and_then(|v| v.as_table()).cloned() {
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
                            // If the file places the include_* toggles inside the
                            // `[header_formatting]` table (common), ensure those
                            // values are propagated into the top-level `Config` fields
                            // when the top-level ones are not present.
                            if let Some(hf) = cfg.header_formatting.clone() {
                                cfg.include_design = cfg.include_design.or(hf.include_design);
                                cfg.include_engraving = cfg.include_engraving.or(hf.include_engraving);
                                cfg.include_watermark = cfg.include_watermark.or(hf.include_watermark);
                                cfg.include_perforation = cfg.include_perforation.or(hf.include_perforation);
                                cfg.include_catalog_numbers = cfg.include_catalog_numbers.or(hf.include_catalog_numbers);
                                cfg.include_series = cfg.include_series.or(hf.include_series);
                                cfg.include_country = cfg.include_country.or(hf.include_country);
                                cfg.include_date = cfg.include_date.or(hf.include_date);
                                cfg.print_flag_cover = cfg.print_flag_cover.or(hf.print_flag_cover);
                            // header_formatting currently does not include the detailed
                            // diagnostics toggle; leave it as-is when present at top-level.
                            }

                            info!("Loaded config from {}", candidate.display());
                            debug!("Parsed Config struct: {:?}", cfg);
                            return cfg;
                            }
                            Err(e) => {
                                println!("[WARN] Failed to deserialize merged config from {}: {} -- using defaults", candidate.display(), e);
                                break;
                            }
                        },
                        Err(e) => {
                            println!("[WARN] Failed to serialize merged toml::Value for {}: {} -- using defaults", candidate.display(), e);
                            break;
                        }
                    }
                }
                Err(e) => {
                    println!("[DEBUG] Failed to parse into toml::Value (this shouldn't happen when parsing into Config succeeded): {}", e);
                }
            }
        }
    }

    println!("[INFO] config.toml not found in parent dirs or failed to parse, using default values");
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
        if let Ok(mut dir) = std::env::current_dir() {
            for _ in 0..8 {
                paths.push(dir.join(path));
                if !dir.pop() { break; }
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
                        println!("[INFO] Loaded slug_overrides from {} ({} entries)", candidate.display(), out.len());
                        return out;
                    }
                }
            }
        }
    }
    HashMap::new()
}
