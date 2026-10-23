use crate::review_ui::pdf_parser::types::PdfStampRecord;
use std::path::Path;
use regex::Regex;
use pdf_extract::extract_text;

#[allow(dead_code)]
pub fn parse_stampworld_pdf<P: AsRef<Path>>(path: P) -> Result<Vec<PdfStampRecord>, String> {
    // extract_text takes a path directly or reader
    let text = extract_text(path.as_ref())
        .map_err(|e| format!("Failed to extract text: {}", e))?;
    
    let mut records = Vec::new();
    
    let re = Regex::new(r"(?x)
        ^(?P<no>\d+)\s+
        (?P<type>\w+)\s+
        (?P<denom>[\d\w]+)\s+
        (?P<color>[\w\s]+?)\s{2,}
        (?P<desc>.+?)\s{2,}
        (?P<issued>\([\d\.,]+\))?\s*
        (?P<mint>[\d\.-]+)\s+
        (?P<unused>[\d\.-]+)\s+
        (?P<used>[\d\.-]+)\s+
        (?P<fdc>[\d\.-]+)
    ").unwrap();
    
    for line in text.lines() {
        let line = line.trim();
        if let Some(caps) = re.captures(line) {
            records.push(PdfStampRecord {
                catalog_number: caps["no"].to_string(),
                stamp_type: caps["type"].to_string(),
                description: caps["desc"].to_string(),
                color: caps["color"].to_string(),
                denomination: caps["denom"].to_string(),
                year: "".to_string(),
                series: "".to_string(),
                designer: "".to_string(),
                engraver: "".to_string(),
                perforation: "".to_string(),
                watermark: "".to_string(),
                currency: "USD".to_string(),
                stamps_issued: caps.name("issued").map(|m| m.as_str().to_string()).unwrap_or_default(),
                mint_price: caps["mint"].to_string(),
                unused_price: caps["unused"].to_string(),
                used_price: caps["used"].to_string(),
                fdc_price: caps["fdc"].to_string(),
                local_image_path: None,
            });
        }
    }
    
    Ok(records)
}
