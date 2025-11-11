//! Album creation and file writing functionality.
//!
//! This module contains functions for creating album files and writing
//! stamp data to various formats.

use crate::album_globals;
use crate::commands_file_generator::image_utils::SeriesInfo;
use crate::commands_file_generator::utils::{
    fix_color, get_catalog_num, is_selected_emission, is_selected_format,
};
use log::{error, info};
use std::io::Write;

/// Create an album file from stamp data
pub fn create_album<P: AsRef<std::path::Path>>(
    file_path: P,
    data: &[String],
    album_name: &str,
    separator: &str,
    fix_sizes: bool,
    formats: &[String],
    emissions: &[String],
    catalog_name: &str,
) -> usize {
    use std::fs::File;
    use std::io::{BufWriter, Write};
    let path = file_path.as_ref();
    let file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            error!("Unable to open output file: {}", e);
            std::process::exit(1);
        }
    };
    let mut writer = BufWriter::new(file);
    let page_count = write_globals(&mut writer, album_name);
    let mut prev_year = String::new();
    let mut prev_series = String::new();
    let mut prev_date = String::new();
    let mut width = "0".to_string();
    let mut height = "0".to_string();
    let mut series_info: Option<SeriesInfo> = None;
    // Add required mutable counters and month_names array
    let mut rows_written = 0;
    let mut stamps_written = 0;
    let mut used_width = 0;
    let mut used_height = 0;
    let mut max_row_height = 0;
    let _month_names = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    for (idx, line) in data.iter().enumerate() {
        if line.starts_with("STAMP") {
            let fields: Vec<&str> = line.split(separator).collect();
            if fields.len() < 24 {
                continue;
            }
            let year = fields[1];
            let month = fields[2];
            let day = fields[3];
            let series = fields[4];
            let date = format!("{}{}{}", year, month, day);
            let _catalog_num = if catalog_name.is_empty() {
                fields[5].to_string()
            } else {
                get_catalog_num(fields[15], false)
            };
            let count;
            let multi_perf;
            if year != prev_year {
                album_globals::new_page(
                    &mut writer,
                    &mut rows_written,
                    &mut stamps_written,
                    &mut used_width,
                    &mut used_height,
                    &mut max_row_height,
                );
                album_globals::new_year(&mut writer, year, None);
                series_info = Some(get_series_info_stub(data, idx, &width, &height));
                count = series_info.as_ref().map_or(1, |s| s.count);
                multi_perf = series_info.as_ref().is_some_and(|s| s.multi_perf);
                width = series_info
                    .as_ref()
                    .map_or("0".to_string(), |s| s.width.clone());
                height = series_info
                    .as_ref()
                    .map_or("0".to_string(), |s| s.height.clone());
                album_globals::new_series(
                    &mut writer,
                    series,
                    year,
                    month,
                    day,
                    count,
                    multi_perf,
                    fields[12],
                    &mut rows_written,
                    &mut stamps_written,
                    &mut used_width,
                    &mut used_height,
                    &mut max_row_height,
                );
                prev_year = year.to_string();
                prev_series = series.to_string();
                prev_date = date.clone();
            } else if series != prev_series || date != prev_date {
                series_info = Some(get_series_info_stub(data, idx, &width, &height));
                count = series_info.as_ref().map_or(1, |s| s.count);
                multi_perf = series_info.as_ref().is_some_and(|s| s.multi_perf);
                width = series_info
                    .as_ref()
                    .map_or("0".to_string(), |s| s.width.clone());
                height = series_info
                    .as_ref()
                    .map_or("0".to_string(), |s| s.height.clone());
                album_globals::new_series(
                    &mut writer,
                    series,
                    year,
                    month,
                    day,
                    count,
                    multi_perf,
                    fields[12],
                    &mut rows_written,
                    &mut stamps_written,
                    &mut used_width,
                    &mut used_height,
                    &mut max_row_height,
                );
                prev_series = series.to_string();
                prev_date = date.clone();
            }
            let mut stamp_width = fields[10].to_string();
            let mut stamp_height = fields[11].to_string();
            if fix_sizes && series_info.as_ref().map_or(1, |s| s.count) > 1 && stamp_width == "0" {
                stamp_width = width.clone();
                stamp_height = height.clone();
            }
            let catalog_num = get_catalog_num(fields[15], true);
            if !catalog_num.is_empty()
                && catalog_num != "Unlisted"
                && is_selected_format(formats, fields[13])
                && is_selected_emission(emissions, fields[14])
            {
                album_globals::add_stamp(
                    &mut writer,
                    year,
                    fields[6],
                    fields[7],
                    fields[8],
                    fix_color(fields[9]),
                    series_info.as_ref().is_some_and(|s| s.multi_perf),
                    fields[12],
                    &stamp_width,
                    &stamp_height,
                    &catalog_num,
                    fields[17],
                );
            }
        }
    }
    if let Err(e) = writer.flush() {
        error!("Unable to close output file: {}", e);
        std::process::exit(1);
    }
    // snooze(500); // This would need to be handled by caller
    page_count
}

