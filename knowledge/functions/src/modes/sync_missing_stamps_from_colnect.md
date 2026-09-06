---
type: Rust Function
title: sync_missing_stamps_from_colnect
resource: src/modes.rs#L560-L1215
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/modes/remap_colnect_sg_to_existing_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/backfill_colnect_links_on_existing_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_default_catalog_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/infer_and_fix_catalog_codes_for_colnect_rows
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn sync_missing_stamps_from_colnect( conn: &rusqlite::Connection, ) -> Result<usize, Box<dyn std::error::Error>>`

# Calls

- [remap_colnect_sg_to_existing_stamps](../../../functions/src/modes/remap_colnect_sg_to_existing_stamps.md)
- [backfill_colnect_links_on_existing_stamps](../../../functions/src/modes/backfill_colnect_links_on_existing_stamps.md)
- [load_config](../../../functions/src/config/load_config.md)
- [effective_default_catalog_code](../../../functions/src/config/Config/effective_default_catalog_code.md)
- [infer_and_fix_catalog_codes_for_colnect_rows](../../../functions/src/modes/infer_and_fix_catalog_codes_for_colnect_rows.md)