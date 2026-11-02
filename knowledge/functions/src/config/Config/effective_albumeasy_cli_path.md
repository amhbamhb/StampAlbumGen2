---
type: Rust Method
title: effective_albumeasy_cli_path
resource: src/config.rs#L1581-L1586
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generated_artifacts_from_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/generate_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/renderer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/album_colnect_pdf_filters/verify_filtered_output
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn effective_albumeasy_cli_path(&self) -> Option<&str>`

# Called by

- [generated_artifacts_from_path](../../../../functions/src/album_globals/generated_artifacts_from_path.md)
- [generate_artifacts](../../../../functions/src/external_album/generate_artifacts.md)
- [main](../../../../functions/src/main.md)
- [renderer](../../../../functions/src/review_ui/external_album_api/renderer.md)
- [serve_bootstrap_requests](../../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [verify_filtered_output](../../../../functions/tests/album_colnect_pdf_filters/verify_filtered_output.md)