---
type: Rust Function
title: load_left_stamp_by_rowid
resource: src/review_ui/matcher_service.rs#L582-L699
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
  - target: functions/src/review_ui/load_stamp_all_fields
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/save_left_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/add_left_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/clear_left_colnect_link
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn load_left_stamp_by_rowid( conn: &Connection, db_path: &str, album_name: &str, rowid: i64, ) -> Result<Option<LeftStamp>, Box<dyn std::error::Error>>`

# Calls

- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [resolve_image_abs](../../../../functions/src/review_ui/resolve_image_abs.md)
- [load_stamp_all_fields](../../../../functions/src/review_ui/load_stamp_all_fields.md)

# Called by

- [add_colnect_stamp_to_stamps](../../../../functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps.md)
- [save_left_stamp](../../../../functions/src/review_ui/matcher_service/save_left_stamp.md)
- [add_left_stamp](../../../../functions/src/review_ui/matcher_service/add_left_stamp.md)
- [clear_left_colnect_link](../../../../functions/src/review_ui/matcher_service/clear_left_colnect_link.md)