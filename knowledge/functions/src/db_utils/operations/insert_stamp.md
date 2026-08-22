---
type: Rust Function
title: insert_stamp
resource: src/db_utils/operations.rs#L93-L507
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/db_utils/operations/is_stampworld_image_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/operations/is_colnect_page_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/operations/is_colnect_image_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/operations/is_real_image_candidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/operations/cleanup_stampworld_identity_duplicates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/operations/is_placeholder_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn insert_stamp( conn: &Connection, country: &str, series: &str, year: i32, month: &str, _month_num: &str, day: &str, sg_number: &str, type_: &str, denomination: &str, color: &str, description: &str, stamps_issued: &str, mint_condition: &str, unused: &str, used: &str, letter_fdc: &str, currency: &str, image_path: &str, remote_image_url: &str, original_image_url: &str, watermark: &str, design: &str, engraving: &str, perforation: &str, context: &str, ) -> Result<bool>`

# Calls

- [is_stampworld_image_url](../../../../functions/src/db_utils/operations/is_stampworld_image_url.md)
- [is_colnect_page_url](../../../../functions/src/db_utils/operations/is_colnect_page_url.md)
- [is_colnect_image_url](../../../../functions/src/db_utils/operations/is_colnect_image_url.md)
- [is_real_image_candidate](../../../../functions/src/db_utils/operations/is_real_image_candidate.md)
- [cleanup_stampworld_identity_duplicates](../../../../functions/src/db_utils/operations/cleanup_stampworld_identity_duplicates.md)
- [is_placeholder_image_path](../../../../functions/src/db_utils/operations/is_placeholder_image_path.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)