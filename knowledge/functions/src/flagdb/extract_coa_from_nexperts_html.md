---
type: Rust Function
title: extract_coa_from_nexperts_html
resource: src/flagdb.rs#L1084-L1112
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/flagdb/ensure_coat_of_arms_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/cache_country_info
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_coa_from_nexperts_html(html: &str, alpha: &str) -> Option<String>`

# Called by

- [ensure_coat_of_arms_with_fetcher](../../../functions/src/flagdb/ensure_coat_of_arms_with_fetcher.md)
- [cache_country_info](../../../functions/src/flagdb/cache_country_info.md)