---
type: Rust Function
title: build_date_page_text_centre_template
resource: src/album_globals/generator_new/helpers.rs#L204-L213
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/helpers/build_scoped_page_text_centre_template
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn build_date_page_text_centre_template(cfg: &Config) -> String`

# Calls

- [build_scoped_page_text_centre_template](../../../../../functions/src/album_globals/generator_new/helpers/build_scoped_page_text_centre_template.md)

# Called by

- [generate_album_output_conn_prepared](../../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)