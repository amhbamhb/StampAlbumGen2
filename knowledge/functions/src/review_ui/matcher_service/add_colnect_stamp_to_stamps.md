---
type: Rust Function
title: add_colnect_stamp_to_stamps
resource: src/review_ui/matcher_service.rs#L852-L1102
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/matcher_service/load_right_candidate_by_colnect_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/store_uploaded_image_from_data_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/load_album_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_default_catalog_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/normalize_catalog_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/infer_catalog_type_for_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_left_stamp_by_rowid
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/mark_colnect_candidate_processed
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn add_colnect_stamp_to_stamps( conn: &Connection, db_path: &str, album_name: &str, req: AddColnectStampRequest, ) -> AddColnectStampResponse`

# Calls

- [load_right_candidate_by_colnect_id](../../../../functions/src/review_ui/matcher_service/load_right_candidate_by_colnect_id.md)
- [store_uploaded_image_from_data_url](../../../../functions/src/review_ui/store_uploaded_image_from_data_url.md)
- [load_album_config](../../../../functions/src/review_ui/load_album_config.md)
- [effective_default_catalog_code](../../../../functions/src/config/Config/effective_default_catalog_code.md)
- [normalize_catalog_number](../../../../functions/src/catalog/normalize_catalog_number.md)
- [infer_catalog_type_for_number](../../../../functions/src/catalog/infer_catalog_type_for_number.md)
- [load_left_stamp_by_rowid](../../../../functions/src/review_ui/matcher_service/load_left_stamp_by_rowid.md)
- [mark_colnect_candidate_processed](../../../../functions/src/review_ui/matcher_service/mark_colnect_candidate_processed.md)

# Called by

- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)