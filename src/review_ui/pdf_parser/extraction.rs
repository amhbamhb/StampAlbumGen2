use super::types::PdfStampRecord;
use oxidize_pdf::operations::{ExtractImagesOptions, extract_images_from_pdf};
use pdf_extract::{Document, MediaBox, OutputDev, OutputError, Transform};
use std::path::Path;

use regex::Regex;
use scraper::{Html, Selector};

#[derive(Default)]
struct LayoutText {
    glyphs: Vec<(f64, f64, f64, f64, String)>,
    word: Option<(f64, f64, f64, f64, String)>,
    text: String,
}

impl OutputDev for LayoutText {
    fn begin_page(
        &mut self,
        _: u32,
        _: &MediaBox,
        _: Option<(f64, f64, f64, f64)>,
    ) -> Result<(), OutputError> {
        Ok(())
    }

    fn output_character(
        &mut self,
        trm: &Transform,
        width: f64,
        _: f64,
        font_size: f64,
        character: &str,
    ) -> Result<(), OutputError> {
        let size = (trm.m11 * font_size).abs();
        let word = self
            .word
            .get_or_insert_with(|| (trm.m31, trm.m32, 0.0, size, String::new()));
        word.2 = trm.m31 + width * size - word.0;
        word.4.push_str(character);
        Ok(())
    }

    fn end_page(&mut self) -> Result<(), OutputError> {
        // Content-stream order is not reading order: browser PDFs can emit
        // table rows before their series headings. Reconstruct visual lines.
        self.glyphs.sort_by(|a, b| b.1.total_cmp(&a.1));
        let mut start = 0;
        while start < self.glyphs.len() {
            let y = self.glyphs[start].1;
            let end = self.glyphs[start..]
                .iter()
                .position(|g| (g.1 - y).abs() > 2.0)
                .map_or(self.glyphs.len(), |offset| start + offset);
            self.glyphs[start..end].sort_by(|a, b| a.0.total_cmp(&b.0));
            let mut last_end = None;
            for (x, _, width, size, character) in &self.glyphs[start..end] {
                if let Some(previous) = last_end {
                    let gap = x - previous;
                    if gap > size * 1.5 {
                        self.text.push_str("   ");
                    } else if gap > size * 0.15 && !self.text.ends_with(' ') {
                        self.text.push(' ');
                    }
                }
                self.text.push_str(character);
                last_end = Some(x + width);
            }
            self.text.push('\n');
            start = end;
        }
        self.text.push('\n');
        self.glyphs.clear();
        Ok(())
    }

    fn begin_word(&mut self) -> Result<(), OutputError> {
        self.word = None;
        Ok(())
    }
    fn end_word(&mut self) -> Result<(), OutputError> {
        if let Some(word) = self.word.take() {
            self.glyphs.push(word);
        }
        Ok(())
    }
    fn end_line(&mut self) -> Result<(), OutputError> {
        Ok(())
    }
}

pub fn parse_stampworld_pdf<P: AsRef<Path>>(
    path: P,
    country: &str,
    start_year: &str,
    end_year: &str,
) -> Result<Vec<PdfStampRecord>, String> {
    let start = start_year.to_string();
    let end = end_year.to_string();
    let _images = extract_stampworld_pdf_images(path.as_ref(), country, &start, &end)?;
    println!("path imgs: {}", path.as_ref().display());
    println!("imgs: {}", _images.len());
    println!("start year: {}", start);
    println!("end year: {}", end);
    println!("country: {}", country);
    parse_stampworld_text(&extract_stampworld_pdf_text(path)?)
}

pub fn extract_stampworld_pdf_content<P: AsRef<Path>>(
    path: P,
) -> Result<(String, Vec<oxidize_pdf::operations::ExtractedImage>), String> {
    let text = extract_stampworld_pdf_text(path.as_ref())?;
    let images = extract_stampworld_pdf_images(path.as_ref(), "", "", "")?;
    Ok((text, images))
}

