---
type: Rust Function
title: normalize_country_input
resource: src/flagdb.rs#L133-L135
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/flagdb/copy_coa_for_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn normalize_country_input(country: &str) -> String`

# Called by

- [copy_coa_for_album](../../../functions/src/flagdb/copy_coa_for_album.md)