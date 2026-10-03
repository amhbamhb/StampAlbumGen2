---
type: Rust Module
title: country_url
resource: src/stampworld/country_url.rs#L1-L271
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/std-collections-hashmap-io-write-path-path-sync-mutex
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

- [country_page_url](../../../functions/src/stampworld/country_url/country_page_url.md)
- [listing_url](../../../functions/src/stampworld/country_url/listing_url.md)
- [listing_url_from_country_url](../../../functions/src/stampworld/country_url/listing_url_from_country_url.md)
- [slug_from_url](../../../functions/src/stampworld/country_url/slug_from_url.md)
- [validate_country](../../../functions/src/stampworld/country_url/validate_country.md)
- [load_country_url](../../../functions/src/stampworld/country_url/load_country_url.md)
- [save_country_url](../../../functions/src/stampworld/country_url/save_country_url.md)
- [save_mapping_at](../../../functions/src/stampworld/country_url/save_mapping_at.md)
- [multiword_names_and_explicit_mappings_build_the_expected_urls](../../../functions/src/stampworld/country_url/multiword_names_and_explicit_mappings_build_the_expected_urls.md)
- [invalid_urls_cannot_be_saved_as_country_paths](../../../functions/src/stampworld/country_url/invalid_urls_cannot_be_saved_as_country_paths.md)
- [saved_mappings_survive_reload_and_preserve_other_settings](../../../functions/src/stampworld/country_url/saved_mappings_survive_reload_and_preserve_other_settings.md)
- [first_save_creates_a_mapping_file_and_invalid_table_is_not_overwritten](../../../functions/src/stampworld/country_url/first_save_creates_a_mapping_file_and_invalid_table_is_not_overwritten.md)

# Imports

- `std::{collections::HashMap, io::Write, path::Path, sync::Mutex}`
- `super::*`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)