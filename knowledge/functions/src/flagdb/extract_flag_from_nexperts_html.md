---
type: Rust Function
title: extract_flag_from_nexperts_html
resource: src/flagdb.rs#L874-L903
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

`fn extract_flag_from_nexperts_html(html: &str, alpha: &str) -> Option<String>`

# Called by

- [cache_country_info](../../../functions/src/flagdb/cache_country_info.md)