pub fn extract_stampworld_pdf_images<P: AsRef<Path>>(
    path: P,
    country: &str,
    start_year: &str,
    end_year: &str,
) -> Result<Vec<oxidize_pdf::operations::ExtractedImage>, String> {
    let pdf_path = path.as_ref();
    // Get the filename without extension, e.g. "Denmark -Postage stamps (1980 - 1989) - Page 4"
    let stem = pdf_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("extracted");

    let output_dir = path
        .as_ref()
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(stem);

    let options = ExtractImagesOptions {
        output_dir, // or whatever you want
        name_pattern: format!(
            "{country}_{start_year}-{end_year}_{{page}}_img_{{index}}.{{format}}",
            country = country,
            start_year = start_year,
            end_year = end_year
        ),
        extract_inline: true,
        min_size: Some(20), // ignore tiny icons if desired
        create_dir: true,
        ..Default::default()
    };

    extract_images_from_pdf(path.as_ref(), options)
        .map_err(|e| format!("Failed to extract images: {}", e))
}

pub fn extract_stampworld_pdf_text<P: AsRef<Path>>(path: P) -> Result<String, String> {
    let mut document = Document::load(path.as_ref())
        .map_err(|e| format!("Failed to read PDF {}: {}", path.as_ref().display(), e))?;
    if document.is_encrypted() {
        document
            .decrypt("")
            .map_err(|e| format!("Cannot decrypt PDF: {}", e))?;
    }
    let mut output = LayoutText::default();
    pdf_extract::output_doc(&document, &mut output)
        .map_err(|e| format!("Failed to extract PDF text: {}", e))?;
    Ok(output.text)
}

fn normalized(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn metadata(text: &str, record: &mut PdfStampRecord) -> Result<(), String> {
    let labels =
        Regex::new(r"(?i)(WM|Watermark|Sheetsize|Design|Designer|Engraving|Perforation):\s*")
            .map_err(|e| e.to_string())?;
    let matches: Vec<_> = labels.captures_iter(text).collect();
    for (index, capture) in matches.iter().enumerate() {
        let Some(label) = capture.get(1) else {
            continue;
        };
        let Some(full) = capture.get(0) else { continue };
        let end = matches
            .get(index + 1)
            .and_then(|c| c.get(0))
            .map_or(text.len(), |m| m.start());
        let value = normalized(&text[full.end()..end])
            .trim_end_matches('.')
            .trim()
            .to_string();
        match label.as_str().to_ascii_lowercase().as_str() {
            "wm" | "watermark" => record.watermark = value,
            "design" | "designer" => record.designer = value,
            "engraving" => record.engraver = value,
            "perforation" => record.perforation = value,
            _ => {}
        }
    }
    Ok(())
}

pub fn parse_stampworld_text(text: &str) -> Result<Vec<PdfStampRecord>, String> {
    let heading = Regex::new(r"^((?:18|19|20)\d{2})(?:\s*[-–]\s*\d{2,4})?\s+(.+)$")
        .map_err(|e| e.to_string())?;
    let row_start =
        Regex::new(r"^\d+[a-zA-Z]?\s+[A-Z][A-Z0-9]*\s+\S*[0-9¼½¾⅓⅔⅛⅜⅝⅞]\S*(?:\s+\S+)?\s{2,}")
            .map_err(|e| e.to_string())?;
    let row = Regex::new(
        r"(?s)^(\d+[a-zA-Z]?)\s+([A-Z][A-Z0-9]*)\s+(.+?)\s{2,}(.*?)\s*(?:\(([\d.,]+(?:\s*mill)?)\)\s*)?([\d.,]+|-)\s+([\d.,]+|-)\s+([\d.,]+|-)\s+([\d.,]+|-)\s+([A-Z]{3})\s*$",
    ).map_err(|e| e.to_string())?;
    let columns = Regex::new(r"\s{2,}").map_err(|e| e.to_string())?;
    let mut group = PdfStampRecord::default();
    let mut records: Vec<PdfStampRecord> = Vec::new();
    let mut pending = String::new();
    for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
        if let Some(capture) = heading.captures(line).filter(|_| !row_start.is_match(line)) {
            if !pending.is_empty() {
                return Err(format!("Incomplete StampWorld row: {}", pending));
            }
            group = PdfStampRecord {
                year: capture[1].to_string(),
                series: normalized(line),
                ..Default::default()
            };
        } else if line.contains("WM:") || line.contains("Design:") || line.contains("Perforation:")
        {
            metadata(line, &mut group)?;
        } else if row_start.is_match(line) || !pending.is_empty() {
            if !pending.is_empty() {
                pending.push(' ');
            }
            pending.push_str(line);
            if let Some(capture) = row.captures(&pending) {
                if group.year.is_empty() {
                    return Err("StampWorld row has no series/year heading".to_string());
                }
                let body = capture[4].trim();
                let mut fields = columns.splitn(body, 2);
                let color = fields.next().unwrap_or_default();
                let description = fields.next().unwrap_or_default();
                records.push(PdfStampRecord {
                    catalog_number: capture[1].to_string(),
                    stamp_type: capture[2].to_string(),
                    denomination: capture[3].to_string(),
                    color: normalized(color),
                    description: normalized(description),
                    stamps_issued: match capture.get(5) {
                        Some(value) => {
                            if let Some(millions) = value.as_str().strip_suffix("mill") {
                                let amount = millions
                                    .trim()
                                    .replace(',', ".")
                                    .parse::<f64>()
                                    .map_err(|e| format!("Invalid issued quantity: {}", e))?;
                                format!("{:.0}", amount * 1_000_000.0)
                            } else {
                                value.as_str().to_string()
                            }
                        }
                        None => String::new(),
                    },
                    mint_price: capture[6].to_string(),
                    unused_price: capture[7].to_string(),
                    used_price: capture[8].to_string(),
                    fdc_price: capture[9].to_string(),
                    currency: capture[10].to_string(),
                    ..group.clone()
                });
                pending.clear();
            }
        } else if !line.starts_with("No.")
            && !line.starts_with("Type")
            && !line.starts_with("Description")
            && !line.starts_with("Now showing:")
            && !line.chars().next().is_some_and(|c| c.is_ascii_digit())
        {
            // Wrapped descriptions follow the first visual row, after its prices.
            if let Some(record) = records.last_mut()
                && record.series == group.series && !record.description.is_empty() {
                    record.description.push(' ');
                    record.description.push_str(&normalized(line));
                }
        }
    }
    if !pending.is_empty() {
        return Err(format!("Incomplete StampWorld row: {}", pending));
    }
    if records.is_empty() {
        return Err(
            "No StampWorld stamp records found (scanned/image-only PDFs require OCR)".to_string(),
        );
    }
    Ok(records)
}

