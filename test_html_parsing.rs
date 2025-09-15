use scraper::{Html, Selector};

fn main() {
    // Test HTML from the user's example
    let html = r#"
    <div class="container-fluid content_table" id="group_box_72571">
      <div class="row">
        <div class="col-12 table_header">
          <a href="https://www.stampworld.com/stamps/Canada/Postage-stamps/g0099//">
            1922 -1925 King George V in Admiral Uniform - New Colors and Values
          </a>
          <p>WM: None</p>
          <p>Design: Robert Savage</p>
          <p>Perforation: 12</p>
        </div>
      </div>
    </div>
    "#;

    let document = Html::parse_document(html);

    // Test the new selector logic
    let table_header_selector = Selector::parse(".table_header").unwrap();
    let p_selector = Selector::parse("p").unwrap();
    let mut details_vec = Vec::new();

    if let Some(table_header) = document.select(&table_header_selector).next() {
        for p in table_header.select(&p_selector) {
            let p_text = p.text().collect::<String>();
            if !p_text.is_empty() {
                details_vec.push(p_text);
            }
        }
    }

    println!("Extracted details: {:?}", details_vec);

    // Test field extraction
    let mut watermark = String::new();
    let mut design = String::new();
    let mut engraving = String::new();
    let mut perforation = String::new();

    for detail in &details_vec {
        let detail_lower = detail.to_lowercase();

        // Extract watermark
        if detail_lower.contains("watermark:") {
            watermark = detail.split(":").nth(1).unwrap_or("").trim().to_string();
        } else if detail_lower.starts_with("wm") && detail.contains(":") {
            watermark = detail.split(":").nth(1).unwrap_or("").trim().to_string();
        }

        // Extract design
        if detail_lower.contains("design:") {
            design = detail.split(":").nth(1).unwrap_or("").trim().to_string();
        }

        // Extract engraving
        if detail_lower.contains("engraving:") {
            engraving = detail.split(":").nth(1).unwrap_or("").trim().to_string();
        }

        // Extract perforation
        if detail_lower.contains("perforation:") {
            perforation = detail.split(":").nth(1).unwrap_or("").trim().to_string();
        }
    }

    println!("Extracted fields:");
    println!("  Watermark: '{}'", watermark);
    println!("  Design: '{}'", design);
    println!("  Engraving: '{}'", engraving);
    println!("  Perforation: '{}'", perforation);
}
