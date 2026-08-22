---
type: Rust Function
title: save_left_stamp
resource: src/review_ui/matcher_service.rs#L1169-L1347
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/matcher_service/query_stamp_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/resolve_left_editor_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_left_stamp_by_rowid
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn save_left_stamp( conn: &Connection, db_path: &str, album_name: &str, req: SaveLeftStampRequest, ) -> SaveLeftStampResponse`

# Calls

- [query_stamp_image_path](../../../../functions/src/review_ui/matcher_service/query_stamp_image_path.md)
- [resolve_left_editor_image_path](../../../../functions/src/review_ui/matcher_service/resolve_left_editor_image_path.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [load_left_stamp_by_rowid](../../../../functions/src/review_ui/matcher_service/load_left_stamp_by_rowid.md)

# Called by

- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)