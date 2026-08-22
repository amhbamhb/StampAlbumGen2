---
type: Rust Function
title: has_downloaded_images
resource: src/modes.rs#L107-L132
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/modes/run_database_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn has_downloaded_images(images_dir: &std::path::Path) -> bool`

# Called by

- [run_database_mode](../../../functions/src/modes/run_database_mode.md)