pub fn parse_stampworld_html(path: &Path) -> Result<Vec<PdfStampRecord>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("Cannot read HTML: {}", e))?;
    let document = Html::parse_document(&text);
    let selector = |value: &str| Selector::parse(value).map_err(|e| e.to_string());
    let groups = selector("div.content_table")?;
    let header = selector(".table_header a")?;
    let details = selector(".table_header p")?;
    let rows = selector("tr.stamp_tr")?;
    let cells = selector("th, td")?;
    let year = Regex::new(r"^((?:18|19|20)\d{2})").map_err(|e| e.to_string())?;
    let mut records = Vec::new();
    for group in document.select(&groups) {
        let series = group
            .select(&header)
            .next()
            .map(|node| normalized(&node.text().collect::<String>()))
            .unwrap_or_default();
        let mut template = PdfStampRecord {
            series,
            ..Default::default()
        };
        template.year = year
            .captures(&template.series)
            .map(|c| c[1].to_string())
            .ok_or("StampWorld HTML group has no year")?;
        for node in group.select(&details) {
            metadata(&normalized(&node.text().collect::<String>()), &mut template)?;
        }
        for row in group.select(&rows) {
            let values: Vec<_> = row
                .select(&cells)
                .map(|cell| normalized(&cell.text().collect::<String>()))
                .collect();
            if values.len() < 14 {
                return Err("StampWorld HTML row has fewer than 14 columns".to_string());
            }
            if !values[0].chars().all(|c| c.is_ascii_alphanumeric()) {
                continue;
            }
            records.push(PdfStampRecord {
                catalog_number: values[0].clone(),
                stamp_type: values[1].clone(),
                denomination: values[2].clone(),
                color: values[4].clone(),
                description: values[6].clone(),
                stamps_issued: values[7].trim_matches(['(', ')']).to_string(),
                mint_price: values[9].clone(),
                unused_price: values[10].clone(),
                used_price: values[11].clone(),
                fdc_price: values[12].clone(),
                currency: values[13].clone(),
                ..template.clone()
            });
        }
    }
    if records.is_empty() {
        return Err("No StampWorld stamp records found in HTML".to_string());
    }
    Ok(records)
}
