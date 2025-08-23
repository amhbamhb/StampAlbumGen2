use std::io::Write;
use crate::album_globals;
use crate::album_globals::{SeriesInfo, ImageStatus, MiscAttrs, YearMonthDay};
use std::io::BufRead;

/// Represents the status of an image operation (stub)
pub fn get_colnect_image(images_directory: &str, url: &str, name: &str) -> ImageStatus {
    let clean_name = name.replace(' ', "");
    let file_path = format!("{}/{}.jpg", images_directory, clean_name);
    get_image(url, name, &file_path)
}

/// Stub for get_image, to be implemented
pub fn get_image(_url: &str, _name: &str, _file_path: &str) -> ImageStatus {
    // TODO: Implement actual image download and save logic
    ImageStatus {
        link: String::new(),
        exists: false,
        width: 0,
        height: 0,
    }
}
impl CommandsFileGenerator {
    pub fn create_album<P: AsRef<std::path::Path>>(&mut self, file_path: P, data: &[String]) {
        use std::fs::File;
        use std::io::{BufWriter, Write};
        let path = file_path.as_ref();
        let file = match File::create(path) {
            Ok(f) => f,
            Err(e) => {
                println!("Unable to open output file: {}", e);
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
                let catalog_num = if self.catalog_name.is_empty() {
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
            album_globals::new_series(&mut writer, series, year, month, day, count, multi_perf, fields[12], &mut rows_written, &mut stamps_written, &mut used_width, &mut used_height, &mut max_row_height, &month_names);
                    prev_year = year.to_string();
                    prev_series = series.to_string();
                    prev_date = date.clone();
                } else if series != prev_series || date != prev_date {
                        series_info = Some(self.get_series_info(data, idx, &width, &height));
                        count = series_info.as_ref().map_or(1, |s| s.count);
                        multi_perf = series_info.as_ref().map_or(false, |s| s.multi_perf);
                        width = series_info.as_ref().map_or("0".to_string(), |s| s.width.clone());
                        height = series_info.as_ref().map_or("0".to_string(), |s| s.height.clone());
            album_globals::new_series(&mut writer, series, year, month, day, count, multi_perf, fields[12], &mut rows_written, &mut stamps_written, &mut used_width, &mut used_height, &mut max_row_height, &month_names);
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
    album_globals::write_globals(_writer, &self.album_name);
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
    pub fn save_stamp_data<P: AsRef<std::path::Path>>(&self, file_path: P, data: &[String], _source: &str) {
        use std::fs::{self, File, OpenOptions};
        use std::io::{Write, BufWriter};
        use std::os::unix::fs::PermissionsExt;
        let path = file_path.as_ref();
        let file = File::create(path);
        match file {
            Ok(f) => {
                let mut writer = BufWriter::new(f);
                if let Err(e) = writeln!(writer, "SOURCE:{}", self.data_source) {
                    println!("Unable to write source line: {}", e);
                }
                for line in data {
                    if let Err(e) = writeln!(writer, "{}", line) {
                        println!("Unable to write stamp data: {}", e);
                    }
                }
                // Set file to read-only
                if let Err(e) = writer.flush() {
                    println!("Unable to flush file: {}", e);
                }
                if let Ok(metadata) = fs::metadata(path) {
                    let mut perms = metadata.permissions();
                    perms.set_mode(0o444); // read-only for all
                    if let Err(e) = fs::set_permissions(path, perms) {
                        println!("Unable to set file to read-only: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Unable to save extract file ... continuing: {}", e);
            }
        }
    }
    pub fn get_image(&self, url: &str, name: &str, file_path: &str) -> ImageStatus {
        use std::process::Command;
        use std::fs;
        use image::io::Reader as ImageReader;
        if name.is_empty() {
            return ImageStatus { link: String::new(), exists: false, width: 0, height: 0 };
        }
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
            println!("Error executing curl for imageURL: {}", e);
            std::process::exit(1);
        }
        // Read image dimensions
        let img_file = std::path::Path::new(file_path);
        match ImageReader::open(img_file) {
            Ok(reader) => match reader.decode() {
                Ok(img) => {
                    width = img.width() as i32;
                    height = img.height() as i32;
                }
                Err(e) => {
                    println!("Error decoding image: {}", e);
                    exists = false;
                }
            },
            Err(e) => {
                println!("Error opening image file: {}", e);
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
        use std::io::{self, Read};
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
        let mut results = Vec::new();
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
        for article in document.select(&article_selector) {
            let mut stamp_name = article.select(&Selector::parse("header.round-header").unwrap()).next().map(|e| e.text().collect::<String>()).unwrap_or_default();
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
            let (mut face_value, mut currency) = ("0".to_string(), "0".to_string());
            let mut value_main = String::new();
            let mut value_rest = String::new();
            if let Some(table_elem) = table {
                format = table_elem.select(&Selector::parse("tr:contains(Format:)").unwrap()).next().map(|e| e.text().collect::<String>().replace("Format: ", "")).unwrap_or_default();
                perforation = table_elem.select(&Selector::parse("tr:contains(Perforation:)").unwrap()).next().map(|e| e.text().collect::<String>().replace("Perforation: ", "")).unwrap_or_default();
                emission = table_elem.select(&Selector::parse("tr:contains(Emission:)").unwrap()).next().map(|e| e.text().collect::<String>().replace("Emission: ", "")).unwrap_or_default();
                let offers_text = table_elem.select(&Selector::parse("td[itemprop=\"offers\"]").unwrap()).next().map(|e| e.text().collect::<String>()).unwrap_or_default();
                let mut idx = offers_text.len();
                while idx > 0 && !offers_text.chars().nth(idx - 1).unwrap().is_digit(10) && ![188, 189, 190].contains(&(offers_text.chars().nth(idx - 1).unwrap() as u8)) {
                    idx -= 1;
                }
                value_main = offers_text.get(0..idx).unwrap_or("").to_string();
                value_rest = offers_text.get(idx..).unwrap_or("").trim().to_string();
            }

            let details_section_html = article.select(&Selector::parse("section.ico-toolbar").unwrap())
                .next()
                .and_then(|e| e.select(&Selector::parse("div.btn-group").unwrap()).next())
                .map(|e| e.html())
                .unwrap_or_default();
            let details_start = details_section_html.find("title=\"Details\" href=\"").map(|i| i + 22);
            let details_url = if let Some(start) = details_start {
                let rest = &details_section_html[start..];
                let end = rest.find('"').unwrap_or(rest.len());
                &rest[..end]
            } else {
                ""
            };
            let details = self.get_touchstamps_details(details_url);
            let details_parts: Vec<&str> = details.split(&self.separator).collect();
            if details_parts.len() >= 2 {
                face_value = details_parts[0].to_string();
                currency = details_parts[1].to_string();
            }

            self.sequencer += 1;
            let sequence = format!("{:07}", self.sequencer);
            let sequence = &sequence[sequence.len() - 6..];

            let output = format!(
                "STAMP{sep}{year}{sep}{month}{sep}{day}{sep}{desc}{sep}{seq}{sep}{stamp_name}{sep}{value_main}{sep}{value_rest}{sep}{}{sep}{face_value}{sep}{currency}{sep}{perforation}{sep}{format}{sep}{emission}{sep}{catalog_code}{sep}{image_link}{sep}{image_exists}{sep}{image_width}{sep}{image_height}",
                sep = self.separator,
                year = year,
                month = month,
                day = day,
                desc = desc,
                seq = sequence,
                stamp_name = stamp_name,
                value_main = value_main,
                value_rest = value_rest,
                face_value = face_value,
                currency = currency,
                perforation = perforation,
                format = format,
                emission = emission,
                catalog_code = catalog_code,
                image_link = image_link,
                image_exists = image_exists,
                image_width = image_width,
                image_height = image_height
            );
            results.push(output);
        }
        results
    }

    /// Stub for get_touchstamps_image helper
    pub fn get_touchstamps_image(&self, _image_url: &str, _image_id: &str) -> ImageStatus {
    let file_path = format!("{}/{}.jpg", self.images_directory, _image_id);
    let full_url = format!("touchstamps.com/{}", _image_url);
    self.get_image(&full_url, _image_id, &file_path)
    }

    /// Stub for get_touchstamps_details helper
    pub fn get_touchstamps_details(&self, _details_url: &str) -> String {
        use reqwest::blocking::Client;
        use scraper::{Html, Selector};

    let url = format!("https://touchstamps.com{}", _details_url);
        let mut width = "0".to_string();
        let mut height = "0".to_string();

        let client = Client::new();
        let document = match client.get(&url).timeout(std::time::Duration::from_secs(60)).send() {
            Ok(resp) => match resp.text() {
                Ok(body) => Html::parse_document(&body),
                Err(_) => {
                    eprintln!("Problem retrieving details page");
                    std::process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("Problem retrieving details page: {}", e);
                std::process::exit(1);
            }
        };

        let width_selector = Selector::parse("span[itemprop=\"width\"]").unwrap();
        if let Some(width_elem) = document.select(&width_selector).next() {
            width = width_elem.text().collect::<String>();
        }
        let height_selector = Selector::parse("span[itemprop=\"height\"]").unwrap();
        if let Some(height_elem) = document.select(&height_selector).next() {
            height = height_elem.text().collect::<String>();
        }

        format!("{}{}{}", width, self.separator, height)
    }
    pub fn get_touchstamps_year(&mut self, year_url: &str) -> Vec<String> {
        use reqwest::blocking::Client;
        use scraper::{Html, Selector};

        let client = Client::new();
        let mut results = Vec::new();
        let mut page = 1;
        let mut rows_found = 1;

        while rows_found > 0 {
            let url = format!("{}?page={}", year_url, page);
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

            let table_selector = Selector::parse("table.table.table-striped.table-condensed").unwrap();
            if let Some(table_elem) = document.select(&table_selector).next() {
                let table_children: Vec<_> = table_elem.children().collect();
                if table_children.len() < 2 { break; }
                let rows = scraper::ElementRef::wrap(table_children[1].clone()).unwrap().children().collect::<Vec<_>>();
                rows_found = rows.len();
                for row in rows {
                    let row_elem = scraper::ElementRef::wrap(row).unwrap();
                    let cols: Vec<_> = row_elem.children().collect();
                    if cols.len() < 3 { continue; }
                    let col1 = scraper::ElementRef::wrap(cols[1].clone()).unwrap().text().collect::<String>();
                    let col2 = scraper::ElementRef::wrap(cols[2].clone()).unwrap().text().collect::<String>();
                    let col2_html = scraper::ElementRef::wrap(cols[2].clone()).unwrap().html();
                    let parts: Vec<&str> = col2_html.split('"').collect();
                    let link = parts.get(1).unwrap_or(&"");
                    let date_parts: Vec<&str> = col1.split('/').collect();
                    if date_parts.len() == 3 {
                        let year = date_parts[2];
                        let month = date_parts[0];
                        let day = date_parts[1];
                        let stamps = self.get_touchstamps_stamps(link, year, month, day, &col2);
                        results.extend(stamps);
                    }
                }
            } else {
                break;
            }
            page += 1;
        }
        results
    }

    pub fn collect_touchstamps_data(&mut self, base_url: &str) -> Vec<String> {
        use reqwest::blocking::Client;
        use scraper::{Html, Selector};

        let client = Client::new();
        let mut results = Vec::new();
        let document = match client.get(base_url).timeout(std::time::Duration::from_secs(60)).send() {
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

        let year_selector = Selector::parse("select#year").unwrap();
        if let Some(year_elem) = document.select(&year_selector).next() {
            let children: Vec<_> = year_elem.children().collect();
            let year_texts: Vec<&str> = year_elem.text().collect();
            for (i, child) in children.iter().enumerate().skip(1) {
                if let Some(year_str) = year_texts.get(i) {
                    if let Ok(year) = year_str.trim().parse::<i32>() {
                        if year >= self.start_year && year <= self.end_year {
                            let year_url = format!("{}/{}", base_url, year);
                            let year_data = self.get_touchstamps_year(&year_url);
                            results.extend(year_data);
                        }
                    }
                }
            }
        }

        results
    }

    pub fn collect_stampworld_data(&mut self, base_url: &str) -> Vec<String> {
        use reqwest::blocking::Client;
        use scraper::{Html, Selector};
        use std::thread::sleep;
        use std::time::Duration;

        let client = Client::new();
        let mut results = Vec::new();
        let mut finished = false;
        let mut page = 1;

        while !finished {
            let url = format!("{}{}", base_url, page);
            let document = match client.get(&url).timeout(Duration::from_secs(60)).send() {
                Ok(resp) => match resp.text() {
                    Ok(body) => Html::parse_document(&body),
                    Err(_) => {
                        eprintln!("Problem retrieving web page");
                        std::process::exit(1);
                    }
                },
                Err(e) => {
                    if page == 1 {
                        println!("Please check that you have entered the correct site country name");
                    } else {
                        println!("Problem retrieving web page: {}", e);
                    }
                    std::process::exit(1);
                }
            };

            page += 1;

            // Check if finished
            let top_bar_selector = Selector::parse("#topBlueBarGroupsCatalogLabel").unwrap();
            let top_bar = document.select(&top_bar_selector).next();
            let mut finished_page = false;
            if let Some(bar) = top_bar {
                let text = bar.text().collect::<String>();
                if !text.is_empty() && text.chars().any(|c| c.is_digit(10)) {
                    let page_info = text.split("page ").nth(1).unwrap_or("");
                    let page_info = page_info.split(')').next().unwrap_or("");
                    let parts: Vec<&str> = page_info.split('/').collect();
                    if parts.len() == 2 && parts[1] == parts[0] {
                        finished_page = true;
                    }
                } else {
                    finished_page = true;
                }
            } else {
                finished_page = true;
            }

            // Select all content tables
            let table_selector = Selector::parse("div.container-fluid.content_table").unwrap();
            for table in document.select(&table_selector) {
                let children: Vec<_> = table.children().collect();
                if children.len() < 3 { continue; }
                let year_elem = scraper::ElementRef::wrap(children[0].clone()).unwrap();
                let month_elem = scraper::ElementRef::wrap(children[1].clone()).unwrap();
                let stamps_elem = scraper::ElementRef::wrap(children[2].clone()).unwrap();

                let year_text = year_elem.select(&Selector::parse("a[href]").unwrap()).next().map(|e| e.text().collect::<String>()).unwrap_or_default();
                let mut year = year_text.get(0..4).unwrap_or("").to_string();
                let year_num = year.parse::<i32>().unwrap_or(0);
                if year_num < self.start_year || year_num > self.end_year { continue; }

                let mut month = String::new();
                let mut day = String::new();
                let month_day_text = year_elem.select(&Selector::parse("p").unwrap()).next().map(|e| e.text().collect::<String>()).unwrap_or_default();
                if month_day_text.contains('.') {
                    let parts: Vec<&str> = month_day_text.split('.').collect();
                    day = parts.get(0).unwrap_or(&"").to_string();
                    let parts: Vec<&str> = month_day_text.split(' ').collect();
                    month = self.month_number(parts.get(1).unwrap_or(&""));
                } else {
                    let parts: Vec<&str> = month_day_text.split(' ').collect();
                    month = self.month_number(parts.get(0).unwrap_or(&""));
                }

                let perforation = year_elem.select(&Selector::parse("p").unwrap()).next().map(|e| {
                    let txt = e.text().collect::<String>();
                    if txt.contains("Perforation:") {
                        txt.split("Perforation:").nth(1).unwrap_or("").trim().to_string()
                    } else {
                        String::new()
                    }
                }).unwrap_or_default();

                let stamp_selector = Selector::parse("tr.stamp_tr ").unwrap();
                for stamp_row in stamps_elem.select(&stamp_selector) {
                    let tds: Vec<_> = stamp_row.select(&Selector::parse("td").unwrap()).collect();
                    if tds.len() < 6 { continue; }
                    let stamp_id = tds[0].text().collect::<String>();
                    let image_status = self.get_stampworld_image(&stamp_id);
                    let mut size_text = tds[1].text().collect::<String>();
                    let mut size_idx = size_text.len();
                    while size_idx > 0 && !size_text.chars().nth(size_idx - 1).unwrap().is_digit(10) &&
                        ![188, 189, 190].contains(&(size_text.chars().nth(size_idx - 1).unwrap() as u8)) {
                        size_idx -= 1;
                    }
                    let size_main = size_text.get(0..size_idx).unwrap_or("").to_string();
                    let size_rest = size_text.get(size_idx..).unwrap_or("").trim().to_string();
                    let field_5 = tds[5].text().collect::<String>();
                    let field_3 = tds[3].text().collect::<String>();
                    let field_17 = "0".to_string();
                    let field_18 = "0".to_string();
                    let image_link = image_status.link.clone();
                    let image_exists = image_status.exists;
                    let image_width = image_status.width.to_string();
                    let image_height = image_status.height.to_string();
                    let catalog_code = format!("Sw:{}", stamp_id);

                    self.sequencer += 1;
                    let sequence = format!("{:07}", self.sequencer);
                    let sequence = &sequence[sequence.len() - 6..];

                    let output = format!(
                        "STAMP{sep}{year}{sep}{month}{sep}{day}{sep}{stamp_id}{sep}{seq}{sep}{field_5}{sep}{size_main}{sep}{size_rest}{sep}{field_3}{sep}{field_17}{sep}{field_18}{sep}{perforation}{sep}{}{sep}{}{sep}{catalog_code}{sep}{image_link}{sep}{image_exists}{sep}{image_width}{sep}{image_height}",
                        sep = self.separator,
                        year = year,
                        month = month,
                        day = day,
                        stamp_id = stamp_id,
                        seq = sequence,
                        field_5 = field_5,
                        size_main = size_main,
                        size_rest = size_rest,
                        field_3 = field_3,
                        field_17 = field_17,
                        field_18 = field_18,
                        perforation = perforation,
                        catalog_code = catalog_code,
                        image_link = image_link,
                        image_exists = image_exists,
                        image_width = image_width,
                        image_height = image_height
                    );
                    results.push(output);
                }
            }

            if finished_page {
                finished = true;
            }
        }

        results
    }

    /// Stub for month_number helper
    pub fn month_number(&self, _month: &str) -> String {
    match _month.trim().to_uppercase().as_str() {
            "JANUARY" => "01".to_string(),
            "FEBRUARY" => "02".to_string(),
            "MARCH" => "03".to_string(),
            "APRIL" => "04".to_string(),
            "MAY" => "05".to_string(),
            "JUNE" => "06".to_string(),
            "JULY" => "07".to_string(),
            "AUGUST" => "08".to_string(),
            "SEPTEMBER" => "09".to_string(),
            "OCTOBER" => "10".to_string(),
            "NOVEMBER" => "11".to_string(),
            "DECEMBER" => "12".to_string(),
            _ => "00".to_string(),
        }
    }

    /// Stub for get_stampworld_image helper
    pub fn get_stampworld_image(&self, _stamp_id: &str) -> ImageStatus {
        let file_path = format!("{}/{}.jpg", self.images_directory, _stamp_id);
        let image_url = format!(
            "https://swmedia-4cd6.kxcdn.com/media/catalogue/{}/Postage-stamps/{}-s.jpg",
            self.site_name, _stamp_id
        );
        self.get_image(&image_url, _stamp_id, &file_path)
    }
    /// Extract catalog codes from an HTML element (stub)
    pub fn get_colnect_catalog_codes(&self, _element: &scraper::ElementRef) -> String {
    let html = _element.html();
        let mut found_code = false;
        let mut code = String::new();
        let mut all_code = String::new();

        // Try to extract the code from the item header
        if let Some(header_elem) = _element.select(&scraper::Selector::parse("h2.item_header").unwrap()).next() {
            let mut header_html = header_elem.html();
            header_html = header_html.replace("<a href=\"/en/stamps/stamp/", "");
            let parts: Vec<&str> = header_html.split('-').collect();
            if !parts.is_empty() {
                all_code = parts[0].to_string();
            }
        }

        // Search for catalog codes in the HTML lines
        for line in html.lines() {
            if found_code {
                // The code is 3 lines after the match
                code = line.trim().replace("<strong>", "").replace("</strong>", "").replace(' ', "");
                break;
            }
            if line.contains("Catalog codes:") {
                found_code = true;
                // Skip next 3 lines
                for _ in 0..3 {
                    // Advance iterator
                    let _ = html.lines().next();
                }
            }
        }

        if !all_code.is_empty() {
            if !code.is_empty() {
                code = format!("{},All:{}", code, all_code);
            } else {
                code = format!("All:{}", all_code);
            }
        }
        code
    }

    /// Extract item value from an HTML element using a selector (stub)
    pub fn get_colnect_item_value(&self, _element: &scraper::ElementRef, _selector: &str, default: &str) -> String {
        let selector = match scraper::Selector::parse(_selector) {
            Ok(sel) => sel,
            Err(_) => return default.to_string(),
        };
        if let Some(found) = _element.select(&selector).next() {
            found.text().collect::<String>()
        } else {
            default.to_string()
        }
    }

    /// Overload for get_colnect_item_value without default
    pub fn get_colnect_item_value_simple(&self, _element: &scraper::ElementRef, _selector: &str) -> String {
    self.get_colnect_item_value(_element, _selector, "")
    }

    /// Fix color string (stub)
    pub fn fix_color(&self, color: &str) -> String {
        let mut c = color.to_string();
        let replacements = [
            ("'.'", ""),
            ("Dk", "Dark"), ("Grn", "Green"), ("grn", "green"),
            ("Brn", "Grown"), ("brn", "brown"),
            ("Org", "Orange"), ("org", "orange"),
            ("Blk", "Black"), ("blk", "black"),
            ("Multi", "Multicolor"), ("multi", "multicolor"),
            ("Car", "Carmine"), ("CarCarmine", "Carmine"),
            ("car", "carmine"), ("carcarmine", "carmine"),
            ("carminemine", "carmine"), ("Carminemine", "Carmine"),
            ("Ultra", "Ultramarine"), ("Ultramarinemarine", "Ultramarine"),
            ("ultra", "ultramarine"), ("ultramarinemarine", "ultramarine"),
            ("Pur", "Purple"), ("Purpleple", "Purple"),
            ("pur", "purple"), ("purpleple", "Purple"),
            ("Vio", "Violet"), ("Violetlet", "Violet"),
            ("vio", "violet"), ("violetlet", "violet"),
            ("Yel", "Yellow"), ("Yellowlow", "Yellow"),
            ("yel", "yellow"), ("yellowlow", "yellow"),
            ("Brt", "Bright"), ("brt", "bright"),
            ("Dp", "Deep"), ("dp", "deep"),
            ("Br ", "Bright "), ("br ", "bright "),
            ("Lt", "Light"), (" lt", " light"),
            ("Prus ", "Prussian "), ("prus ", "prussian "),
        ];
        for (from, to) in replacements.iter() {
            c = c.replace(from, to);
        }
        c
    }

    /// Extract miscellaneous attributes from HTML (stub)
    pub fn get_colnect_misc_attrs(&self, _html: &str) -> MiscAttrs {
        let mut width = "0".to_string();
        let mut height = "0".to_string();
        let mut face_value = String::new();
        let mut currency = String::new();
        let mut month = "00".to_string();
        let mut day = "00".to_string();
        let mut remaining = 3;
        let mut idx = 0;
    let lines: Vec<&str> = _html.lines().collect();
        while remaining > 0 && idx < lines.len() {
            let line = lines[idx];
            if line.contains("Face value:") {
                idx += 3;
                if idx < lines.len() {
                    let value_line = lines[idx].trim();
                    let parts_space: Vec<&str> = value_line.split(' ').collect();
                    if !parts_space.is_empty() {
                        face_value = parts_space[0].to_string();
                    }
                    let parts_quote: Vec<&str> = value_line.split('"').collect();
                    if parts_quote.len() > 3 {
                        currency = parts_quote[3].to_string();
                    }
                }
                remaining -= 1;
            } else if line.contains("Size:") {
                idx += 3;
                if idx < lines.len() {
                    let size_line = lines[idx].replace(' ', "");
                    let parts_x: Vec<&str> = size_line.split('x').collect();
                    if parts_x.len() > 1 {
                        let mut w = parts_x[0].to_string();
                        let mut h = parts_x[1].to_string();
                        while !w.chars().last().map(|c| c.is_digit(10)).unwrap_or(false) && !w.is_empty() {
                            w.pop();
                        }
                        while !h.chars().last().map(|c| c.is_digit(10)).unwrap_or(false) && !h.is_empty() {
                            h.pop();
                        }
                        width = w;
                        height = h;
                    }
                }
                remaining -= 1;
            } else if line.contains("/en/stamps/list/country/") && line.contains("/year/") {
                month = "00".to_string();
                day = "00".to_string();
                let parts_gt: Vec<&str> = line.split('>').collect();
                if parts_gt.len() > 2 {
                    let mut date_str = parts_gt[2].trim();
                    while date_str.starts_with('-') {
                        date_str = &date_str[1..];
                    }
                    let parts_dash: Vec<&str> = date_str.split('-').collect();
                    if !parts_dash.is_empty() {
                        month = parts_dash[0].to_string();
                        if parts_dash.len() > 1 {
                            day = parts_dash[1].to_string();
                        }
                    }
                }
                if month.len() < 2 {
                    month = format!("0{}", month);
                }
                if day.len() < 2 {
                    day = format!("0{}", day);
                }
                remaining -= 1;
            }
            idx += 1;
        }
        MiscAttrs {
            face_value,
            currency,
            width,
            height,
            month,
            day,
        }
    }
}

/// Extracts year, month, and day from a date string in formats like "YYYY/MM/DD", "MM-DD-YYYY", etc.
///
/// # Arguments
/// * `date` - A string containing a date, separated by '/' or '-'
///
/// # Returns
/// * `Result<YearMonthDay, &'static str>` - The parsed date components or an error message
pub fn extract_year_month_day(date: &str) -> Result<YearMonthDay, &'static str> {
    // Default values
    let mut year = "0000".to_string();
    let mut month = "00".to_string();
    let mut day = "00".to_string();

    // Split by either '/' or '-'
    let components: Vec<&str> = if date.contains('/') {
        date.split('/').collect()
    } else if date.contains('-') {
        date.split('-').collect()
    } else {
        vec![date]
    };

    match components.len() {
        1 => {
            // Only year provided
            year = components[0].to_string();
        }
        2 => {
            // Year and month
            if components[0].len() == 4 {
                year = components[0].to_string();
                month = components[1].to_string();
            } else {
                year = components[1].to_string();
                month = components[0].to_string();
            }
        }
        3 => {
            // Year, month, and day
            if components[0].len() == 4 {
                // Format: YYYY/MM/DD
                year = components[0].to_string();
                month = components[1].to_string();
                day = components[2].to_string();
            } else {
                // Format: MM/DD/YYYY
                year = components[2].to_string();
                month = components[0].to_string();
                day = components[1].to_string();
            }
        }
        _ => return Err("Invalid date format: too many components"),
    }

    Ok(YearMonthDay { year, month, day })
}
// CommandsFileGenerator struct and related types
use rusttype::Font;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OSType {
    Windows,
    MacOS,
    Linux,
    Unknown,
}

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
    // pub affinetransform: ... // Placeholder for future implementation
    // pub frc: ... // Placeholder for future implementation
    pub font: Option<Font<'static>>,
    pub month_names: Vec<String>,
    pub valid_formats: Vec<String>,
    pub valid_emissions: Vec<String>,
}

impl CommandsFileGenerator {
    /// Escapes double quotes in a string by prefixing them with a backslash
    pub fn escape_quotes(&self, input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        for c in input.chars() {
            if c == '"' {
                result.push('\\');
            }
            result.push(c);
        }
        result
    }
    pub fn get_data_from_export<P: AsRef<std::path::Path>>(&mut self, file_path: P) -> Vec<String> {
        use csv::ReaderBuilder;
        use std::fs::File;
        let file = match File::open(&file_path) {
            Ok(f) => f,
            Err(_) => {
                println!("Unable to open input file");
                std::process::exit(1);
            }
        };
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);
        let mut results = Vec::new();
        for result in rdr.records() {
            let record = match result {
                Ok(rec) => rec,
                Err(e) => {
                    eprintln!("Error reading CSV record: {}", e);
                    continue;
                }
            };
            if record.len() <= 32 {
                continue;
            }
            let date_str = &record[4];
            if date_str.parse::<f64>().is_err() {
                continue;
            }
            let ymd = extract_year_month_day(date_str).unwrap_or(YearMonthDay { year: "0000".to_string(), month: "00".to_string(), day: "00".to_string() });
            let mut id = record[0].to_string();
            if id.contains('"') {
                id = self.escape_quotes(&id);
            }
            let field_20 = record[20].replace("<i>", "").replace("</i>", "");
            let field_11 = record[11].replace("<i>", "").replace("</i>", "");
            let field_3 = record[3].replace(" ", "");
            let field_32 = &record[32];
            let categories: Vec<&str> = field_32.split('/').collect();
            let last_category = categories.last().unwrap_or(&"");
            let combined_field = format!("{},All:{}", field_3, last_category);
            self.sequencer += 1;
            let sequence = format!("{:07}", self.sequencer);
            let sequence = &sequence[sequence.len() - 6..];
            let output = format!(
                "STAMP{sep}{year}{sep}{month}{sep}{day}{sep}{f2}{sep}{seq}{sep}{id}{sep}{f20}{sep}{f19}{sep}{f27}{sep}{f6}{sep}{f7}{sep}{f11}{sep}{f9}{sep}{f10}{sep}{comb}{sep}{f32}{sep}{bool}{sep}{byte1}{sep}{byte2}",
                sep = self.separator,
                year = ymd.year,
                month = ymd.month,
                day = ymd.day,
                f2 = &record[2],
                seq = sequence,
                id = id,
                f20 = field_20,
                f19 = &record[19],
                f27 = &record[27],
                f6 = &record[6],
                f7 = &record[7],
                f11 = field_11,
                f9 = &record[9],
                f10 = &record[10],
                comb = combined_field,
                f32 = field_32,
                bool = true,
                byte1 = 0u8,
                byte2 = 0u8
            );
            results.push(output);
        }
        results
    }
    pub fn get_data_from_file<P: AsRef<std::path::Path>>(&self, path: P) -> Vec<String> {
        let mut lines = Vec::new();
        match std::fs::File::open(&path) {
            Ok(file) => {
                let reader = std::io::BufReader::new(file);
                for line in reader.lines() {
                    match line {
                        Ok(l) => lines.push(l),
                        Err(_) => {
                            println!("Unable to retrieve data from extract file");
                            std::process::exit(1);
                        }
                    }
                }
            }
            Err(_) => {
                println!("Unable to retrieve data from extract file");
                std::process::exit(1);
            }
        }
        lines
    }
    pub fn snooze(&self, millis: u64) {
        use std::thread;
        use std::time::Duration;
    thread::sleep(Duration::from_millis(millis));
    }
    pub fn get_file_source<P: AsRef<std::path::Path>>(&self, path: P) -> String {
        let mut source = String::new();
        match std::fs::File::open(&path) {
            Ok(file) => {
                let mut reader = std::io::BufReader::new(file);
                let mut first_line = String::new();
                if let Ok(_) = reader.read_line(&mut first_line) {
                    if first_line.starts_with("SOURCE:") {
                        if let Some(idx) = first_line.find(':') {
                            source = first_line[idx+1..].trim().to_string();
                        }
                    }
                }
            }
            Err(_) => {
                println!("Unable to identify source of extract file");
                std::process::exit(1);
            }
        }
        source
    }
    pub fn new() -> Self {
        Self {
            detected_os: None,
            file_separator: std::path::MAIN_SEPARATOR.to_string(),
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
            separator: String::new(),
            rows_written: 0,
            stamps_written: 0,
            used_width: 0,
            used_height: 0,
            max_row_height: 0,
            sequencer: 0,
            start_year: 0,
            end_year: 0,
            formats: Vec::new(),
            emissions: Vec::new(),
            currencies: Vec::new(),
            font: None,
            month_names: Vec::new(),
            valid_formats: Vec::new(),
            valid_emissions: Vec::new(),
        }
    }

    pub fn check_options(&mut self, args: &[String], start_idx: usize) -> bool {
        let mut valid = true;
        if args.len() > start_idx {
            for arg in &args[start_idx..] {
                let lower = arg.to_lowercase();
                if lower.starts_with("catalog:") {
                    if !self.data_source.eq_ignore_ascii_case("colnect") && !self.file_source.eq_ignore_ascii_case("colnect") {
                        println!("Catalog option only valid for colnect");
                        valid = false;
                    } else {
                        let parts: Vec<&str> = arg.split(':').collect();
                        if parts.len() < 2 {
                            println!("Invalid catalog option {}\nMust be of the form 'catalog:catalog id'", arg);
                            valid = false;
                        } else if !matches!(parts[1].to_lowercase().as_str(), "mi" | "sg" | "sn" | "yt") {
                            println!("Invalid catalog name {}", parts[1]);
                            println!("Valid catalog ids are Mi Sg Sn and Yt");
                            valid = false;
                        } else {
                            let mut cat = parts[1].to_string();
                            cat = format!("{}:", cat);
                            cat = cat[..1].to_uppercase() + &cat[1..].to_lowercase();
                            self.catalog_name = cat;
                        }
                    }
                } else if lower.starts_with("format:") {
                    if !self.data_source.eq_ignore_ascii_case("colnect") && !self.file_source.eq_ignore_ascii_case("colnect") && !self.data_source.eq_ignore_ascii_case("touchstamps") && !self.file_source.eq_ignore_ascii_case("touchstamps") {
                        println!("Format option only valid for colnect and touchstamps");
                        valid = false;
                    } else {
                        let parts: Vec<&str> = arg.split(':').collect();
                        if parts.len() < 2 {
                            println!("Invalid format option {}\nMust be of the form 'format:format id'", arg);
                            valid = false;
                        } else if self.is_valid_format(parts[1]) {
                            self.formats.push(parts[1].to_string());
                        } else {
                            println!("Invalid format {}", parts[1]);
                            print!("Valid formats are ");
                            for f in &self.valid_formats {
                                print!("'{}' ", f);
                            }
                            println!();
                            valid = false;
                        }
                    }
                } else if lower.starts_with("emission:") {
                    if !self.data_source.eq_ignore_ascii_case("colnect") && !self.file_source.eq_ignore_ascii_case("colnect") && !self.data_source.eq_ignore_ascii_case("touchstamps") && !self.file_source.eq_ignore_ascii_case("touchstamps") {
                        println!("Emission option only valid for colnect and touchstamps");
                        valid = false;
                    } else {
                        let parts: Vec<&str> = arg.split(':').collect();
                        if parts.len() < 2 {
                            println!("Invalid emission option {}\nMust be of the form 'emission:emission id'", arg);
                            valid = false;
                        } else if self.is_valid_emission(parts[1]) {
                            self.emissions.push(parts[1].to_string());
                        } else {
                            println!("Invalid emission {}", parts[1]);
                            print!("Valid emissions are ");
                            for e in &self.valid_emissions {
                                print!("'{}' ", e);
                            }
                            println!();
                            valid = false;
                        }
                    }
                } else if lower.starts_with("currency:") {
                    let parts: Vec<&str> = arg.split(':').collect();
                    if parts.len() < 4 {
                        println!("Invalid currency option {}\nMust be of the form 'currency:currency name:currency symbol:[pre|post]'", arg);
                        valid = false;
                    } else if !matches!(parts[3].to_lowercase().as_str(), "pre" | "post") {
                        println!("Invalid currency position {}\nCurrency position (fourth element) must be pre or post", parts[3]);
                        valid = false;
                    } else {
                        self.currencies.push(arg.replace('"', ""));
                    }
                } else if lower.starts_with("startyear:") {
                    let parts: Vec<&str> = arg.split(':').collect();
                    if parts.len() < 2 {
                        println!("Invalid start year option {}\nMust be of the form 'startYear:yyyy'", arg);
                        valid = false;
                    } else if self.is_valid_year(parts[1]) {
                        self.start_year = parts[1].parse().unwrap_or(0);
                    } else {
                        println!("Invalid start year {}", parts[1]);
                        println!("start year must be a positive number");
                        valid = false;
                    }
                } else if lower.starts_with("endyear:") {
                    let parts: Vec<&str> = arg.split(':').collect();
                    if parts.len() < 2 {
                        println!("Invalid end year option {}\nMust be of the form 'endYear:yyyy'", arg);
                        valid = false;
                    } else if self.is_valid_year(parts[1]) {
                        self.end_year = parts[1].parse().unwrap_or(0);
                    } else {
                        println!("Invalid end year {}", parts[1]);
                        println!("end year must be a positive number");
                        valid = false;
                    }
                } else if arg.eq_ignore_ascii_case("noImages") {
                    self.no_images = true;
                } else if arg.eq_ignore_ascii_case("fixSizes") {
                    self.fix_sizes = true;
                } else if arg.eq_ignore_ascii_case("noSSLCheck") {
                    self.no_ssl_check = true;
                } else {
                    println!("Invalid argument {}", arg);
                    valid = false;
                }
            }
        }
        if self.start_year > 0 && self.end_year > 0 && self.start_year > self.end_year {
            println!("startYear may not be greater than endYear");
            valid = false;
        }
        valid
    }

    pub fn is_valid_format(&self, format: &str) -> bool {
        self.valid_formats.iter().any(|f| f.eq_ignore_ascii_case(format))
    }

    pub fn is_valid_emission(&self, emission: &str) -> bool {
        self.valid_emissions.iter().any(|e| e.eq_ignore_ascii_case(emission))
    }

    pub fn is_valid_year(&self, year: &str) -> bool {
        match year.parse::<f64>() {
            Ok(val) => val >= 0.0,
            Err(_) => false,
        }
    }

    pub fn get_operating_system_type() -> OSType {
        let os_name = std::env::var("OSTYPE")
            .or_else(|_| std::env::var("os.name"))
            .unwrap_or_else(|_| "generic".to_string())
            .to_lowercase();

        if os_name.contains("mac") || os_name.contains("darwin") {
            OSType::MacOS
        } else if os_name.contains("win") {
            OSType::Windows
        } else if os_name.contains("nux") {
            OSType::Linux
        } else {
            OSType::Unknown
        }
    }
}
