use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct ColnectStamp {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Series")]
    pub series: String,
    #[serde(rename = "Catalog Codes")]
    pub catalog_codes: String,
    #[serde(rename = "Issued on")]
    pub issued_on: String,
    #[serde(rename = "Expiry date")]
    pub expiry_date: Option<String>,
    #[serde(rename = "Width")]
    pub width: Option<String>,
    #[serde(rename = "Height")]
    pub height: Option<String>,
    #[serde(rename = "Watermark")]
    pub watermark: Option<String>,
    #[serde(rename = "Format")]
    pub format: Option<String>,
    #[serde(rename = "Emission")]
    pub emission: Option<String>,
    #[serde(rename = "Perforation")]
    pub perforation: Option<String>,
    #[serde(rename = "Printing")]
    pub printing: Option<String>,
    #[serde(rename = "Paper")]
    pub paper: Option<String>,
    #[serde(rename = "Gum")]
    pub gum: Option<String>,
    #[serde(rename = "Currency")]
    pub currency: Option<String>,
    #[serde(rename = "FaceValue")]
    pub face_value: Option<String>,
    #[serde(rename = "Print run")]
    pub print_run: Option<String>,
    #[serde(rename = "Colors")]
    pub colors: Option<String>,
    #[serde(rename = "Designers")]
    pub designers: Option<String>,
    #[serde(rename = "Printers")]
    pub printers: Option<String>,
    #[serde(rename = "Themes")]
    pub themes: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Link")]
    pub link: String,
}

pub fn parse_csv<P: AsRef<Path>>(path: P) -> Result<Vec<ColnectStamp>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut csv_content = String::new();
    let mut header_found = false;

    // Read lines until we find the header
    while reader.read_line(&mut buffer)? > 0 {
        if !header_found {
            // The header line starts with "Name,Country"
            if buffer.trim_start().starts_with("Name,Country") {
                header_found = true;
                csv_content.push_str(&buffer);
            }
        } else {
            // Check for footer
            if buffer.starts_with("\"END of Colnect list") {
                break;
            }
            // Skip empty lines
            if buffer.trim().is_empty() {
                buffer.clear();
                continue;
            }
            csv_content.push_str(&buffer);
        }
        buffer.clear();
    }

    if !header_found {
        return Err("Could not find CSV header starting with 'Name,Country'".into());
    }

    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(csv_content.as_bytes());

    let mut stamps = Vec::new();
    for result in rdr.deserialize() {
        let record: ColnectStamp = result?;
        stamps.push(record);
    }
    Ok(stamps)
}
