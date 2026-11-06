---
type: Rust Function
title: load_left_stamps
resource: src/review_ui/matcher_service.rs#L41-L223
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/resolve_image_abs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/load_stamp_all_fields
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/build_matcher_payload
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn load_left_stamps( conn: &Connection, db_path: &str, album_name: &str, include_imageless: bool, ) -> Result<Vec<LeftStamp>, Box<dyn std::error::Error>>`

# Calls

- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)
- [resolve_image_abs](../../../../functions/src/review_ui/resolve_image_abs.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [load_stamp_all_fields](../../../../functions/src/review_ui/load_stamp_all_fields.md)

# Called by

- [build_matcher_payload](../../../../functions/src/review_ui/build_matcher_payload.md)
- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)