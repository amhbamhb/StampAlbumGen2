---
type: Rust Function
title: layout_hash
resource: src/external_album.rs#L1172-L1177
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/digest
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/uploaded_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render_pair
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn layout_hash(layout: &str) -> String`

# Calls

- [digest](../../../functions/src/external_album/digest.md)

# Called by

- [uploaded_hash](../../../functions/src/external_album/uploaded_hash.md)
- [render_pair](../../../functions/src/external_album/render_pair.md)
- [verify_layout](../../../functions/src/external_album/verify_layout.md)