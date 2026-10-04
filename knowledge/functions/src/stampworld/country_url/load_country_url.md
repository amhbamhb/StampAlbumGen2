---
type: Rust Function
title: load_country_url
resource: src/stampworld/country_url.rs#L91-L99
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stampworld/country_url/validate_country
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/try_load_slug_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/country_page_url
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/start_bootstrap_stampworld_job
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/handle_country_url_request
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load_country_url(country: &str) -> Result<(String, bool), String>`

# Calls

- [validate_country](../../../../functions/src/stampworld/country_url/validate_country.md)
- [try_load_slug_overrides](../../../../functions/src/config/try_load_slug_overrides.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [country_page_url](../../../../functions/src/stampworld/country_url/country_page_url.md)

# Called by

- [start_bootstrap_stampworld_job](../../../../functions/src/review_ui/start_bootstrap_stampworld_job.md)
- [handle_country_url_request](../../../../functions/src/review_ui/http_handlers/handle_country_url_request.md)