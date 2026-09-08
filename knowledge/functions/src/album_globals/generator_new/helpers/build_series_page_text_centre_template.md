---
type: Rust Function
title: build_series_page_text_centre_template
resource: src/album_globals/generator_new/helpers.rs#L136-L168
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/helpers/build_page_text_template
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn build_series_page_text_centre_template(cfg: &Config) -> String`

# Calls

- [build_page_text_template](../../../../../functions/src/album_globals/generator_new/helpers/build_page_text_template.md)

# Called by

- [generate_album_output_conn](../../../../../functions/src/album_globals/generator/generate_album_output_conn.md)