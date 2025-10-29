use crate::album_globals;
use crate::album_utils::sanitize_filename;
use crate::types::{ImageStatus, OSType};
use log::{debug, error, info};
use std::io::Write;

// Native blocking downloader helper: streams to a temp file and atomically
// renames into place. Retries on transient failures.
fn download_to_path(url: &str, dest: &std::path::Path, _no_ssl_check: bool) -> Result<(), String> {
    use reqwest::blocking::Client;
    use std::io::{self};
    use std::time::Duration;
    use tempfile::NamedTempFile;

    let client_builder = Client::builder().timeout(Duration::from_secs(30));
    let client = client_builder
        .build()
        .map_err(|e| format!("failed to build client: {}", e))?;

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create dir {}: {}", parent.display(), e))?;
    }

    let mut last_err: Option<String> = None;
    for attempt in 1..=3 {
        match client.get(url).send() {
            Ok(mut resp) => {
                if !resp.status().is_success() {
                    last_err = Some(format!("HTTP {}", resp.status()));
                } else {
                    match NamedTempFile::new_in(dest.parent().unwrap_or(std::path::Path::new(".")))
                    {
                        Ok(mut tmp) => {
                            let copy_res = io::copy(&mut resp, &mut tmp);
                            if let Err(e) = copy_res {
                                last_err = Some(format!("io copy error: {}", e));
                            } else {
                                if let Err(e) = tmp.persist(dest) {
                                    last_err = Some(format!("persist error: {}", e));
                                } else {
                                    return Ok(());
                                }
                            }
                        }
                        Err(e) => last_err = Some(format!("tempfile create error: {}", e)),
                    }
                }
            }
            Err(e) => last_err = Some(format!("request error: {}", e)),
        }
        std::thread::sleep(Duration::from_millis(500 * attempt));
    }
    Err(last_err.unwrap_or_else(|| "unknown error".to_string()))
}
pub struct SeriesInfo {
    pub count: i32,
    pub multi_perf: bool,
    pub width: String,
    pub height: String,
}

/// Represents the status of an image operation (stub)
pub fn get_colnect_image(images_directory: &str, url: &str, name: &str) -> ImageStatus {
    let clean_name = name.replace(' ', "");
    let file_path = format!("{}/{}.jpg", images_directory, clean_name);
    get_image(url, name, &file_path)
}

/// Stub for get_image, to be implemented
pub fn get_image(_url: &str, _name: &str, _file_path: &str) -> ImageStatus {
    // Debug: Show the process of accessing the external website and getting the image
    debug!("Attempting to fetch image from URL: {}", _url);
    debug!("Image will be saved as: {}", _file_path);
    // Example: Use reqwest to fetch the image
    match reqwest::blocking::get(_url) {
        Ok(resp) => {
            debug!("HTTP status: {}", resp.status());
            if resp.status().is_success() {
                match resp.bytes() {
                    Ok(bytes) => {
                        match std::fs::write(_file_path, &bytes) {
                            Ok(_) => debug!("Image saved successfully."),
                            Err(e) => error!("Failed to save image: {}", e),
                        }
                        ImageStatus {
                            link: _url.to_string(),
                            exists: true,
                            width: 0,
                            height: 0,
                            filename: _name.to_string(),
                        }
                    }
                    Err(e) => {
                        error!("Failed to read image bytes: {}", e);
                        ImageStatus {
                            link: _url.to_string(),
                            exists: false,
                            width: 0,
                            height: 0,
                            filename: _name.to_string(),
                        }
                    } // Remove duplicate ImageStatus struct
                      // pub struct ImageStatus {
                      //     pub status: String,
                      //     pub message: String,
                      // }
                }
            } else {
                error!("HTTP request failed with status: {}", resp.status());
                ImageStatus {
                    link: _url.to_string(),
                    exists: false,
                    width: 0,
                    height: 0,
                    filename: _name.to_string(),
                }
            }
        }
        Err(e) => {
            error!("HTTP request error: {}", e);
            crate::types::ImageStatus {
                link: _url.to_string(),
                exists: false,
                width: 0,
                height: 0,
                filename: String::new(),
            }
        }
    }
}
#[derive(Debug, Default)]
pub struct CommandsFileGenerator {
    pub detected_os: Option<OSType>,
    pub file_separator: String,
    pub data_source: String,
    pub file_source: String,
    pub album_directory: String,
    pub images_directory: String,
    pub album_name: String,
    pub album_file_name: String,
    pub extract_file_name: String,
    pub export_file_name: String,
    pub site_name: String,
    pub catalog_name: String,
    pub no_images: bool,
    pub fix_sizes: bool,
    pub no_ssl_check: bool,
    pub separator: String,
    pub rows_written: i32,
    pub stamps_written: i32,
    pub used_width: i32,
    pub used_height: i32,
    pub max_row_height: i32,
    pub sequencer: i32,
    pub start_year: i32,
    pub end_year: i32,
    pub formats: Vec<String>,
    pub emissions: Vec<String>,
    pub currencies: Vec<String>,
    pub month_names: Vec<String>,
    pub valid_formats: Vec<String>,
    pub valid_emissions: Vec<String>,
    pub touchstamps_data_file: Vec<String>,
}

