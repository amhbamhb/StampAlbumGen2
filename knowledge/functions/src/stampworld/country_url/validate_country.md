---
type: Rust Function
title: validate_country
resource: src/stampworld/country_url.rs#L82-L88
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/stampworld/country_url/load_country_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/save_country_url
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn validate_country(country: &str) -> Result<&str, String>`

# Called by

- [load_country_url](../../../../functions/src/stampworld/country_url/load_country_url.md)
- [save_country_url](../../../../functions/src/stampworld/country_url/save_country_url.md)