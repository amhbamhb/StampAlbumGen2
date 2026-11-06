---
type: Rust Function
title: load_right_candidates
resource: src/review_ui/matcher_service.rs#L225-L378
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/resolve_image_abs
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

`pub(super) fn load_right_candidates( conn: &Connection, db_path: &str, album_name: &str, ) -> Result<Vec<RightCandidate>, Box<dyn std::error::Error>>`

# Calls

- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [resolve_image_abs](../../../../functions/src/review_ui/resolve_image_abs.md)

# Called by

- [build_matcher_payload](../../../../functions/src/review_ui/build_matcher_payload.md)
- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)