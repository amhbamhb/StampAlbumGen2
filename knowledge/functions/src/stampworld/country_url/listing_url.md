---
type: Rust Function
title: listing_url
resource: src/stampworld/country_url.rs#L18-L34
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/country_page_url
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/listing_url_from_country_url
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn listing_url( country: &str, start_year: i32, end_year: i32, overrides: &HashMap<String, String>, ) -> String`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [country_page_url](../../../../functions/src/stampworld/country_url/country_page_url.md)

# Called by

- [run_scraping_mode](../../../../functions/src/modes/run_scraping_mode.md)
- [listing_url_from_country_url](../../../../functions/src/stampworld/country_url/listing_url_from_country_url.md)