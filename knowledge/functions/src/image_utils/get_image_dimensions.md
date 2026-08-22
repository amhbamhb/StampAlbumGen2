---
type: Rust Function
title: get_image_dimensions
resource: src/image_utils.rs#L116-L124
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_utils/dimensions/calculate_stamp_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/backfill_image_dims/resolve_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/updates/persist_image_dimensions_from_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_utils/estimate_real_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/backfill_image_dims/backfill_image_dimensions_persists_to_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_image_dimensions<P: AsRef<Path>>(path: P) -> Option<(u32, u32)>`

# Called by

- [calculate_stamp_dimensions](../../../functions/src/album_utils/dimensions/calculate_stamp_dimensions.md)
- [resolve_dimensions](../../../functions/src/bin/backfill_image_dims/resolve_dimensions.md)
- [persist_image_dimensions_from_file](../../../functions/src/db_utils/updates/persist_image_dimensions_from_file.md)
- [estimate_real_dimensions](../../../functions/src/image_utils/estimate_real_dimensions.md)
- [collect_stampworld_data_with_fetcher](../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)
- [backfill_image_dimensions_persists_to_db](../../../functions/tests/backfill_image_dims/backfill_image_dimensions_persists_to_db.md)