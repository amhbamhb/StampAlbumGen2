---
type: Rust Function
title: wait_for_generated_pdf_with_timing
resource: src/album_globals/mod.rs#L40-L66
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/wait_for_generated_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/wait_for_generated_pdf_detects_delayed_new_write
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn wait_for_generated_pdf_with_timing( expected_pdf_path: &Path, previous_modified: Option<SystemTime>, timeout: Duration, poll_every: Duration, ) -> bool`

# Called by

- [wait_for_generated_pdf](../../../functions/src/album_globals/wait_for_generated_pdf.md)
- [wait_for_generated_pdf_detects_delayed_new_write](../../../functions/src/album_globals/wait_for_generated_pdf_detects_delayed_new_write.md)