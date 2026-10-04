---
type: Rust Module
title: generator
resource: src/album_globals/generator.rs#L1-L2953
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-constants-month-names
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-globals-compute-usable-height-write-globals
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-helpers-build-date-page-text-centre-template-build-engraving-page-text-centre-template-build-page-text-template-build-series-page-text-centre-template-build-watermark-page-text-centre-template
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-structs-stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-album-globals-generator-new-layout
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-album-utils-clean-text
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config-load-effective-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-utils-get-all-series-init-db
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-image-utils-calc-mm-per-pixel-embedded-estimate-real-dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: external/log-error-info-warn
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-connection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs-file
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-bufwriter-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-datelike
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-album-globals-structs-genericattrs
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [json_escape](../../../functions/src/album_globals/generator/json_escape.md)
- [normalize_denom](../../../functions/src/album_globals/generator/normalize_denom.md)
- [normalize_digits](../../../functions/src/album_globals/generator/normalize_digits.md)
- [parse_sort_day](../../../functions/src/album_globals/generator/parse_sort_day.md)
- [stamp_date_key](../../../functions/src/album_globals/generator/stamp_date_key.md)
- [stamp_dedupe_fingerprints](../../../functions/src/album_globals/generator/stamp_dedupe_fingerprints.md)
- [strip_bracket_tags](../../../functions/src/album_globals/generator/strip_bracket_tags.md)
- [normalize_for_text_dedupe](../../../functions/src/album_globals/generator/normalize_for_text_dedupe.md)
- [is_effectively_duplicate_text](../../../functions/src/album_globals/generator/is_effectively_duplicate_text.md)
- [is_real_image_path](../../../functions/src/album_globals/generator/is_real_image_path.md)
- [format_number_with_commas](../../../functions/src/album_globals/generator/format_number_with_commas.md)
- [count_albumeasy_text_lines](../../../functions/src/album_globals/generator/count_albumeasy_text_lines.md)
- [count_nonempty_albumeasy_text_lines](../../../functions/src/album_globals/generator/count_nonempty_albumeasy_text_lines.md)
- [pad_centre_body_for_side_columns](../../../functions/src/album_globals/generator/pad_centre_body_for_side_columns.md)
- [StampText](../../../classes/src/album_globals/generator/StampText.md)
- [build_stamp_text](../../../functions/src/album_globals/generator/build_stamp_text.md)
- [stamp_content_height](../../../functions/src/album_globals/generator/stamp_content_height.md)
- [generate_album_output_conn](../../../functions/src/album_globals/generator/generate_album_output_conn.md)
- [generate_album_output_conn_prepared](../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [generate_album_output](../../../functions/src/album_globals/generator/generate_album_output.md)
- [center_tmp_pages](../../../functions/src/album_globals/generator/center_tmp_pages.md)
- [rich_stamp](../../../functions/src/album_globals/generator/rich_stamp.md)
- [rich_config](../../../functions/src/album_globals/generator/rich_config.md)
- [colnect_height_uses_the_exact_emitted_text_for_every_layout](../../../functions/src/album_globals/generator/colnect_height_uses_the_exact_emitted_text_for_every_layout.md)
- [colnect_measurement_respects_hidden_fields_fonts_and_legacy_flags](../../../functions/src/album_globals/generator/colnect_measurement_respects_hidden_fields_fonts_and_legacy_flags.md)

# Imports

- `super::constants::MONTH_NAMES`
- `super::globals::{compute_usable_height, write_globals}`
- `super::helpers::{
    build_date_page_text_centre_template, build_engraving_page_text_centre_template,
    build_page_text_template, build_series_page_text_centre_template,
    build_watermark_page_text_centre_template,
}`
- `super::structs::Stamp`
- `crate::album_globals::generator_new::layout`
- `crate::album_utils::clean_text`
- `crate::config::load_effective_config`
- `crate::db_utils::{get_all_series, init_db}`
- `crate::image_utils::{calc_mm_per_pixel_embedded, estimate_real_dimensions}`
- `log::{error, info, warn}`
- `rusqlite::Connection`
- `std::fs::File`
- `std::io::{BufWriter, Write}`
- `std::path::Path`
- `chrono::Datelike`
- `std::fs`
- `super::*`
- `crate::album_globals::structs::GenericAttrs`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)