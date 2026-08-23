use crate::config::Config;

/// Estimate the height (in mm) required to display text content on a page
/// Uses a simple character-count based approach that's more robust across font sizes
pub fn estimate_text_height(text: &str, config: &Config) -> f64 {
    // Simple approach: count characters and estimate based on average content density
    // This avoids complex font math that doesn't match PDF rendering

    // Get the configured page dimensions
    let page_width = config.album_page_width_mm.unwrap_or(216.0) as f64;
    let left_margin = config.page_left_margin_mm.unwrap_or(10.0) as f64;
    let right_margin = config.page_right_margin_mm.unwrap_or(10.0) as f64;

    // Account for paragraph padding (left and right margins inside paragraphs)
    let wiki_left_pad = 8.0;
    let wiki_right_pad = 8.0;
    let actual_content_width =
        page_width - left_margin - right_margin - wiki_left_pad - wiki_right_pad;

    // Get paragraph font size - this is KEY to proper estimation
    let paragraph_font_size = config.wikipedia_paragraph_font_size.unwrap_or(12) as f64;

    // Empirical formula: based on actual PDF rendering observations
    // At 11pt with ~180mm width, roughly 80-85 characters fit per line
    // So chars_per_line = width_mm * (font_size / 11) * chars_per_mm_at_11pt
    // At 11pt, approximately 2.0 chars per mm (for proportional fonts with padding)
    let chars_per_line = (actual_content_width * (paragraph_font_size / 11.0) * 2.0).max(30.0);

    // Count total characters and newlines
    let total_chars = text.len() as f64;
    let newline_count = text.lines().count() as f64;

    // Estimate number of visual lines (accounting for wrapping)
    let visual_lines = (total_chars / chars_per_line).max(newline_count);

    // Line height in mm depends on font size
    // Empirical: 11pt ~= 4.5mm per line (including space between)
    // Scale for other font sizes: line_height = 4.5 * (font_size / 11)
    let line_height_mm = (4.5 * (paragraph_font_size / 11.0)).max(2.0);

    // Calculate base text height
    let text_height = visual_lines * line_height_mm;

    // Count paragraphs - each adds overhead for padding and spacing
    let num_paragraphs = text.split("PAGE_TEXT_PARAGRAPH_END").count().max(1);

    // Paragraph separation overhead - increases with font size
    // Each paragraph boundary adds spacing: roughly 0.2 * font_size mm
    let paragraph_overhead = (paragraph_font_size * 0.25) * (num_paragraphs as f64 - 1.0);

    // Header spacing if this text contains section breaks (heuristic)
    // Count approximate headers by looking for common transitions
    let header_spacing_factor = if text.contains("\n\n") { 5.0 } else { 0.0 };

    // Total estimate: be CONSERVATIVE by adding extra margin
    // Use 1.4x safety factor to account for rendering variations

    (text_height + paragraph_overhead + header_spacing_factor) * 1.4
}

/// Estimate the height of a section header
pub fn estimate_header_height(header_text: &str, config: &Config) -> f64 {
    // Simple approach based on font size
    let section_title_font_size = config.wikipedia_section_title_font_size.unwrap_or(14) as f64;

    // Header line height scales with font size: at 14pt ~= 6mm per line
    // Scale formula: (font_size / 14) * 6.0
    let line_height_mm = (section_title_font_size / 14.0 * 6.0).max(3.0);

    // Headers typically need 1-2 lines depending on text length
    // Rough heuristic: 40 characters per line at default font size
    let header_lines = ((header_text.len() as f64) / (40.0 * section_title_font_size / 14.0))
        .ceil()
        .max(1.0);

    let text_height = header_lines * line_height_mm;

    // Add spacing BEFORE and AFTER header
    let header_spacing = config.wikipedia_section_title_spacing.unwrap_or(2.0);
    let spacing_height = (header_spacing * 0.5) + header_spacing;

    // Apply safety factor for headers too
    (text_height + spacing_height) * 1.2
}
