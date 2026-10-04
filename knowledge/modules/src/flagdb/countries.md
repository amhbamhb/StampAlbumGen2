---
type: Rust Module
title: countries
resource: src/flagdb/countries.rs#L1-L125
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [normalize](../../../functions/src/flagdb/countries/normalize.md)
- [valid_code](../../../functions/src/flagdb/countries/valid_code.md)
- [parse_iso_codes](../../../functions/src/flagdb/countries/parse_iso_codes.md)
- [merge_overrides](../../../functions/src/flagdb/countries/merge_overrides.md)
- [bundled_map](../../../functions/src/flagdb/countries/bundled_map.md)
- [supplied_iso_file_has_246_unique_codes_and_names](../../../functions/src/flagdb/countries/supplied_iso_file_has_246_unique_codes_and_names.md)
- [malformed_or_duplicate_iso_rows_are_rejected](../../../functions/src/flagdb/countries/malformed_or_duplicate_iso_rows_are_rejected.md)
- [partial_overrides_keep_iso_coverage_and_invalid_overrides_do_not_apply](../../../functions/src/flagdb/countries/partial_overrides_keep_iso_coverage_and_invalid_overrides_do_not_apply.md)

# Imports

- `std::collections::HashMap`
- `super::*`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)