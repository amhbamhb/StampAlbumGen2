---
type: Rust Function
title: slug_from_url
resource: src/stampworld/country_url.rs#L46-L80
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/parser/parse
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/stampworld/country_url/listing_url_from_country_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/save_country_url
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn slug_from_url(value: &str) -> Result<String, String>`

# Calls

- [parse](../../../../functions/src/external_album/parser/parse.md)

# Called by

- [listing_url_from_country_url](../../../../functions/src/stampworld/country_url/listing_url_from_country_url.md)
- [save_country_url](../../../../functions/src/stampworld/country_url/save_country_url.md)