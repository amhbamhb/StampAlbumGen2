use log::{error, info};
use std::io::Write;
use crate::globals::{SeriesInfo, ImageStatus};
use crate::text_utils::sanitize_filename;
use crate::globals::OSType;


/// Represents the status of an image operation (stub)
pub fn get_colnect_image(images_directory: &str, url: &str, name: &str) -> ImageStatus {
    let clean_name = name.replace(' ', "");
    let file_path = format!("{}/{}.jpg", images_directory, clean_name);
    get_image(url, name, &file_path)
}

/// Stub for get_image, to be implemented
pub fn get_image(_url: &str, _name: &str, _file_path: &str) -> ImageStatus {
    // Use reqwest to fetch the image
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap_or_else(|_| reqwest::blocking::Client::new());
    match client.get(_url).send() {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.bytes() {
                    Ok(bytes) => {
                        match std::fs::write(_file_path, &bytes) {
                            Ok(_) => {
                                ImageStatus {
                                    link: _url.to_string(),
                                    exists: true,
                                    width: 0,
                                    height: 0,
                                }
                            }
                            Err(_) => {
                                ImageStatus {
                                    link: _url.to_string(),
                                    exists: false,
                                    width: 0,
                                    height: 0,
                                }
                            }
                        }
                    }
                    Err(_) => {
                        ImageStatus {
                            link: _url.to_string(),
                            exists: false,
                            width: 0,
                            height: 0,
                        }
                    }
                }
            } else {
                ImageStatus {
                    link: _url.to_string(),
                    exists: false,
                    width: 0,
                    height: 0,
                }
            }
        }
        Err(_) => {
            ImageStatus {
                link: _url.to_string(),
                exists: false,
                width: 0,
                height: 0,
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

    pub fn get_stampworld_image(&self, url: &str, overwrite: bool) -> ImageStatus {
        use std::path::Path;
        let raw_image_name = url.split('/').last().unwrap_or("stampworld.jpg");
        let image_name = sanitize_filename(raw_image_name);
        let file_path = format!("{}/{}", self.images_directory, image_name);
        if !overwrite && Path::new(&file_path).exists() {
            info!("Skipping download, image already exists: {}", file_path);
            return ImageStatus { link: url.to_string(), exists: true, width: 0, height: 0 };
        }
        self.get_image(url, &image_name, &file_path)
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
                info!("Skipping download, image already exists: {}", file_path);
                return ImageStatus { link: url.to_string(), exists: true, width: 0, height: 0 };
            }
            
            info!("Downloading image: {} -> {}", url, file_path);
            
            // Use curl to download the image
            use std::process::Command;
            let curl_cmd = if url.starts_with("https://") {
                vec!["curl", "--ssl-no-revoke", "-L", "-o", &file_path, url]
            } else {
                vec!["curl", "-L", "-o", &file_path, url]
            };
            
            info!("Running curl command: {:?}", curl_cmd);
            
            match Command::new(curl_cmd[0])
                .args(&curl_cmd[1..])
                .status() {
                Ok(status) => {
                    if status.success() {
                        info!("Successfully downloaded: {}", file_path);
                        ImageStatus { link: url.to_string(), exists: true, width: 0, height: 0 }
                    } else {
                        error!("Failed to download {}: curl exit code {:?}", url, status.code());
                        ImageStatus { link: url.to_string(), exists: false, width: 0, height: 0 }
                    }
                }
                Err(e) => {
                    error!("Failed to run curl for {}: {}", url, e);
                    ImageStatus { link: url.to_string(), exists: false, width: 0, height: 0 }
                }
            }
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
            file_separator: crate::globals::FILE_SEPARATOR.to_string(),
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
            month_names: vec!["January".to_string(), "February".to_string(), "March".to_string(), "April".to_string(), "May".to_string(), "June".to_string(), "July".to_string(), "August".to_string(), "September".to_string(), "October".to_string(), "November".to_string(), "December".to_string()],
            valid_formats: Vec::new(),
            valid_emissions: Vec::new(),
            touchstamps_data_file: Vec::new(),
        }
    }
    /// Stub for write_globals helper
    pub fn write_globals<W: Write>(&self, _writer: &mut W) {
    crate::stampworld::write_globals(_writer, &self.album_name);
    }

    /// Remove stub for new_page helper
    // pub fn new_page<W: Write>(&self, _writer: &mut W) {
    //     // TODO: Implement new page writing
    // }
    /// Remove stub for new_year helper
    // pub fn new_year<W: Write>(&self, _writer: &mut W, _year: &str) {
    //     // TODO: Implement new year writing
    // }
    /// Remove stub for new_series helper
    // pub fn new_series<W: Write>(&self, _writer: &mut W, _series: &str, _year: &str, _month: &str, _day: &str, _count: i32, _multi_perf: bool, _desc: &str) {
    //     // TODO: Implement new series writing
    // }
    /// Remove stub for add_stamp helper
    // pub fn add_stamp<W: Write>(&self, _writer: &mut W, _year: &str, _field1: &str, _field2: &str, _field3: &str, _color: String, _multi_perf: bool, _desc: &str, _width: &str, _height: &str, _catalog_num: &str, _field4: &str) {
    //     // TODO: Implement stamp writing
    // }

    /// Stub for get_series_info helper
    pub fn get_series_info(&self, data: &[String], idx: usize, width: &str, height: &str) -> SeriesInfo {
        let mut count = 1;
        let mut multi_perf = false;
        let mut w = width.to_string();
        let mut h = height.to_string();
        if idx >= data.len() { return SeriesInfo { count, multi_perf, width: w, height: h }; }
        let fields: Vec<&str> = data[idx].split(&self.separator).collect();
        if fields.len() < 19 { return SeriesInfo { count, multi_perf, width: w, height: h }; }
        let date = format!("{}{}{}", fields[1], fields[2], fields[3]);
        let series = fields[4];
        let desc = fields[12];
        let mut i = idx;
        let mut done = false;
        if !series.is_empty() {
            while !done && i < data.len() - 1 {
                i += 1;
                let next_fields: Vec<&str> = data[i].split(&self.separator).collect();
                if next_fields.len() < 19 { break; }
                let next_date = format!("{}{}{}", next_fields[1], next_fields[2], next_fields[3]);
                let next_series = next_fields[4];
                let next_desc = next_fields[12];
                let next_format = next_fields[13];
                let next_emission = next_fields[14];
                let next_width = next_fields[10];
                let next_height = next_fields[11];
                if next_series == series && next_date == date {
                    if self.is_selected_format(next_format) && self.is_selected_emission(next_emission) {
                        count += 1;
                        if next_desc != desc && !next_desc.is_empty() {
                            multi_perf = true;
                        }
                        if width == "0" {
                            w = next_width.to_string();
                            h = next_height.to_string();
                        }
                    }
                } else {
                    done = true;
                }
            }
        }
        SeriesInfo { count, multi_perf, width: w, height: h }
    }

    /// Stub for get_catalog_num helper
    pub fn get_catalog_num(&self, field: &str, flag: bool) -> String {
        let mut result = String::new();
        let mut cat_name = self.catalog_name.clone();
        if flag && (self.data_source.eq("colnect") || self.file_source.eq("colnect")) {
            cat_name = "All".to_string();
        }
        if field.contains(&cat_name) {
            for part in field.split(',') {
                if result.is_empty() && part.starts_with(&cat_name) {
                    if let Some(idx) = part.find(':') {
                        result = part[idx+1..].to_string();
                    }
                }
            }
        }
        result
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

    /// SeriesInfo struct for series metadata
    pub fn save_stamp_data<P: AsRef<std::path::Path>>(&self, file_path: P, data: &[String], _source: &str, overwrite_album: bool) {
        use std::fs::{self, File};
        use std::io::{Write, BufWriter};
        use std::os::unix::fs::PermissionsExt;
        let path = file_path.as_ref();

        // If overwrite is false and file exists, skip without prompting
        if !overwrite_album && path.exists() {
            info!("File {} already exists and overwrite is disabled. Skipping save.", path.display());
            return;
        }

        // If overwrite is true and file exists, remove it first
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
                // Set file to read-only
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
    pub fn get_image(&self, url: &str, name: &str, file_path: &str) -> ImageStatus {
        use std::process::Command;
        use std::fs;
        use image::io::Reader as ImageReader;
        use std::path::Path;
        
        if name.is_empty() {
            return ImageStatus { link: String::new(), exists: false, width: 0, height: 0 };
        }
        
        // Check if file already exists
        if Path::new(file_path).exists() {
            info!("Skipping download, image already exists: {}", file_path);
            return ImageStatus { link: url.to_string(), exists: true, width: 0, height: 0 };
        }
        
        println!("[INFO] Downloading image: {} -> {}", name, file_path);
        
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
    ImageStatus { link: url.to_string(), exists, width, height }
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
        self.save_stamp_data(&file_path, &all_stamp_data, "touchstamps", false);
        all_stamp_data
    }
    pub fn fix_color(&self, color: &str) -> String {
        // Stub: just return the color for now
        color.to_string()
    }

    pub fn snooze(&self, millis: u64) {
        std::thread::sleep(std::time::Duration::from_millis(millis));
    }

    pub fn get_touchstamps_image(&self, image_url: &str, _image_id: &str) -> ImageStatus {
        // Stub: just return a default ImageStatus for now
        ImageStatus {
            link: image_url.to_string(),
            exists: false,
            width: 0,
            height: 0,
        }
    }
}