impl CommandsFileGenerator {
    pub fn month_number(&self, month: &str) -> String {
        // Stub: convert month name to number as string
        match month.to_lowercase().as_str() {
            "january" => "1".to_string(),
            "february" => "2".to_string(),
            "march" => "3".to_string(),
            "april" => "4".to_string(),
            "may" => "5".to_string(),
            "june" => "6".to_string(),
            "july" => "7".to_string(),
            "august" => "8".to_string(),
            "september" => "9".to_string(),
            "october" => "10".to_string(),
            "november" => "11".to_string(),
            "december" => "12".to_string(),
            _ => "0".to_string(),
        }
    }

    pub fn get_stampworld_image(&self, url: &str, overwrite: bool) -> crate::types::ImageStatus {
        use std::path::Path;
        // If the caller passed an id (not a full URL), build the canonical StampWorld CDN URL
        let remote_url = if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else {
            // Java implementation used: https://swmedia-4cd6.kxcdn.com/media/catalogue/{siteName}/Postage-stamps/{id}-s.jpg
            // Keep site_name as provided by the caller for compatibility.
            format!(
                "https://swmedia-4cd6.kxcdn.com/media/catalogue/{}/Postage-stamps/{}-s.jpg",
                self.site_name, url
            )
        };

        let raw_image_name = remote_url.split('/').last().unwrap_or("stampworld.jpg");
        // Normalize filename: strip an extra trailing "-s" before the extension so we don't end up with "-s-s.jpg"
        let mut image_name = sanitize_filename(raw_image_name);
        if image_name.to_lowercase().ends_with("-s.jpg") {
            if let Some(stripped) = image_name.strip_suffix("-s.jpg") {
                image_name = format!("{}.jpg", stripped);
            }
        }
        let file_path = format!("{}/{}", self.images_directory, image_name);
        if !overwrite && Path::new(&file_path).exists() {
            debug!("Skipping download, file exists: {}", file_path);
            // Try to compute dimensions from the existing file so callers get real values
            if let Some((w, h)) = crate::image_utils::get_image_dimensions(&file_path) {
                return crate::types::ImageStatus {
                    link: url.to_string(),
                    exists: true,
                    width: w as i32,
                    height: h as i32,
                    filename: image_name.to_string(),
                };
            } else {
                return crate::types::ImageStatus {
                    link: url.to_string(),
                    exists: true,
                    width: 0,
                    height: 0,
                    filename: image_name.to_string(),
                };
            }
        }
        self.get_image(&remote_url, &image_name, &file_path)
    }

