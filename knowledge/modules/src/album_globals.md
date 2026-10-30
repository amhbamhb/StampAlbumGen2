---
type: Rust Module
title: album_globals
resource: src/album_globals/mod.rs#L1-L635
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/pub-use-constants-month-names
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-globals-compute-usable-height-write-globals
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-helpers-build-page-text-template-emit-row-start
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-page-functions-add-stamp-new-page-new-row-new-series-new-year
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-structs-genericattrs-stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-connection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-process-command
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-atomic-atomicbool-ordering
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration-instant-systemtime
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-wait-for-generated-pdf-with-timing
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration-systemtime
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-os-unix-fs-permissionsext
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [set_pdf_dx_logging_enabled](../../functions/src/album_globals/set_pdf_dx_logging_enabled.md)
- [pdf_dx_logging_enabled](../../functions/src/album_globals/pdf_dx_logging_enabled.md)
- [wait_for_generated_pdf_with_timing](../../functions/src/album_globals/wait_for_generated_pdf_with_timing.md)
- [wait_for_generated_pdf](../../functions/src/album_globals/wait_for_generated_pdf.md)
- [GeneratedAlbumArtifacts](../../classes/src/album_globals/GeneratedAlbumArtifacts.md)
- [albumeasy_diagnostics](../../functions/src/album_globals/albumeasy_diagnostics.md)
- [albumeasy_reported_error](../../functions/src/album_globals/albumeasy_reported_error.md)
- [maybe_generate_album_pdf_with_settings](../../functions/src/album_globals/maybe_generate_album_pdf_with_settings.md)
- [generated_artifacts_from_path](../../functions/src/album_globals/generated_artifacts_from_path.md)
- [imported_artifacts](../../functions/src/album_globals/imported_artifacts.md)
- [generate_album_output_conn](../../functions/src/album_globals/generate_album_output_conn.md)
- [generate_album_output_conn_with_pdf_override](../../functions/src/album_globals/generate_album_output_conn_with_pdf_override.md)
- [generate_album_output_conn_with_filters](../../functions/src/album_globals/generate_album_output_conn_with_filters.md)
- [generate_album_output_conn_with_prepared_pdf](../../functions/src/album_globals/generate_album_output_conn_with_prepared_pdf.md)
- [generate_album_output](../../functions/src/album_globals/generate_album_output.md)
- [albumeasy_errors_are_detected_in_both_output_streams](../../functions/src/album_globals/albumeasy_errors_are_detected_in_both_output_streams.md)
- [albumeasy_success_exit_with_render_error_fails_without_pdf_timeout](../../functions/src/album_globals/albumeasy_success_exit_with_render_error_fails_without_pdf_timeout.md)
- [albumeasy_captured_output_preserves_successful_pdf_generation](../../functions/src/album_globals/albumeasy_captured_output_preserves_successful_pdf_generation.md)
- [wait_for_generated_pdf_rejects_missing_empty_and_stale_files](../../functions/src/album_globals/wait_for_generated_pdf_rejects_missing_empty_and_stale_files.md)
- [albumeasy_pdf_timeout_failure_reports_actual_wait](../../functions/src/album_globals/albumeasy_pdf_timeout_failure_reports_actual_wait.md)
- [wait_for_generated_pdf_detects_delayed_new_write](../../functions/src/album_globals/wait_for_generated_pdf_detects_delayed_new_write.md)

# Imports

- `pub use constants::MONTH_NAMES`
- `pub use globals::{compute_usable_height, write_globals}`
- `pub use helpers::{build_page_text_template, emit_row_start}`
- `pub use page_functions::{add_stamp, new_page, new_row, new_series, new_year}`
- `pub use structs::{GenericAttrs, Stamp}`
- `rusqlite::Connection`
- `std::path::{Path, PathBuf}`
- `std::process::Command`
- `std::sync::atomic::{AtomicBool, Ordering}`
- `std::time::{Duration, Instant, SystemTime}`
- `super::wait_for_generated_pdf_with_timing`
- `std::fs`
- `std::time::{Duration, SystemTime}`
- `std::os::unix::fs::PermissionsExt`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)