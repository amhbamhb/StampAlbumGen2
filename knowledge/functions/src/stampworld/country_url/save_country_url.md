---
type: Rust Function
title: save_country_url
resource: src/stampworld/country_url.rs#L102-L118
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stampworld/country_url/validate_country
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/slug_from_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/slug_override_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/save_mapping_at
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/country_page_url
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/handle_country_url_request
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn save_country_url(country: &str, value: &str) -> Result<(String, bool), String>`

# Calls

- [validate_country](../../../../functions/src/stampworld/country_url/validate_country.md)
- [slug_from_url](../../../../functions/src/stampworld/country_url/slug_from_url.md)
- [slug_override_path](../../../../functions/src/config/slug_override_path.md)
- [save_mapping_at](../../../../functions/src/stampworld/country_url/save_mapping_at.md)
- [country_page_url](../../../../functions/src/stampworld/country_url/country_page_url.md)

# Called by

- [handle_country_url_request](../../../../functions/src/review_ui/http_handlers/handle_country_url_request.md)