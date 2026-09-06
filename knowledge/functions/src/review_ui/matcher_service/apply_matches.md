---
type: Rust Function
title: apply_matches
resource: src/review_ui/matcher_service.rs#L380-L580
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/load_album_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_default_catalog_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_right_candidate_by_colnect_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/normalize_catalog_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/infer_catalog_type_for_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
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

`pub(super) fn apply_matches( conn: &Connection, db_path: &str, album_name: &str, matches: Vec<SelectedMatch>, fields: FieldSelection, right_map: &HashMap<String, RightCandidate>, ) -> ApplyMatchesResponse`

# Calls

- [load_album_config](../../../../functions/src/review_ui/load_album_config.md)
- [effective_default_catalog_code](../../../../functions/src/config/Config/effective_default_catalog_code.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [load_right_candidate_by_colnect_id](../../../../functions/src/review_ui/matcher_service/load_right_candidate_by_colnect_id.md)
- [normalize_catalog_number](../../../../functions/src/catalog/normalize_catalog_number.md)
- [infer_catalog_type_for_number](../../../../functions/src/catalog/infer_catalog_type_for_number.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [mark_colnect_candidate_processed](../../../../functions/src/review_ui/matcher_service/mark_colnect_candidate_processed.md)

# Called by

- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)