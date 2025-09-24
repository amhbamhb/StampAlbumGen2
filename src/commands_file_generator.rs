use log::{debug, error, info};
use std::io::Write;
use crate::album_globals;
use crate::album_utils::sanitize_filename;
use crate::types::{ImageStatus, OSType};
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
                    }
                        // Remove duplicate ImageStatus struct
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
            album_globals::ImageStatus {
                link: _url.to_string(),
                exists: false,
                width: 0,
                height: 0,
            }
        }
    }
}#[derive(Debug, Default)]
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

    pub fn get_stampworld_image(&self, url: &str, overwrite: bool) -> album_globals::ImageStatus {        use std::path::Path;
        // If the caller passed an id (not a full URL), build the canonical StampWorld CDN URL
        let remote_url = if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else {
            // Java implementation used: https://swmedia-4cd6.kxcdn.com/media/catalogue/{siteName}/Postage-stamps/{id}-s.jpg
            // Keep site_name as provided by the caller for compatibility.
            format!("https://swmedia-4cd6.kxcdn.com/media/catalogue/{}/Postage-stamps/{}-s.jpg", self.site_name, url)
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
            return ImageStatus { link: url.to_string(), exists: true, width: 0, height: 0, filename: image_name.to_string() };        }
        self.get_image(&remote_url, &image_name, &file_path)
    }
    
    pub fn get_stampworld_image_with_filename(&self, url: &str, filename: &str, overwrite: bool) -> ImageStatus {
        use std::path::Path;
        let mut filename = filename.to_string();
        // Normalize any trailing -s.jpg in provided filename to avoid duplication
        if filename.to_lowercase().ends_with("-s.jpg") {
            if let Some(stripped) = filename.strip_suffix("-s.jpg") {
                filename = format!("{}.jpg", stripped);
            }
        }
        let file_path = format!("{}/{}", self.images_directory, filename);
        
        // If overwrite is disabled and file exists, skip download
        if !overwrite && Path::new(&file_path).exists() {
            info!("Skipping download, image already exists: {}", file_path);
            return ImageStatus { link: url.to_string(), exists: true, width: 0, height: 0, filename: filename.to_string() };
        }
        
        // If `url` is an id rather than a full URL, build the CDN fallback
        let remote_url = if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else {
            format!("https://swmedia-4cd6.kxcdn.com/media/catalogue/{}/Postage-stamps/{}-s.jpg", self.site_name, url)
        };

        self.get_image(&remote_url, &filename, &file_path)
    }
    
    pub fn collect_stampworld_data(&mut self, base_url: &str, overwrite_images: bool, page_callback: Option<&dyn Fn(usize)>) -> Vec<String> {
        let separator = self.separator.clone();
        let images_directory = self.images_directory.clone();
        let start_year = self.start_year;
        let end_year = self.end_year;
        let month_number = |month: &str| {
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
        };
        let get_stampworld_image = |url: &str, overwrite: bool| {
            let raw_image_name = url.split('/').last().unwrap_or("stampworld.jpg");
            let image_name = sanitize_filename(raw_image_name);
            let file_path = format!("{}/{}", images_directory, image_name);
            if !overwrite && std::path::Path::new(&file_path).exists() {
                debug!("Skipping download, file exists: {}", file_path);
                return album_globals::ImageStatus { link: url.to_string(), exists: true, width: 0, height: 0 };            }
        };
        crate::stampworld::collect_stampworld_data(
            &mut self.sequencer,
            &separator,
            &self.site_name,
            &images_directory,
            &month_number,
            &get_stampworld_image,
            base_url,
            start_year,
            end_year,
            overwrite_images,
            page_callback,
        )
    }
    pub fn new() -> Self {
        Self {
            detected_os: None,
            file_separator: "@".to_string(),            data_source: String::new(),
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
            month_names: vec!["January".to_string(), "February".to_string(), "March".to_string(), "April".to_string(), "May".to_string(), "June".to_string(), "July".to_string(), "August".to_string(), "September".to_string(), "October".to_string(), "November".to_string(), "December".to_string()],
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
    let month_names = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
        for (idx, line) in data.iter().enumerate() {
            if line.starts_with("STAMP") {
                let fields: Vec<&str> = line.split(&self.separator).collect();
                if fields.len() < 24 { continue; }
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
            album_globals::new_page(&mut writer, &mut rows_written, &mut stamps_written, &mut used_width, &mut used_height, &mut max_row_height);
                    album_globals::new_year(&mut writer, year);
                        series_info = Some(self.get_series_info(data, idx, &width, &height));
                        count = series_info.as_ref().map_or(1, |s| s.count);
                        multi_perf = series_info.as_ref().map_or(false, |s| s.multi_perf);
                        width = series_info.as_ref().map_or("0".to_string(), |s| s.width.clone());
                        height = series_info.as_ref().map_or("0".to_string(), |s| s.height.clone());
            album_globals::new_series(&mut writer, series, year, month, day, count, multi_perf, fields[12], &mut rows_written, &mut stamps_written, &mut used_width, &mut used_height, &mut max_row_height);
                    prev_year = year.to_string();
                    prev_series = series.to_string();
                    prev_date = date.clone();
                album_globals::new_series(&mut writer, series, year, month, day, count, multi_perf, fields[12], &mut rows_written, &mut stamps_written, &mut used_width, &mut used_height, &mut max_row_height);
                prev_year = year.to_string();
                prev_series = series.to_string();
                prev_date = date.clone();
                } else if series != prev_series || date != prev_date {
                        series_info = Some(self.get_series_info(data, idx, &width, &height));
                        count = series_info.as_ref().map_or(1, |s| s.count);
                        multi_perf = series_info.as_ref().map_or(false, |s| s.multi_perf);
                        width = series_info.as_ref().map_or("0".to_string(), |s| s.width.clone());
                        height = series_info.as_ref().map_or("0".to_string(), |s| s.height.clone());
                album_globals::new_series(&mut writer, series, year, month, day, count, multi_perf, fields[12], &mut rows_written, &mut stamps_written, &mut used_width, &mut used_height, &mut max_row_height);
                    prev_series = series.to_string();
                    prev_date = date.clone();
                }
                let mut stamp_width = fields[10].to_string();
                let mut stamp_height = fields[11].to_string();
                if self.fix_sizes && series_info.as_ref().map_or(1, |s| s.count) > 1 && stamp_width == "0" {
                    stamp_width = width.clone();
                    stamp_height = height.clone();
                }
                let catalog_num = self.get_catalog_num(fields[15], true);
                if catalog_num != "" && catalog_num != "Unlisted" && self.is_selected_format(fields[13]) && self.is_selected_emission(fields[14]) {
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
                        fields[17]
                    );
                }
            }
        }
        if let Err(e) = writer.flush() {
            println!("Unable to close output file: {}", e);
            std::process::exit(1);
        }
        self.snooze(500);
    }
    /// Stub for write_globals helper
    pub fn write_globals<W: Write>(&self, _writer: &mut W) {
    let cfg = crate::config::load_config("config.toml");
    let _ = crate::album_globals::write_globals(_writer, &self.album_name);
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

    pub fn save_stamp_data<P: AsRef<std::path::Path>>(&self, file_path: P, data: &[String], _source: &str, overwrite_album: bool) {
        use std::fs::{self, File};
        use std::io::{Write, BufWriter};
        use std::os::unix::fs::PermissionsExt;
        let path = file_path.as_ref();

        if !overwrite_album && path.exists() {
            info!("File {} already exists and overwrite is disabled. Skipping save.", path.display());
            return;
        }

        if overwrite_album && path.exists() {
            if let Err(e) = fs::remove_file(path) {
                error!("Failed to remove existing file for overwrite: {}", e);
                return;
            }
        }

        let file = File::create(path);
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
                if let Ok(metadata) = fs::metadata(path) {
                    let mut perms = metadata.permissions();
                    perms.set_mode(0o444); // read-only for all
                    if let Err(e) = fs::set_permissions(path, perms) {
                        error!("Unable to set file to read-only: {}", e);
                    }
                }
                info!("Successfully saved stamp data to {}", path.display());
            }
            Err(e) => {
                error!("Unable to save extract file: {}", e);
            }
        }
    }
    pub fn get_image(&self, url: &str, name: &str, file_path: &str) -> album_globals::ImageStatus {
        use std::process::Command;
        use std::fs;
        use image::io::Reader as ImageReader;
        use std::path::Path;
        
        if name.is_empty() {
            return album_globals::ImageStatus { link: String::new(), exists: false, width: 0, height: 0 };        }
        
        // If file already exists, compare content with remote and only replace if different
        if Path::new(file_path).exists() {
            info!("Target exists, comparing remote vs local: {}", file_path);
            use sha2::{Digest, Sha256};
            use std::fs::File;
            use std::io::Read;

            // Read local sha
            let mut local_sum = String::new();
            if let Ok(mut f) = File::open(file_path) {
                let mut hasher = Sha256::new();
                let mut buf = [0u8; 8192];
                loop {
                    match f.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => { hasher.update(&buf[..n]); }
                        Err(_) => break,
                    }
                }
                local_sum = format!("{:x}", hasher.finalize());
            }

            // If local looks like a placeholder by name, mark as empty so we'll replace
            if file_path.contains("no_image") {
                local_sum.clear();
            }

            // Download remote to temp file
            let tmp_path = format!("{}{}.tmp", file_path, ".remote");
            let curl_cmd = if self.no_ssl_check {
                vec!["curl", "--ssl-no-revoke", "-L", "-o", &tmp_path, url]
            } else {
                vec!["curl", "-L", "-o", &tmp_path, url]
            };
            let status = Command::new(curl_cmd[0]).args(&curl_cmd[1..]).status();
            if let Err(e) = status {
                error!("Error executing curl for imageURL: {}", e);
            } else if let Ok(s) = status {
                if s.success() {
                    // compute remote sha
                    if let Ok(mut rf) = File::open(&tmp_path) {
                        let mut hasher = Sha256::new();
                        let mut buf = [0u8; 8192];
                        loop {
                            match rf.read(&mut buf) {
                                Ok(0) => break,
                                Ok(n) => { hasher.update(&buf[..n]); }
                                Err(_) => break,
                            }
                        }
                        let remote_sum = format!("{:x}", hasher.finalize());
                        // If different (or local_sum empty), replace
                        if local_sum.is_empty() || remote_sum != local_sum {
                            if let Err(e) = std::fs::rename(&tmp_path, file_path) {
                                // fallback to copy
                                if let Err(e2) = std::fs::copy(&tmp_path, file_path) {
                                    error!("Failed to move/copy temp image into place: {} / {}", e, e2);
                                }
                            }
                            info!("Replaced local image with remote: {}", file_path);
                        } else {
                            // identical; remove temp
                            let _ = std::fs::remove_file(&tmp_path);
                            info!("Remote image identical to local; kept existing: {}", file_path);
                        }
                    }
                } else {
                    error!("Failed to download remote image for comparison: {} exit {:?}", url, s.code());
                }
            }

            // At this point return status based on existence and validation below
        }
        
        // println!("[INFO] Downloading image: {} -> {}", name, file_path);
        
        let curl_cmd = if self.no_ssl_check {
            vec!["curl", "--ssl-no-revoke", "-L", "-o", file_path, url]
        } else {
            vec!["curl", "-L", "-o", file_path, url]
        };
        let mut exists = true;
        let mut width = 0;
        let mut height = 0;
        // Download image
        let status = Command::new(curl_cmd[0])
            .args(&curl_cmd[1..])
            .status();
        if let Err(e) = status {
            error!("Error executing curl for imageURL: {}", e);
            std::process::exit(1);
        }
        
        println!("[INFO] Image downloaded successfully: {}", name);
        
        // Read image dimensions
        let img_file = std::path::Path::new(file_path);
        match ImageReader::open(img_file) {
            Ok(reader) => match reader.decode() {
                Ok(img) => {
                    width = img.width() as i32;
                    height = img.height() as i32;
                    println!("[INFO] Image dimensions: {} x {} pixels", width, height);
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
    album_globals::ImageStatus { link: url.to_string(), exists, width, height }    }

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
    pub fn get_touchstamps_stamps(&mut self, link: &str, year: &str, month: &str, day: &str, desc: &str) -> Vec<String> {
        use reqwest::blocking::Client;
        use scraper::{Html, Selector};

        let client = Client::new();
        let url = format!("https://touchstamps.com{}", link);
        let document = match client.get(&url).timeout(std::time::Duration::from_secs(60)).send() {
            Ok(resp) => match resp.text() {
                Ok(body) => Html::parse_document(&body),
                Err(_) => {
                    eprintln!("Problem retrieving web page");
                    std::process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("Problem retrieving web page: {}", e);
                std::process::exit(1);
            }
        };

        let article_selector = Selector::parse("article.contentColumn.item").unwrap();
        let mut all_stamp_data = Vec::new();
        for article in document.select(&article_selector) {
            let stamp_name = article.select(&Selector::parse("header.round-header").unwrap()).next().map(|e| e.text().collect::<String>()).unwrap_or_default();
            let right_section_html = article.select(&Selector::parse("section.right").unwrap()).next().map(|e| e.html()).unwrap_or_default();
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

            let table_selector = Selector::parse("table.table.table-striped.table-condensed").unwrap();
            let table = article.select(&table_selector).next();
            let (mut format, mut perforation, mut emission) = (String::new(), String::new(), String::new());
            let (face_value, currency) = ("0".to_string(), "0".to_string());
            let value_main = String::new();
            let value_rest = String::new();
            if let Some(table_elem) = table {
                format = table_elem.select(&Selector::parse("tr:contains(Format:)").unwrap())
                    .next()
                    .map(|e| e.text().collect::<String>().replace("Format: ", ""))
                    .unwrap_or_default();
                perforation = table_elem.select(&Selector::parse("tr:contains(Perforation:)").unwrap())
                    .next()
                    .map(|e| e.text().collect::<String>().replace("Perforation: ", ""))
                    .unwrap_or_default();
                emission = table_elem.select(&Selector::parse("tr:contains(Emission:)").unwrap())
                    .next()
                    .map(|e| e.text().collect::<String>().replace("Emission: ", ""))
                    .unwrap_or_default();
            }
            let mut stamp_data = vec![
                format!(
                    "STAMP;{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{}",
                    year, month, day, desc, catalog_code, image_link, image_exists, image_width, image_height, format, perforation, emission, face_value, currency, value_main, value_rest
                ),
            ];
            if image_exists {
                let image_status = self.get_image(&image_link, &stamp_name, &format!("{}/{}", self.images_directory, image_id));
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
        self.save_stamp_data(&file_path, &all_stamp_data, "touchstamps");
        all_stamp_data
    }
    pub fn fix_color(&self, color: &str) -> String {
        // Stub: just return the color for now
        color.to_string()
    }

    pub fn snooze(&self, millis: u64) {
        std::thread::sleep(std::time::Duration::from_millis(millis));
    }

    pub fn get_touchstamps_image(&self, image_url: &str, _image_id: &str) -> album_globals::ImageStatus {
        // Stub: just return a default ImageStatus for now
            ImageStatus { link: image_url.to_string(), exists: false, width: 0, height: 0, filename: String::new() }
    }}

