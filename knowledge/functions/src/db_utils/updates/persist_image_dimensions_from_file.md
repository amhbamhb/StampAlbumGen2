---
type: Rust Function
title: persist_image_dimensions_from_file
resource: src/db_utils/updates.rs#L24-L42
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/image_utils/get_image_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/updates/update_image_dimensions
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/download_stamp_images
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/sync_us1847_to_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn persist_image_dimensions_from_file( conn: &Connection, country: &str, series: &str, sg_number: &str, local_image_path: &str, ) -> Result<bool>`

# Calls

- [get_image_dimensions](../../../../functions/src/image_utils/get_image_dimensions.md)
- [update_image_dimensions](../../../../functions/src/db_utils/updates/update_image_dimensions.md)

# Called by

- [collect_stampworld_data_with_fetcher](../../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)
- [download_stamp_images](../../../../functions/src/us1847/collector/download_stamp_images.md)
- [sync_us1847_to_stamps](../../../../functions/src/us1847/collector/sync_us1847_to_stamps.md)