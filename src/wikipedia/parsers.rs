use scraper::{Html, Selector};

/// Parse Grokipedia HTML and extract clean text content
pub fn parse_grokipedia_html(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = Html::parse_document(html);

    // Try to find the main content area
    let content_selector = Selector::parse("article").unwrap();
    let mut content = String::new();

    if let Some(article) = document.select(&content_selector).next() {
        // Extract text from paragraphs and headings
        let paragraph_selector = Selector::parse("p, h1, h2, h3, h4, h5, h6").unwrap();

        for element in article.select(&paragraph_selector) {
            let tag_name = element.value().name();
            let text = element
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            if !text.is_empty() {
                // Add section headers for h1-h6
                if tag_name.starts_with('h') && tag_name.len() == 2 {
                    let level = tag_name.chars().nth(1).unwrap().to_digit(10).unwrap_or(1);
                    content.push_str(&format!("{} {}\n\n", "#".repeat(level as usize), text));
                } else {
                    content.push_str(&format!("{}\n\n", text));
                }
            }
        }
    }

    if content.is_empty() {
        return Err("No content found in Grokipedia HTML".into());
    }

    Ok(content)
}

/// Parse Wikipedia HTML and extract clean text content with sections
pub fn parse_wikipedia_html(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = Html::parse_document(html);

    // Find the main content area
    let content_selector = Selector::parse(".mw-content-ltr").unwrap();
    let mut content = String::new();

    if let Some(content_div) = document.select(&content_selector).next() {
        // Extract sections and paragraphs
        let section_selector = Selector::parse("h2, h3, h4, h5, h6, p").unwrap();

        for element in content_div.select(&section_selector) {
            let tag_name = element.value().name();
            let text = element
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            // Skip empty elements and references
            if text.is_empty() || text.starts_with('[') && text.ends_with(']') {
                continue;
            }

            // Skip edit links and other Wikipedia artifacts
            if text.contains("edit") && text.len() < 20 {
                continue;
            }

            // Add section headers
            if tag_name.starts_with('h') && tag_name.len() == 2 {
                let level = tag_name.chars().nth(1).unwrap().to_digit(10).unwrap_or(2);
                content.push_str(&format!("{} {}\n\n", "#".repeat(level as usize), text));
            } else {
                // Regular paragraph
                content.push_str(&format!("{}\n\n", text));
            }
        }
    }

    if content.is_empty() {
        return Err("No content found in Wikipedia HTML".into());
    }

    Ok(content)
}
