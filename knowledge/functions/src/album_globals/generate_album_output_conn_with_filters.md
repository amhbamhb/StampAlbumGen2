---
type: Rust Function
title: generate_album_output_conn_with_filters
resource: src/album_globals/mod.rs#L335-L365
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/selection/ColnectPdfFilters/is_all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output_conn_with_pdf_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/prepare_pdf_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output_conn_with_prepared_pdf
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/album_colnect_pdf_filters/verify_filtered_output
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn generate_album_output_conn_with_filters( conn: &Connection, album_name: &str, start_year: Option<i32>, end_year: Option<i32>, generate_pdf_override: Option<bool>, filters: &selection::ColnectPdfFilters, ) -> Result<GeneratedAlbumArtifacts, String>`

# Calls

- [is_all](../../../functions/src/album_globals/selection/ColnectPdfFilters/is_all.md)
- [generate_album_output_conn_with_pdf_override](../../../functions/src/album_globals/generate_album_output_conn_with_pdf_override.md)
- [prepare_pdf_stamps](../../../functions/src/album_globals/selection/prepare_pdf_stamps.md)
- [generate_album_output_conn_with_prepared_pdf](../../../functions/src/album_globals/generate_album_output_conn_with_prepared_pdf.md)

# Called by

- [verify_filtered_output](../../../functions/tests/album_colnect_pdf_filters/verify_filtered_output.md)