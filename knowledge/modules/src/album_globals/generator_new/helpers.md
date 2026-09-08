---
type: Rust Module
title: helpers
resource: src/album_globals/generator_new/helpers.rs#L1-L951
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-album-globals-structs-genericattrs-stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-colnect-db-get-colnect-info-get-colnect-info-by-id
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-utils-get-stamps-by-series
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-utils-update-image-dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-image-utils-estimate-real-dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-connection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-optionalextension
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-result-as-sqlresult
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [PlaceholderOptions](../../../../classes/src/album_globals/generator_new/helpers/PlaceholderOptions.md)
- [PlaceholderCallOptions](../../../../classes/src/album_globals/generator_new/helpers/PlaceholderCallOptions.md)
- [emit_row_start_es](../../../../functions/src/album_globals/generator_new/helpers/emit_row_start_es.md)
- [emit_stamp_diag](../../../../functions/src/album_globals/generator_new/helpers/emit_stamp_diag.md)
- [build_page_text_template](../../../../functions/src/album_globals/generator_new/helpers/build_page_text_template.md)
- [build_series_page_text_centre_template](../../../../functions/src/album_globals/generator_new/helpers/build_series_page_text_centre_template.md)
- [build_scoped_page_text_centre_template](../../../../functions/src/album_globals/generator_new/helpers/build_scoped_page_text_centre_template.md)
- [build_date_page_text_centre_template](../../../../functions/src/album_globals/generator_new/helpers/build_date_page_text_centre_template.md)
- [build_watermark_page_text_centre_template](../../../../functions/src/album_globals/generator_new/helpers/build_watermark_page_text_centre_template.md)
- [build_engraving_page_text_centre_template](../../../../functions/src/album_globals/generator_new/helpers/build_engraving_page_text_centre_template.md)
- [StoredColnectMeta](../../../../classes/src/album_globals/generator_new/helpers/StoredColnectMeta.md)
- [load_stored_colnect_meta](../../../../functions/src/album_globals/generator_new/helpers/load_stored_colnect_meta.md)
- [set_attr_if_non_empty](../../../../functions/src/album_globals/generator_new/helpers/set_attr_if_non_empty.md)
- [set_attr_if_missing](../../../../functions/src/album_globals/generator_new/helpers/set_attr_if_missing.md)
- [leading_year_from_series](../../../../functions/src/album_globals/generator_new/helpers/leading_year_from_series.md)
- [fetch_and_filter_stamps](../../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps.md)
- [estimate_and_ensure_placeholders](../../../../functions/src/album_globals/generator_new/helpers/estimate_and_ensure_placeholders.md)
- [ensure_placeholder_for_stamp_and_update_db](../../../../functions/src/album_globals/generator_new/helpers/ensure_placeholder_for_stamp_and_update_db.md)

# Imports

- `crate::album_globals::structs::{GenericAttrs, Stamp}`
- `crate::colnect::db::{get_colnect_info, get_colnect_info_by_id}`
- `crate::config::Config`
- `crate::db_utils::get_stamps_by_series`
- `crate::db_utils::update_image_dimensions`
- `crate::image_utils::estimate_real_dimensions`
- `rusqlite::Connection`
- `rusqlite::OptionalExtension`
- `rusqlite::Result as SqlResult`
- `std::io::Write`

# Member of

- [stamp_album_gen2](../../../../packages/stamp_album_gen2.md)