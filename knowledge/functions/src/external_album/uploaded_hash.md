---
type: Rust Function
title: uploaded_hash
resource: src/external_album.rs#L336-L356
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/layout_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/uploaded_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/no_links
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/portable_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/digest
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/render_pair
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_uploaded
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn uploaded_hash(album: &Path, pages: &[LayoutPage]) -> Result<String>`

# Calls

- [layout_hash](../../../functions/src/external_album/layout_hash.md)
- [json](../../../functions/src/external_album/json.md)
- [uploaded_image](../../../functions/src/external_album/uploaded_image.md)
- [no_links](../../../functions/src/external_album/no_links.md)
- [portable_path](../../../functions/src/external_album/portable_path.md)
- [digest](../../../functions/src/external_album/digest.md)

# Called by

- [render_pair](../../../functions/src/external_album/render_pair.md)
- [verify_uploaded](../../../functions/src/external_album/verify_uploaded.md)