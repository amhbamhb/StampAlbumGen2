---
type: Rust Function
title: maybe_generate_album_pdf_with_settings
resource: src/album_globals/mod.rs#L89-L246
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/wait_for_generated_pdf
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generated_artifacts_from_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/albumeasy_pdf_timeout_failure_reports_actual_wait
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn maybe_generate_album_pdf_with_settings( output_txt_path: &Path, generate_pdf: bool, cli_path: Option<&str>, pdf_timeout: Duration, ) -> (Option<PathBuf>, Option<String>)`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [status](../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status.md)
- [wait_for_generated_pdf](../../../functions/src/album_globals/wait_for_generated_pdf.md)

# Called by

- [generated_artifacts_from_path](../../../functions/src/album_globals/generated_artifacts_from_path.md)
- [albumeasy_pdf_timeout_failure_reports_actual_wait](../../../functions/src/album_globals/albumeasy_pdf_timeout_failure_reports_actual_wait.md)