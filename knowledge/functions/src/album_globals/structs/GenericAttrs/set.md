---
type: Rust Method
title: set
resource: src/album_globals/structs.rs#L20-L22
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/build/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/rich_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/colnect_measurement_respects_hidden_fields_fonts_and_legacy_flags
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/set_attr_if_non_empty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/set_attr_if_missing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_impl
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/toggleImagelessLeftRows
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/inferInitialCatalogCodeFromLeftRows
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/review_pdf_filters_ui/fixture
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/review_ui_external_album/test/harness
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/review_ui_stampworld_credentials/test/element
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/review_ui_stampworld_credentials/test/harness
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set(&mut self, key: &str, value: &str)`

# Called by

- [main](../../../../../functions/build/main.md)
- [rich_stamp](../../../../../functions/src/album_globals/generator/rich_stamp.md)
- [colnect_measurement_respects_hidden_fields_fonts_and_legacy_flags](../../../../../functions/src/album_globals/generator/colnect_measurement_respects_hidden_fields_fonts_and_legacy_flags.md)
- [set_attr_if_non_empty](../../../../../functions/src/album_globals/generator_new/helpers/set_attr_if_non_empty.md)
- [set_attr_if_missing](../../../../../functions/src/album_globals/generator_new/helpers/set_attr_if_missing.md)
- [fetch_and_filter_stamps_impl](../../../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_impl.md)
- [toggleImagelessLeftRows](../../../../../functions/src/review_ui/views/compare/toggleImagelessLeftRows.md)
- [inferInitialCatalogCodeFromLeftRows](../../../../../functions/src/review_ui/views/compare/inferInitialCatalogCodeFromLeftRows.md)
- [fixture](../../../../../functions/tests/review_pdf_filters_ui/fixture.md)
- [harness](../../../../../functions/tests/review_ui_external_album/test/harness.md)
- [element](../../../../../functions/tests/review_ui_stampworld_credentials/test/element.md)
- [harness](../../../../../functions/tests/review_ui_stampworld_credentials/test/harness.md)