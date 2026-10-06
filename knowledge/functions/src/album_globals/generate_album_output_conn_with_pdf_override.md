---
type: Rust Function
title: generate_album_output_conn_with_pdf_override
resource: src/album_globals/mod.rs#L334-L372
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/imported_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generated_artifacts_from_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output_conn_with_filters
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_database_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/album_colnect_pdf_filters/verify_filtered_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/generator_dispatch_uses_saved_layout_and_draft_never_falls_back
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn generate_album_output_conn_with_pdf_override<F>( conn: &Connection, album_name: &str, page_callback: Option<F>, start_year: Option<i32>, end_year: Option<i32>, generate_pdf_override: Option<bool>, ) -> GeneratedAlbumArtifacts where F: FnMut(usize),`

# Calls

- [imported_artifacts](../../../functions/src/album_globals/imported_artifacts.md)
- [generated_artifacts_from_path](../../../functions/src/album_globals/generated_artifacts_from_path.md)

# Called by

- [generate_album_output_conn](../../../functions/src/album_globals/generate_album_output_conn.md)
- [generate_album_output_conn_with_filters](../../../functions/src/album_globals/generate_album_output_conn_with_filters.md)
- [run_colnect_inbox_mode](../../../functions/src/modes/run_colnect_inbox_mode.md)
- [run_database_mode](../../../functions/src/modes/run_database_mode.md)
- [run_scraping_mode](../../../functions/src/modes/run_scraping_mode.md)
- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)
- [verify_filtered_output](../../../functions/tests/album_colnect_pdf_filters/verify_filtered_output.md)
- [generator_dispatch_uses_saved_layout_and_draft_never_falls_back](../../../functions/tests/external_album_backend/generator_dispatch_uses_saved_layout_and_draft_never_falls_back.md)