    pub fn get_stampworld_image_with_filename(
        &self,
        url: &str,
        filename: &str,
        overwrite: bool,
    ) -> ImageStatus {
        use std::path::Path;
        let mut filename = filename.to_string();
        // Normalize any trailing -s.jpg in provided filename to avoid duplication
        if filename.to_lowercase().ends_with("-s.jpg") {
            if let Some(stripped) = filename.strip_suffix("-s.jpg") {
                filename = format!("{}.jpg", stripped);
            }
        }
        let file_path = format!("{}/{}", self.images_directory, filename);

        // If overwrite is disabled and file exists, skip download but try to get dimensions
        if !overwrite && Path::new(&file_path).exists() {
            info!("Skipping download, image already exists: {}", file_path);
            if let Some((w, h)) = crate::image_utils::get_image_dimensions(&file_path) {
                return ImageStatus {
                    link: url.to_string(),
                    exists: true,
                    width: w as i32,
                    height: h as i32,
                    filename: filename.to_string(),
                };
            } else {
                return ImageStatus {
                    link: url.to_string(),
                    exists: true,
                    width: 0,
                    height: 0,
                    filename: filename.to_string(),
                };
            }
        }

        // If `url` is an id rather than a full URL, build the CDN fallback
        let remote_url = if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else {
            format!(
                "https://swmedia-4cd6.kxcdn.com/media/catalogue/{}/Postage-stamps/{}-s.jpg",
                self.site_name, url
            )
        };

        self.get_image(&remote_url, &filename, &file_path)
    }

