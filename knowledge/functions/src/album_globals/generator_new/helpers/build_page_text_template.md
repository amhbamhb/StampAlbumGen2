---
type: Rust Function
title: build_page_text_template
resource: src/album_globals/generator_new/helpers.rs#L91-L136
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/build_series_page_text_centre_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/build_scoped_page_text_centre_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/page_functions/new_year
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn build_page_text_template(cfg: &Config, kind: &str) -> String`

# Called by

- [generate_album_output_conn_prepared](../../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [build_series_page_text_centre_template](../../../../../functions/src/album_globals/generator_new/helpers/build_series_page_text_centre_template.md)
- [build_scoped_page_text_centre_template](../../../../../functions/src/album_globals/generator_new/helpers/build_scoped_page_text_centre_template.md)
- [new_year](../../../../../functions/src/album_globals/page_functions/new_year.md)