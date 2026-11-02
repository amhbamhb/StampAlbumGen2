---
type: Rust Function
title: maybe_generate_album_pdf_with_settings
resource: src/album_globals/mod.rs#L105-L265
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/pdf_parser/extraction/metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/albumeasy_diagnostics
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/albumeasy_reported_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/wait_for_generated_pdf
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generated_artifacts_from_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/albumeasy_success_exit_with_render_error_fails_without_pdf_timeout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/albumeasy_captured_output_preserves_successful_pdf_generation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/albumeasy_pdf_timeout_failure_reports_actual_wait
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn maybe_generate_album_pdf_with_settings( output_txt_path: &Path, generate_pdf: bool, cli_path: Option<&str>, pdf_timeout: Duration, ) -> (Option<PathBuf>, Option<String>)`

# Calls

- [metadata](../../../functions/src/review_ui/pdf_parser/extraction/metadata.md)
- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [albumeasy_diagnostics](../../../functions/src/album_globals/albumeasy_diagnostics.md)
- [albumeasy_reported_error](../../../functions/src/album_globals/albumeasy_reported_error.md)
- [wait_for_generated_pdf](../../../functions/src/album_globals/wait_for_generated_pdf.md)

# Called by

- [generated_artifacts_from_path](../../../functions/src/album_globals/generated_artifacts_from_path.md)
- [albumeasy_success_exit_with_render_error_fails_without_pdf_timeout](../../../functions/src/album_globals/albumeasy_success_exit_with_render_error_fails_without_pdf_timeout.md)
- [albumeasy_captured_output_preserves_successful_pdf_generation](../../../functions/src/album_globals/albumeasy_captured_output_preserves_successful_pdf_generation.md)
- [albumeasy_pdf_timeout_failure_reports_actual_wait](../../../functions/src/album_globals/albumeasy_pdf_timeout_failure_reports_actual_wait.md)