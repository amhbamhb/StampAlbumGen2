---
type: Rust Function
title: wait_for_generated_pdf
resource: src/album_globals/mod.rs#L68-L79
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/wait_for_generated_pdf_with_timing
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/maybe_generate_album_pdf_with_settings
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn wait_for_generated_pdf( expected_pdf_path: &Path, previous_modified: Option<SystemTime>, timeout: Duration, ) -> bool`

# Calls

- [wait_for_generated_pdf_with_timing](../../../functions/src/album_globals/wait_for_generated_pdf_with_timing.md)

# Called by

- [maybe_generate_album_pdf_with_settings](../../../functions/src/album_globals/maybe_generate_album_pdf_with_settings.md)