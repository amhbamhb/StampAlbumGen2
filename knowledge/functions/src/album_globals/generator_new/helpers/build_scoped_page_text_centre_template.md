---
type: Rust Function
title: build_scoped_page_text_centre_template
resource: src/album_globals/generator_new/helpers.rs#L170-L200
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/helpers/build_page_text_template
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/helpers/build_date_page_text_centre_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/build_watermark_page_text_centre_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/build_engraving_page_text_centre_template
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_scoped_page_text_centre_template( cfg: &Config, format_enabled: bool, format_opt: Option<String>, font_opt: Option<String>, size_opt: Option<f32>, vspace_opt: Option<f32>, ) -> String`

# Calls

- [build_page_text_template](../../../../../functions/src/album_globals/generator_new/helpers/build_page_text_template.md)

# Called by

- [build_date_page_text_centre_template](../../../../../functions/src/album_globals/generator_new/helpers/build_date_page_text_centre_template.md)
- [build_watermark_page_text_centre_template](../../../../../functions/src/album_globals/generator_new/helpers/build_watermark_page_text_centre_template.md)
- [build_engraving_page_text_centre_template](../../../../../functions/src/album_globals/generator_new/helpers/build_engraving_page_text_centre_template.md)