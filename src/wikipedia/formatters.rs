use crate::config::Config;
use crate::wikipedia::models::ContentSource;

/// Generate AlbumEasy PAGE_TEXT_PARAGRAPH commands for content
pub fn format_content_for_album(
    content: &str,
    source: &ContentSource,
    config: &Config,
) -> Vec<String> {
    let mut lines = Vec::new();

    // Get configuration values with defaults
    let paragraph_font = config.wikipedia_paragraph_font.as_deref().unwrap_or("HN");
    let paragraph_font_size = config.wikipedia_paragraph_font_size.unwrap_or(12);
    let paragraph_alignment = config
        .wikipedia_paragraph_alignment
        .as_deref()
        .unwrap_or("JUSTIFIED");
    let paragraph_padding = config.wikipedia_paragraph_padding.unwrap_or(10.0);
    // Compute a default paragraph vspace that scales with the paragraph font size
    // if the config value is not explicitly set. This reduces excessive spacing
    // for smaller fonts and increases it for larger fonts.
    let paragraph_vspace = config.wikipedia_paragraph_vspace.unwrap_or_else(|| {
        // Rough heuristic: ~0.25 * font size (in mm) gives reasonable spacing
        (paragraph_font_size as f32) * 0.25
    });
    let paragraph_right_padding = config.wikipedia_paragraph_right_padding.unwrap_or(10.0);

    let section_title_font = config
        .wikipedia_section_title_font
        .as_deref()
        .unwrap_or("HB");
    let section_title_font_size = config.wikipedia_section_title_font_size.unwrap_or(14);

    // Add section header indicating source
    let source_name = match source {
        ContentSource::Grokipedia => "Grokipedia",
        ContentSource::Wikipedia => "Wikipedia",
    };

    lines.push(format!(
        "PAGE_TEXT ({} {} \"{} Postal History\" 3.0)",
        section_title_font, section_title_font_size, source_name
    ));

    // Process content line by line, handling section headers and paragraphs
    let content_lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < content_lines.len() {
        let line = content_lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        // Check if this is a section header (starts with #)
        if line.starts_with('#') {
            let header_text = line.trim_start_matches('#').trim();
            if !header_text.is_empty() {
                // Add section header
                lines.push(format!(
                    "PAGE_TEXT ({} {} \"{}\" 3.0)",
                    section_title_font, section_title_font_size, header_text
                ));
            }
        } else {
            // This is a paragraph - collect all consecutive non-empty lines
            let mut paragraph_lines = Vec::new();
            while i < content_lines.len() && !content_lines[i].trim().is_empty() {
                paragraph_lines.push(content_lines[i].trim());
                i += 1;
            }

            if !paragraph_lines.is_empty() {
                let paragraph_text = paragraph_lines.join(" ");

                // Start paragraph with formatting. Use the computed paragraph_vspace
                // which scales sensibly with font size when not explicitly configured.
                lines.push(format!(
                    "PAGE_TEXT_PARAGRAPH_START({} {} {} {:.1} {:.1} {:.1})",
                    paragraph_font,
                    paragraph_font_size,
                    paragraph_alignment,
                    paragraph_padding,
                    paragraph_vspace,
                    paragraph_right_padding
                ));

                // Add the paragraph content, escaping quotes
                let escaped_content = paragraph_text.replace('"', "\\\"");
                lines.push(escaped_content);

                // End paragraph
                lines.push("PAGE_TEXT_PARAGRAPH_END".to_string());
            }
        }

        i += 1;
    }

    lines
}

/// Legacy function for backwards compatibility
pub fn format_wikipedia_content_for_album(content: &str, config: &Config) -> Vec<String> {
    format_content_for_album(content, &ContentSource::Wikipedia, config)
}