    pub fn collect_stampworld_data(
        &mut self,
        base_url: &str,
        overwrite_images: bool,
        page_callback: Option<&dyn Fn(usize)>,
        max_pages: i32,
        stop_after_no_hits: i32,
    ) -> Vec<String> {
        let separator = self.separator.clone();
        let images_directory = self.images_directory.clone();
        let start_year = self.start_year;
        let end_year = self.end_year;
        let month_number = |month: &str| match month.to_lowercase().as_str() {
            "january" => "1".to_string(),
            "february" => "2".to_string(),
            "march" => "3".to_string(),
            "april" => "4".to_string(),
            "may" => "5".to_string(),
            "june" => "6".to_string(),
            "july" => "7".to_string(),
            "august" => "8".to_string(),
            "september" => "9".to_string(),
            "october" => "10".to_string(),
            "november" => "11".to_string(),
            "december" => "12".to_string(),
            _ => "0".to_string(),
        };
        // Use the CommandsFileGenerator's real downloader so images are fetched.
        // To avoid borrowing `self` while it's mutably borrowed for sequencer,
        // capture the site_name and images_directory and call a temporary
        // CommandsFileGenerator instance to perform the download.
        let captured_site = self.site_name.clone();
        let captured_images_dir = images_directory.clone();
        let downloader = move |url: &str, overwrite: bool| {
            let mut tmp = CommandsFileGenerator::new();
            tmp.site_name = captured_site.clone();
            tmp.images_directory = captured_images_dir.clone();
            tmp.get_stampworld_image(url, overwrite)
        };
        crate::stampworld::collect_stampworld_data(
            &mut self.sequencer,
            &separator,
            &self.site_name,
            &images_directory,
            &month_number,
            &downloader,
            base_url,
            start_year,
            end_year,
            overwrite_images,
            page_callback,
            max_pages,
            stop_after_no_hits,
        )
    }
    pub fn new() -> Self {
        Self {
            detected_os: None,
            file_separator: "@".to_string(),
            data_source: String::new(),
            file_source: String::new(),
            album_directory: String::new(),
            images_directory: String::new(),
            album_name: String::new(),
            album_file_name: String::new(),
            extract_file_name: String::new(),
            export_file_name: String::new(),
            site_name: String::new(),
            catalog_name: String::new(),
            no_images: false,
            fix_sizes: false,
            no_ssl_check: false,
            separator: "@".to_string(),
            rows_written: 0,
            stamps_written: 0,
            used_width: 0,
            used_height: 0,
            max_row_height: 0,
            sequencer: 0,
            start_year: 0,
            end_year: 9999,
            formats: Vec::new(),
            emissions: Vec::new(),
            currencies: Vec::new(),
            month_names: vec![
                "January".to_string(),
                "February".to_string(),
                "March".to_string(),
                "April".to_string(),
                "May".to_string(),
                "June".to_string(),
                "July".to_string(),
                "August".to_string(),
                "September".to_string(),
                "October".to_string(),
                "November".to_string(),
                "December".to_string(),
            ],
            valid_formats: Vec::new(),
            valid_emissions: Vec::new(),
            touchstamps_data_file: Vec::new(),
        }
    }
    pub fn create_album<P: AsRef<std::path::Path>>(&mut self, file_path: P, data: &[String]) {
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
        self.write_globals(&mut writer);
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
                let fields: Vec<&str> = line.split(&self.separator).collect();
                if fields.len() < 24 {
                    continue;
                }
                let year = fields[1];
                let month = fields[2];
                let day = fields[3];
                let series = fields[4];
                let date = format!("{}{}{}", year, month, day);
                let _catalog_num = if self.catalog_name.is_empty() {
                    fields[5].to_string()
                } else {
                    self.get_catalog_num(fields[15], false)
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
                    series_info = Some(self.get_series_info(data, idx, &width, &height));
                    count = series_info.as_ref().map_or(1, |s| s.count);
                    multi_perf = series_info.as_ref().map_or(false, |s| s.multi_perf);
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
                    series_info = Some(self.get_series_info(data, idx, &width, &height));
                    count = series_info.as_ref().map_or(1, |s| s.count);
                    multi_perf = series_info.as_ref().map_or(false, |s| s.multi_perf);
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
                if self.fix_sizes
                    && series_info.as_ref().map_or(1, |s| s.count) > 1
                    && stamp_width == "0"
                {
                    stamp_width = width.clone();
                    stamp_height = height.clone();
                }
                let catalog_num = self.get_catalog_num(fields[15], true);
                if catalog_num != ""
                    && catalog_num != "Unlisted"
                    && self.is_selected_format(fields[13])
                    && self.is_selected_emission(fields[14])
                {
                    album_globals::add_stamp(
                        &mut writer,
                        year,
                        fields[6],
                        fields[7],
                        fields[8],
                        self.fix_color(fields[9].into()),
                        series_info.as_ref().map_or(false, |s| s.multi_perf),
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
        self.snooze(500);
    }
    /// Stub for write_globals helper
    pub fn write_globals<W: Write>(&self, _writer: &mut W) {
        let _cfg = crate::config::load_config("config.toml");
        // No DB connection or explicit year range available here: pass None values
        let _ = crate::album_globals::write_globals(_writer, &self.album_name, None, None, None);
    }

    /// Stub for is_selected_format helper
    pub fn is_selected_format(&self, field: &str) -> bool {
        if self.formats.is_empty() {
            true
        } else {
            self.formats.iter().any(|f| f.eq_ignore_ascii_case(field))
        }
    }

    /// Stub for is_selected_emission helper
    pub fn is_selected_emission(&self, field: &str) -> bool {
        if self.emissions.is_empty() {
            true
        } else {
            self.emissions.iter().any(|e| e.eq_ignore_ascii_case(field))
        }
    }

    pub fn save_stamp_data<P: AsRef<std::path::Path>>(
        &self,
        file_path: P,
        data: &[String],
        _source: &str,
        overwrite_album: bool,
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

        let file = File::create(&target_path);
        match file {
            Ok(f) => {
                let mut writer = BufWriter::new(f);
                if let Err(e) = writeln!(writer, "SOURCE:{}", self.data_source) {
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

    /// Backwards-compatible wrapper for older call sites that pass 3 args
    pub fn save_stamp_data_default<P: AsRef<std::path::Path>>(
        &self,
        file_path: P,
        data: &[String],
        _source: &str,
    ) {
        self.save_stamp_data(file_path, data, _source, false)
    }

    /// Return a catalog number for a record. Simple stub used by the commands generator.
    pub fn get_catalog_num(&self, field: &str, _prefer_full: bool) -> String {
        field.to_string()
    }

    /// Simple series-info extractor stub. Real implementation inspects neighboring lines.
    pub fn get_series_info(
        &self,
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
    pub fn get_image(&self, url: &str, name: &str, file_path: &str) -> crate::types::ImageStatus {
        use image::io::Reader as ImageReader;
        use std::fs;
        use std::path::Path;

        if name.is_empty() {
            return crate::types::ImageStatus {
                link: String::new(),
                exists: false,
                width: 0,
                height: 0,
                filename: String::new(),
            };
        }
        // Ensure the images directory (parent of file_path) exists before any downloads
        if let Some(parent) = Path::new(file_path).parent() {
            if !parent.exists() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    error!(
                        "Failed to create image directory {}: {}",
                        parent.display(),
                        e
                    );
                }
            }
        }

        // Initialize result vars
        let mut exists: bool = false;
        let mut width: i32 = 0;
        let mut height: i32 = 0;

        // Compute local checksum if file exists
        use sha2::{Digest, Sha256};
        use std::fs::File;
        use std::io::Read;
        let mut local_sum = String::new();
        if Path::new(file_path).exists() {
            if let Ok(mut f) = File::open(file_path) {
                let mut hasher = Sha256::new();
                let mut buf = [0u8; 8192];
                loop {
                    match f.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => {
                            hasher.update(&buf[..n]);
                        }
                        Err(_) => break,
                    }
                }
                local_sum = format!("{:x}", hasher.finalize());
            }
            if file_path.contains("no_image") {
                local_sum.clear();
            }
        }

        // Attempt to download/refresh the file
        let dest = std::path::Path::new(file_path);
        match download_to_path(url, dest, self.no_ssl_check) {
            Ok(()) => {
                exists = true;
            }
            Err(e) => {
                error!("Download failed for {}: {}", url, e);
                // leave exists=false; caller will use placeholder
            }
        }

        // Read image dimensions
        let img_file = std::path::Path::new(file_path);
        match ImageReader::open(img_file) {
            Ok(reader) => match reader.decode() {
                Ok(img) => {
                    width = img.width() as i32;
                    height = img.height() as i32;
                    info!(
                        "Image dimensions: {} x {} pixels - {} ",
                        width,
                        height,
                        img_file.display()
                    );
                }
                Err(e) => {
                    error!("Error decoding image: {}", e);
                    exists = false;
                }
            },
            Err(e) => {
                error!("Error opening image file: {}", e);
                exists = false;
            }
        }
        if exists {
            exists = self.is_valid_jpeg(file_path);
        }
        if !exists && img_file.exists() {
            let _ = fs::remove_file(img_file);
        }
        crate::types::ImageStatus {
            link: url.to_string(),
            exists,
            width,
            height,
            filename: name.to_string(),
        }
    }

    /// Stub for is_valid_jpeg helper
    pub fn is_valid_jpeg(&self, _file_path: &str) -> bool {
        use std::fs::File;
        use std::io::Read;
        let mut valid = true;
        match File::open(_file_path) {
            Ok(mut file) => {
                let mut buf = [0u8; 4];
                if let Err(_) = file.read_exact(&mut buf) {
                    valid = false;
                } else {
                    // Java's -2555936 is 0xFFD8FFE0 (JPEG SOI + JFIF marker)
                    let magic = i32::from_be_bytes(buf);
                    if magic != -2555936 {
                        valid = false;
                    }
                }
            }
            Err(_) => {
                valid = false;
            }
        }
        valid
    }
    pub fn get_touchstamps_stamps(
        &mut self,
        link: &str,
        year: &str,
        month: &str,
        day: &str,
        desc: &str,
    ) -> Vec<String> {
        use reqwest::blocking::Client;
        use scraper::{Html, Selector};

        let client = Client::new();
        let url = format!("https://touchstamps.com{}", link);
        let document = match client
            .get(&url)
            .timeout(std::time::Duration::from_secs(60))
            .send()
        {
            Ok(resp) => match resp.text() {
                Ok(body) => Html::parse_document(&body),
                Err(_) => {
                    error!("Problem retrieving web page");
                    std::process::exit(1);
                }
            },
            Err(e) => {
                error!("Problem retrieving web page: {}", e);
                std::process::exit(1);
            }
        };

        let article_selector = Selector::parse("article.contentColumn.item").unwrap();
        let mut all_stamp_data = Vec::new();
        for article in document.select(&article_selector) {
            let stamp_name = article
                .select(&Selector::parse("header.round-header").unwrap())
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();
            let right_section_html = article
                .select(&Selector::parse("section.right").unwrap())
                .next()
                .map(|e| e.html())
                .unwrap_or_default();
            let (mut image_url, mut image_id) = (String::new(), String::new());
            if right_section_html.contains("image_url") {
                let start = right_section_html.find("image_url").unwrap() + 10;
                let end = right_section_html[start..].find('"').unwrap() + start;
                image_url = right_section_html[start..end].to_string();
                let parts: Vec<&str> = image_url.split('/').collect();
                image_id = parts.last().unwrap_or(&"").to_string();
            } else if right_section_html.contains("src") {
                let start = right_section_html.find("src").unwrap() + 6;
                let end = right_section_html[start..].find(' ').unwrap_or(0) + start;
                image_url = right_section_html[start..end.saturating_sub(1)].to_string();
                image_id = image_url.get(15..).unwrap_or("").to_string();
            }

            let image_status = self.get_touchstamps_image(&image_url, &image_id);
            let image_link = image_status.link;
            let image_exists = image_status.exists;
            let image_width = image_status.width.to_string();
            let image_height = image_status.height.to_string();
            let catalog_code = format!("Ts:{}", image_id);

            let table_selector =
                Selector::parse("table.table.table-striped.table-condensed").unwrap();
            let table = article.select(&table_selector).next();
            let (mut format, mut perforation, mut emission) =
                (String::new(), String::new(), String::new());
            let (face_value, currency) = ("0".to_string(), "0".to_string());
            let value_main = String::new();
            let value_rest = String::new();
            if let Some(table_elem) = table {
                format = table_elem
                    .select(&Selector::parse("tr:contains(Format:)").unwrap())
                    .next()
                    .map(|e| e.text().collect::<String>().replace("Format: ", ""))
                    .unwrap_or_default();
                perforation = table_elem
                    .select(&Selector::parse("tr:contains(Perforation:)").unwrap())
                    .next()
                    .map(|e| e.text().collect::<String>().replace("Perforation: ", ""))
                    .unwrap_or_default();
                emission = table_elem
                    .select(&Selector::parse("tr:contains(Emission:)").unwrap())
                    .next()
                    .map(|e| e.text().collect::<String>().replace("Emission: ", ""))
                    .unwrap_or_default();
            }
            let mut stamp_data = vec![format!(
                "STAMP;{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{}",
                year,
                month,
                day,
                desc,
                catalog_code,
                image_link,
                image_exists,
                image_width,
                image_height,
                format,
                perforation,
                emission,
                face_value,
                currency,
                value_main,
                value_rest
            )];
            if image_exists {
                let image_status = self.get_image(
                    &image_link,
                    &stamp_name,
                    &format!("{}/{}", self.images_directory, image_id),
                );
                if image_status.exists {
                    stamp_data.push(format!(
                        "IMAGE;{};{};{};{};{};{};{}",
                        year, month, day, desc, image_id, image_status.width, image_status.height
                    ));
                }
            }
            all_stamp_data.extend(stamp_data);
        }
        // Save all data once at the end
        let file_path = format!("{}_data.txt", self.album_name);
        self.save_stamp_data_default(&file_path, &all_stamp_data, "touchstamps");
        all_stamp_data
    }
    pub fn fix_color(&self, color: &str) -> String {
        // Stub: just return the color for now
        color.to_string()
    }

    pub fn snooze(&self, millis: u64) {
        std::thread::sleep(std::time::Duration::from_millis(millis));
    }

    pub fn get_touchstamps_image(
        &self,
        image_url: &str,
        _image_id: &str,
    ) -> crate::types::ImageStatus {
        // Stub: just return a default ImageStatus for now
        crate::types::ImageStatus {
            link: image_url.to_string(),
            exists: false,
            width: 0,
            height: 0,
            filename: String::new(),
        }
    }
}
