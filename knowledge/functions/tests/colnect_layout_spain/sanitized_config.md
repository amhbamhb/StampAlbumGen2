---
type: Rust Function
title: sanitized_config
resource: tests/colnect_layout_spain.rs#L27-L51
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/colnect_layout_spain/scrub
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/colnect_layout_spain/spain_colnect_layout_packs_rich_rows_without_losing_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn sanitized_config(path: &Path) -> String`

# Calls

- [scrub](../../../functions/tests/colnect_layout_spain/scrub.md)

# Called by

- [spain_colnect_layout_packs_rich_rows_without_losing_stamps](../../../functions/tests/colnect_layout_spain/spain_colnect_layout_packs_rich_rows_without_losing_stamps.md)