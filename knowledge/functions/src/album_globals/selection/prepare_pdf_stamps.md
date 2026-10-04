---
type: Rust Function
title: prepare_pdf_stamps
resource: src/album_globals/selection.rs#L98-L116
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/selection/load_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/ColnectPdfFilters/matches
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generate_album_output_conn_with_filters
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/selection_composes_or_and_years_and_preserves_metadata_precedence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn prepare_pdf_stamps( conn: &Connection, filters: &ColnectPdfFilters, start_year: Option<i32>, end_year: Option<i32>, ) -> Result<PreparedPdfStamps, String>`

# Calls

- [load_stamps](../../../../functions/src/album_globals/selection/load_stamps.md)
- [matches](../../../../functions/src/album_globals/selection/ColnectPdfFilters/matches.md)

# Called by

- [generate_album_output_conn_with_filters](../../../../functions/src/album_globals/generate_album_output_conn_with_filters.md)
- [selection_composes_or_and_years_and_preserves_metadata_precedence](../../../../functions/src/album_globals/selection/selection_composes_or_and_years_and_preserves_metadata_precedence.md)
- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)