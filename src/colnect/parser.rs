use regex::Regex;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Clean up the Name field by extracting perforation suffix like "- P(14 x 15)"
/// and reformatting it on a new line without the dash and brackets
fn clean_name(name: &str) -> String {
    // Match pattern like "- P(14 x 15)" or "- P(14½ x 14)" at end of string
    let re = Regex::new(r"\s*-\s*P\(([^)]+)\)\s*$").unwrap();
    if let Some(caps) = re.captures(name) {
        let base_name = re.replace(name, "").trim().to_string();
        let perf_info = &caps[1];
        format!("{} \\n P {}", base_name, perf_info)
    } else {
        name.to_string()
    }
}

/// Clean up the Perforation field by removing HTML tags and type prefixes
/// Example: "comb<i> 14 x 14½</i>" -> "14 x 14½"
fn clean_perforation(perf: &str) -> String {
    // Remove HTML tags
    let re_tags = Regex::new(r"<[^>]+>").unwrap();
    let no_tags = re_tags.replace_all(perf, "");

    // Remove "comb" or "line" prefix (case insensitive)
    let re_prefix = Regex::new(r"(?i)^(comb|line)\s*").unwrap();
    let cleaned = re_prefix.replace(&no_tags, "");

    cleaned.trim().to_string()
}

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
    #[serde(rename = "Ord Mi")]
    pub ord_mi: Option<String>,
    #[serde(rename = "Ord Sn")]
    pub ord_sn: Option<String>,
    #[serde(rename = "Ord Yt")]
    pub ord_yt: Option<String>,
    #[serde(rename = "Ord Sg")]
    pub ord_sg: Option<String>,
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
    #[serde(rename = "Parent Item")]
    pub parent_item: Option<String>,
    #[serde(rename = "Variant")]
    pub variant: Option<String>,
    #[serde(rename = "Overprint Variant")]
    pub overprint_variant: Option<String>,
    #[serde(rename = "Score")]
    pub score: Option<String>,
    #[serde(rename = "Accuracy")]
    pub accuracy: Option<String>,
    #[serde(rename = "List")]
    pub list: Option<String>,
    #[serde(rename = "Quantity")]
    pub quantity: Option<String>,
    #[serde(rename = "Condition")]
    pub condition: Option<String>,
    #[serde(rename = "Public Note")]
    pub public_note: Option<String>,
    #[serde(rename = "Private Note")]
    pub private_note: Option<String>,
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
    for (i, result) in rdr.deserialize().enumerate() {
        match result {
            Ok(record) => {
                let mut record: ColnectStamp = record;
                // Clean up the name field (extract perforation suffix)
                record.name = clean_name(&record.name);
                // Clean up the perforation field (remove HTML tags and prefixes)
                if let Some(ref perf) = record.perforation {
                    record.perforation = Some(clean_perforation(perf));
                }
                stamps.push(record);
            }
            Err(e) => {
                eprintln!("Warning: Failed to parse CSV record {}: {}", i + 1, e);
            }
        }
    }
    eprintln!("Successfully parsed {} stamps from CSV", stamps.len());
    Ok(stamps)
}
