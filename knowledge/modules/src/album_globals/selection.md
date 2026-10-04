---
type: Rust Module
title: selection
resource: src/album_globals/selection.rs#L1-L251
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-generator-new-helpers-fetch-and-filter-stamps-for-selection-stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-connection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-btreemap-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-utils-init-db
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ColnectPdfFilters](../../../classes/src/album_globals/selection/ColnectPdfFilters.md)
- [normalized](../../../functions/src/album_globals/selection/normalized.md)
- [validate](../../../functions/src/album_globals/selection/ColnectPdfFilters/validate.md)
- [is_all](../../../functions/src/album_globals/selection/ColnectPdfFilters/is_all.md)
- [matches](../../../functions/src/album_globals/selection/ColnectPdfFilters/matches.md)
- [ColnectPdfOptions](../../../classes/src/album_globals/selection/ColnectPdfOptions.md)
- [load_stamps](../../../functions/src/album_globals/selection/load_stamps.md)
- [colnect_pdf_options](../../../functions/src/album_globals/selection/colnect_pdf_options.md)
- [prepare_pdf_stamps](../../../functions/src/album_globals/selection/prepare_pdf_stamps.md)
- [fixture](../../../functions/src/album_globals/selection/fixture.md)
- [count](../../../functions/src/album_globals/selection/count.md)
- [dynamic_options_use_resolved_album_records_and_keep_countries_separate](../../../functions/src/album_globals/selection/dynamic_options_use_resolved_album_records_and_keep_countries_separate.md)
- [selection_composes_or_and_years_and_preserves_metadata_precedence](../../../functions/src/album_globals/selection/selection_composes_or_and_years_and_preserves_metadata_precedence.md)
- [selection_reports_empty_results_and_database_errors](../../../functions/src/album_globals/selection/selection_reports_empty_results_and_database_errors.md)
- [selection_allows_country_without_reference_tables](../../../functions/src/album_globals/selection/selection_allows_country_without_reference_tables.md)

# Imports

- `super::{generator_new::helpers::fetch_and_filter_stamps_for_selection, Stamp}`
- `rusqlite::Connection`
- `serde::{Deserialize, Serialize}`
- `std::collections::{BTreeMap, HashMap}`
- `super::*`
- `crate::db_utils::init_db`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)