---
type: Rust Function
title: country_slug
resource: src/modes.rs#L156-L158
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/modes/canonical_album_dir_name
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn country_slug(s: &str) -> String`

# Calls

- [canonical_album_dir_name](../../../functions/src/modes/canonical_album_dir_name.md)

# Called by

- [run_colnect_inbox_mode](../../../functions/src/modes/run_colnect_inbox_mode.md)