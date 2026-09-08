---
type: Rust Function
title: sync_us1847_to_stamps
resource: src/us1847/collector.rs#L1534-L1648
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/db_utils/updates/persist_image_dimensions_from_file
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/us1847/collector/process_1847us_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn sync_us1847_to_stamps( conn: &Connection, series_id: i64, series_title: &str, ) -> Result<(), Box<dyn Error>>`

# Calls

- [persist_image_dimensions_from_file](../../../../functions/src/db_utils/updates/persist_image_dimensions_from_file.md)

# Called by

- [process_1847us_page](../../../../functions/src/us1847/collector/process_1847us_page.md)