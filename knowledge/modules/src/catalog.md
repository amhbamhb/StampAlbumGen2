---
type: Rust Module
title: catalog
resource: src/catalog.rs#L1-L297
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-catalog-display-name-infer-catalog-type-for-number-normalize-catalog-number-normalize-catalog-type-preferred-catalog-entry-preferred-catalog-number
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [normalize_catalog_type](../../functions/src/catalog/normalize_catalog_type.md)
- [catalog_display_name](../../functions/src/catalog/catalog_display_name.md)
- [normalize_catalog_number](../../functions/src/catalog/normalize_catalog_number.md)
- [parse_catalog_codes](../../functions/src/catalog/parse_catalog_codes.md)
- [preferred_catalog_entry](../../functions/src/catalog/preferred_catalog_entry.md)
- [preferred_catalog_number](../../functions/src/catalog/preferred_catalog_number.md)
- [infer_catalog_type_for_number](../../functions/src/catalog/infer_catalog_type_for_number.md)
- [strips_country_prefix_from_catalog_number](../../functions/src/catalog/strips_country_prefix_from_catalog_number.md)
- [prefers_requested_catalog_family](../../functions/src/catalog/prefers_requested_catalog_family.md)
- [infers_catalog_type_from_normalized_number](../../functions/src/catalog/infers_catalog_type_from_normalized_number.md)
- [pol_and_aliases_normalize_and_display_correctly](../../functions/src/catalog/pol_and_aliases_normalize_and_display_correctly.md)
- [nor_and_zum_aliases_display_correctly](../../functions/src/catalog/nor_and_zum_aliases_display_correctly.md)
- [all_known_catalog_codes_have_non_catalog_display_names](../../functions/src/catalog/all_known_catalog_codes_have_non_catalog_display_names.md)

# Imports

- `super::{
        catalog_display_name, infer_catalog_type_for_number, normalize_catalog_number,
        normalize_catalog_type, preferred_catalog_entry, preferred_catalog_number,
    }`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)