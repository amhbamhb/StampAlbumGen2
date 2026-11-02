---
type: Rust Function
title: verify_source
resource: src/external_album.rs#L1018-L1049
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/relative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/no_links
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/extraction/metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/digest
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/render_pair
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn verify_source(conn: &Connection, source: &Path) -> Result<()>`

# Calls

- [err](../../../functions/src/external_album/err.md)
- [relative](../../../functions/src/external_album/relative.md)
- [no_links](../../../functions/src/external_album/no_links.md)
- [metadata](../../../functions/src/review_ui/pdf_parser/extraction/metadata.md)
- [digest](../../../functions/src/external_album/digest.md)

# Called by

- [render_pair](../../../functions/src/external_album/render_pair.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)
- [export_layout](../../../functions/src/external_album/export_layout.md)