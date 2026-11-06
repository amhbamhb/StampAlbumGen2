---
type: Rust Function
title: load_right_candidate_by_colnect_id
resource: src/review_ui/matcher_service.rs#L701-L850
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
  - target: functions/src/colnect/image_fetcher/resolve_colnect_image_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/resolve_image_abs
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/matcher_service/apply_matches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn load_right_candidate_by_colnect_id( conn: &Connection, db_path: &str, album_name: &str, colnect_id: &str, ) -> Result<Option<RightCandidate>, Box<dyn std::error::Error>>`

# Calls

- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [resolve_colnect_image_url](../../../../functions/src/colnect/image_fetcher/resolve_colnect_image_url.md)
- [resolve_image_abs](../../../../functions/src/review_ui/resolve_image_abs.md)

# Called by

- [apply_matches](../../../../functions/src/review_ui/matcher_service/apply_matches.md)
- [add_colnect_stamp_to_stamps](../../../../functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps.md)