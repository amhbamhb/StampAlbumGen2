---
type: Rust Function
title: reflow
resource: src/external_album/reflow.rs#L11-L128
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/reflow/setting
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/count
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn reflow(globals: &[LayoutCommand], page: LayoutPage) -> Result<LayoutPage>`

# Calls

- [setting](../../../../functions/src/external_album/reflow/setting.md)
- [count](../../../../functions/src/album_globals/selection/count.md)

# Called by

- [edit_page](../../../../functions/src/external_album/edit_page.md)