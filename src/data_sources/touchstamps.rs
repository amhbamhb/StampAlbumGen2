use crate::album_globals::ImageStatus;
use reqwest::blocking::Client;
use scraper::{Html, Selector};

pub fn get_touchstamps_stamps(
    link: &str,
    year: &str,
    month: &str,
    day: &str,
    desc: &str,
    images_directory: &str,
    get_touchstamps_image: &dyn Fn(&str, &str) -> ImageStatus,
    get_image: &dyn Fn(&str, &str, &str) -> ImageStatus,
    album_name: &str,
    save_stamp_data: &dyn Fn(&str, &[String], &str),
) -> Vec<String> {
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

        let image_status = get_touchstamps_image(&image_url, &image_id);
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
            let image_status = get_image(&image_link, &stamp_name, &format!("{}/{}", images_directory, image_id));
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
    let file_path = format!("{}_data.txt", album_name);
    save_stamp_data(&file_path, &all_stamp_data, "touchstamps");
    all_stamp_data
}
// Touchstamps data source handler
pub fn collect_touchstamps_data() {
    // TODO: Move logic from CommandsFileGenerator
}
