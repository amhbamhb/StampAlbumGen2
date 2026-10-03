---
type: Rust Function
title: country_page_url
resource: src/stampworld/country_url.rs#L9-L15
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/stampworld/country_url/listing_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/load_country_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/save_country_url
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn country_page_url(slug: &str) -> String`

# Called by

- [listing_url](../../../../functions/src/stampworld/country_url/listing_url.md)
- [load_country_url](../../../../functions/src/stampworld/country_url/load_country_url.md)
- [save_country_url](../../../../functions/src/stampworld/country_url/save_country_url.md)