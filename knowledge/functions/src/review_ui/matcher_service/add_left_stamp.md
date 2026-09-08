---
type: Rust Function
title: add_left_stamp
resource: src/review_ui/matcher_service.rs#L1357-L1518
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/matcher_service/resolve_left_editor_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/normalize_catalog_type
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/load_album_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_default_catalog_code
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

`pub(super) fn add_left_stamp( conn: &Connection, db_path: &str, album_name: &str, req: AddLeftStampRequest, ) -> AddLeftStampResponse`

# Calls

- [resolve_left_editor_image_path](../../../../functions/src/review_ui/matcher_service/resolve_left_editor_image_path.md)
- [normalize_catalog_type](../../../../functions/src/catalog/normalize_catalog_type.md)
- [load_album_config](../../../../functions/src/review_ui/load_album_config.md)
- [effective_default_catalog_code](../../../../functions/src/config/Config/effective_default_catalog_code.md)
- [load_left_stamp_by_rowid](../../../../functions/src/review_ui/matcher_service/load_left_stamp_by_rowid.md)

# Called by

- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)