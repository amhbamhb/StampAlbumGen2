---
type: Rust Function
title: ensure_coat_of_arms
resource: src/flagdb.rs#L317-L320
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stampworld/fetch/ReqwestFetcher/new_for_api
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/ensure_coat_of_arms_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/copy_coa_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/ensure_coa_integration/ensure_coa_from_local_svg_creates_png
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/ensure_coa_local/ensure_coa_nl_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/stamp_flags_dir_lookup/stamp_flags_dir_lookup_found_coa_svg
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ensure_coat_of_arms(alpha2: &str) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [new_for_api](../../../functions/src/stampworld/fetch/ReqwestFetcher/new_for_api.md)
- [ensure_coat_of_arms_with_fetcher](../../../functions/src/flagdb/ensure_coat_of_arms_with_fetcher.md)

# Called by

- [copy_coa_for_album](../../../functions/src/flagdb/copy_coa_for_album.md)
- [ensure_coa_from_local_svg_creates_png](../../../functions/tests/ensure_coa_integration/ensure_coa_from_local_svg_creates_png.md)
- [ensure_coa_nl_local](../../../functions/tests/ensure_coa_local/ensure_coa_nl_local.md)
- [stamp_flags_dir_lookup_found_coa_svg](../../../functions/tests/stamp_flags_dir_lookup/stamp_flags_dir_lookup_found_coa_svg.md)