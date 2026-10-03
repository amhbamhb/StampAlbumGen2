---
type: Rust Function
title: generate_album_output_conn_with_prepared_pdf
resource: src/album_globals/mod.rs#L367-L385
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generated_artifacts_from_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generate_album_output_conn_with_filters
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) async fn generate_album_output_conn_with_prepared_pdf( conn: &Connection, album_name: &str, start_year: Option<i32>, end_year: Option<i32>, generate_pdf_override: Option<bool>, prepared: selection::PreparedPdfStamps, ) -> GeneratedAlbumArtifacts`

# Calls

- [generate_album_output_conn_prepared](../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [generated_artifacts_from_path](../../../functions/src/album_globals/generated_artifacts_from_path.md)

# Called by

- [generate_album_output_conn_with_filters](../../../functions/src/album_globals/generate_album_output_conn_with_filters.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)