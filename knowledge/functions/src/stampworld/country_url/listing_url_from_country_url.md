---
type: Rust Function
title: listing_url_from_country_url
resource: src/stampworld/country_url.rs#L37-L44
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stampworld/country_url/slug_from_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/listing_url
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn listing_url_from_country_url( value: &str, start_year: i32, end_year: i32, ) -> Result<String, String>`

# Calls

- [slug_from_url](../../../../functions/src/stampworld/country_url/slug_from_url.md)
- [listing_url](../../../../functions/src/stampworld/country_url/listing_url.md)

# Called by

- [run_scraping_mode](../../../../functions/src/modes/run_scraping_mode.md)