/// Write global album configuration
pub fn write_globals<W: Write>(writer: &mut W, album_name: &str) -> usize {
    let _cfg = crate::config::load_config("config.toml");
    // No DB connection or explicit year range available here: pass None values
    crate::album_globals::write_globals(writer, album_name, None, None, None)
}

/// Save stamp data to a file
pub fn save_stamp_data<P: AsRef<std::path::Path>>(
    file_path: P,
    data: &[String],
    source: &str,
    overwrite_album: bool,
    data_source: &str,
) {
    use std::fs::{self, File};
    use std::io::{BufWriter, Write};
    use std::os::unix::fs::PermissionsExt;
    let path = file_path.as_ref();

    // Decide how to handle an existing target file:
    // - If overwrite_album == true: overwrite the existing file in-place (File::create will truncate).
    // - If overwrite_album == false (default) and the file exists: create a new versioned filename
    //   by appending a datetime stamp before the extension so the original file's creation time
    //   is preserved as the "original".
    let mut target_path = path.to_path_buf();
    if path.exists() {
        if overwrite_album {
            // Intentionally overwrite the file in-place. No special handling needed;
            // File::create below will truncate the existing file.
            info!("Overwriting existing file {} as requested", path.display());
            // If the existing file is read-only, make it writable so we can truncate/overwrite it.
            if let Ok(metadata) = std::fs::metadata(path) {
                let mut perms = metadata.permissions();
                // add owner write bit (rw-r--r--)
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut mode = perms.mode();
                    mode |= 0o200; // owner write
                    perms.set_mode(mode);
                    if let Err(e) = std::fs::set_permissions(path, perms.clone()) {
                        error!("Failed to set file writable for overwrite: {}", e);
                    }
                }
                #[cfg(not(unix))]
                {
                    // On non-unix platforms, attempt to set writable via readonly flag
                    perms.set_readonly(false);
                    if let Err(e) = std::fs::set_permissions(path, perms.clone()) {
                        error!("Failed to clear readonly flag for overwrite: {}", e);
                    }
                }
            }
        } else {
            // Create a timestamped copy name: insert `.created-YYYYMMDD-HHMMSS` before extension.
            use chrono::Local;
            let ts = Local::now().format("%Y%m%d-%H%M%S").to_string();
            if let Some(parent) = path.parent() {
                if let Some(stem_os) = path.file_stem() {
                    if let Some(stem) = stem_os.to_str() {
                        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                        let candidate_name = if ext.is_empty() {
                            format!("{}.created-{}", stem, ts)
                        } else {
                            format!("{}.created-{}.{}", stem, ts, ext)
                        };
                        let candidate = parent.join(&candidate_name);
                        target_path = candidate;
                    }
                }
            }
        }
    }

    // If there is nothing to save, avoid overwriting or creating new files.
    if data.is_empty() {
        info!(
            "No stamp data to save for {}; leaving any existing file intact",
            target_path.display()
        );
        return;
    }

    let file = File::create(&target_path);
    match file {
        Ok(f) => {
            let mut writer = BufWriter::new(f);
            if let Err(e) = writeln!(writer, "SOURCE:{}", data_source) {
                error!("Unable to write source line: {}", e);
            }
            for line in data {
                if let Err(e) = writeln!(writer, "{}", line) {
                    error!("Unable to write stamp data: {}", e);
                }
            }
            if let Err(e) = writer.flush() {
                error!("Unable to flush file: {}", e);
            }
            if let Ok(metadata) = fs::metadata(&target_path) {
                let mut perms = metadata.permissions();
                perms.set_mode(0o444); // read-only for all
                if let Err(e) = fs::set_permissions(&target_path, perms) {
                    error!("Unable to set file to read-only: {}", e);
                }
            }
            info!("Successfully saved stamp data to {}", target_path.display());
        }
        Err(e) => {
            error!("Unable to save extract file: {}", e);
        }
    }
}

/// Simple series-info extractor stub. Real implementation inspects neighboring lines.
pub fn get_series_info_stub(
    _data: &[String],
    _idx: usize,
    width: &str,
    height: &str,
) -> SeriesInfo {
    SeriesInfo {
        count: 1,
        multi_perf: false,
        width: width.to_string(),
        height: height.to_string(),
    }
}
