use stamp_album_gen2::config::Config;
use stamp_album_gen2::wikipedia::formatters;

#[test]
fn paragraph_vspace_uses_configured_value() {
    let mut cfg = Config::with_defaults();
    cfg.wikipedia_paragraph_vspace = Some(1.8);
    cfg.wikipedia_paragraph_font_size = Some(12);

    let content = "This is a short paragraph to test vspace formatting.";
    let formatted = formatters::format_wikipedia_content_for_album(content, &cfg);

    // Find the PAGE_TEXT_PARAGRAPH_START line and extract the vspace value
    let mut found = false;
    for line in formatted.iter() {
        if line.starts_with("PAGE_TEXT_PARAGRAPH_START(") {
            // Example: PAGE_TEXT_PARAGRAPH_START(HN 12 JUSTIFIED 10.0 1.8 10.0)
            let parts: Vec<&str> = line
                .trim_start_matches("PAGE_TEXT_PARAGRAPH_START(")
                .trim_end_matches(')')
                .split_whitespace()
                .collect();
            // vspace is the 5th item (0-based index 4) when format is: <font> <size> <align> <pad> <vspace> <rightpad>
            assert!(
                parts.len() >= 6,
                "unexpected PAGE_TEXT_PARAGRAPH_START format: {}",
                line
            );
            let vspace_str = parts[4];
            // Should be formatted with one decimal place
            assert_eq!(vspace_str, "1.8");
            found = true;
        }
    }
    assert!(
        found,
        "PAGE_TEXT_PARAGRAPH_START not found in formatted output"
    );
}
