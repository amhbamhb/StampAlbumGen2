---
type: Rust Function
title: cache_country_info
resource: src/flagdb.rs#L846-L872
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/extract_country_name_from_nexperts_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/extract_flag_from_nexperts_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/extract_coa_from_nexperts_html
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/ensure_coat_of_arms_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn cache_country_info(alpha: &str, html: &str) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [open](../../../functions/src/external_album/open.md)
- [extract_country_name_from_nexperts_html](../../../functions/src/flagdb/extract_country_name_from_nexperts_html.md)
- [extract_flag_from_nexperts_html](../../../functions/src/flagdb/extract_flag_from_nexperts_html.md)
- [extract_coa_from_nexperts_html](../../../functions/src/flagdb/extract_coa_from_nexperts_html.md)

# Called by

- [ensure_coat_of_arms_with_fetcher](../../../functions/src/flagdb/ensure_coat_of_arms_with_fetcher.md)