use crate::config::Config;

/// Estimate the height (in mm) required to display text content on a page
/// This includes calculations for font size, line wrapping, and spacing
pub fn estimate_text_height(text: &str, config: &Config) -> f64 {
    // Get configuration values
    let page_width = config.album_page_width_mm.unwrap_or(216.0) as f64;
    let text_line_height = config.text_line_height_mm.unwrap_or(2.6) as f64;

    // Estimate characters per line based on font size
    // This is a rough approximation - smaller fonts fit more characters per line
    let paragraph_font_size = config.wikipedia_paragraph_font_size.unwrap_or(12) as f64;
    let chars_per_line = (page_width / (paragraph_font_size * 0.6)).floor() as usize;

    // Count actual lines and estimate wrapped lines
    let mut total_lines = 0;
    for line in text.lines() {
        let line_len = line.len();
        if line_len == 0 {
            total_lines += 1; // Empty lines still take space
        } else {
            // Estimate how many display lines this text line will wrap to
            let wrapped_lines = ((line_len as f64) / (chars_per_line as f64)).ceil() as usize;
            total_lines += wrapped_lines.max(1);
        }
    }

    // Add spacing between paragraphs (vspace)
    let paragraph_vspace = config.wikipedia_paragraph_vspace.unwrap_or(5.0) as f64;
    let num_paragraphs = text.lines().filter(|l| !l.trim().is_empty()).count() as f64;

    // Calculate total height with conservative multiplier
    let text_height = (total_lines as f64) * text_line_height;
    let spacing_height = num_paragraphs * paragraph_vspace;
    let total_estimated_height = text_height + spacing_height;

    // Add 20% buffer for estimation errors
    total_estimated_height * 1.2
}

/// Estimate the height of a section header
pub fn estimate_header_height(header_text: &str, config: &Config) -> f64 {
    let text_line_height = config.text_line_height_mm.unwrap_or(2.6) as f64;

    // Headers typically take 1-2 lines with some extra spacing
    let header_lines = ((header_text.len() as f64) / 40.0).ceil() as f64; // Rough estimate
    let estimated_height = header_lines * text_line_height * 1.2; // 20% extra for header spacing

    // Add 30% buffer for headers (they can be more variable)
    estimated_height * 1.3
}
