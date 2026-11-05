---
type: Rust Method
title: format
resource: src/native_pdf/date.rs#L25-L27
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/date/DateFormat/format_for
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_utils/globals/write_globals
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/ingest/link_colnect_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/create_config_from_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/date/DateSource/parse_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/store_uploaded_image_from_data_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/time_utils/formatted_now
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/serialized_footer_dates_match_frozen_fonts_glyphs_colour_and_geometry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/serialized_dates_and_coexistence_match_frozen_reference_pdfs
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn format(self, date: NaiveDate, separator: &str) -> Result<String>`

# Calls

- [format_for](../../../../../functions/src/native_pdf/date/DateFormat/format_for.md)

# Called by

- [write_globals](../../../../../functions/src/album_utils/globals/write_globals.md)
- [link_colnect_data](../../../../../functions/src/colnect/ingest/link_colnect_data.md)
- [create_config_from_template](../../../../../functions/src/config/create_config_from_template.md)
- [parse_for](../../../../../functions/src/native_pdf/date/DateSource/parse_for.md)
- [store_uploaded_image_from_data_url](../../../../../functions/src/review_ui/store_uploaded_image_from_data_url.md)
- [formatted_now](../../../../../functions/src/time_utils/formatted_now.md)
- [serialized_footer_dates_match_frozen_fonts_glyphs_colour_and_geometry](../../../../../functions/tests/native_pdf_footer_date/serialized_footer_dates_match_frozen_fonts_glyphs_colour_and_geometry.md)
- [serialized_dates_and_coexistence_match_frozen_reference_pdfs](../../../../../functions/tests/native_pdf_header_date/serialized_dates_and_coexistence_match_frozen_reference_pdfs.md)