---
type: Rust Function
title: albumeasy_diagnostics
resource: src/album_globals/mod.rs#L89-L96
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/maybe_generate_album_pdf_with_settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/albumeasy_errors_are_detected_in_both_output_streams
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn albumeasy_diagnostics(stdout: &[u8], stderr: &[u8]) -> String`

# Called by

- [maybe_generate_album_pdf_with_settings](../../../functions/src/album_globals/maybe_generate_album_pdf_with_settings.md)
- [albumeasy_errors_are_detected_in_both_output_streams](../../../functions/src/album_globals/albumeasy_errors_are_detected_in_both_output_streams.md)