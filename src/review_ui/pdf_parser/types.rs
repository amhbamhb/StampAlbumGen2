use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PdfStampRecord {
    pub catalog_number: String,
    pub stamp_type: String,
    pub description: String,
    pub color: String,
    pub denomination: String,
    pub year: String,
    pub series: String,
    pub designer: String,
    pub engraver: String,
    pub perforation: String,
    pub watermark: String,
    pub currency: String,
    pub stamps_issued: String,
    pub mint_price: String,
    pub unused_price: String,
    pub used_price: String,
    pub fdc_price: String,
    pub local_image_path: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ImportSummary {
    pub total_found: usize,
    pub imported: usize,
    pub skipped: usize,
}
