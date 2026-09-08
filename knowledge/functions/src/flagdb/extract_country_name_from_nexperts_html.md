---
type: Rust Function
title: extract_country_name_from_nexperts_html
resource: src/flagdb.rs#L1064-L1082
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/flagdb/cache_country_info
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_country_name_from_nexperts_html(html: &str) -> Option<String>`

# Called by

- [cache_country_info](../../../functions/src/flagdb/cache_country_info.md)