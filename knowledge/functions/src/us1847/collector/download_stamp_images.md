---
type: Rust Function
title: download_stamp_images
resource: src/us1847/collector.rs#L1446-L1532
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/updates/persist_image_dimensions_from_file
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/us1847/collector/process_1847us_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn download_stamp_images( client: &Client, conn: &Connection, series_id: i64, image_path: &str, overwrite_images: bool, base_url: &str, ) -> Result<(), Box<dyn Error>>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [persist_image_dimensions_from_file](../../../../functions/src/db_utils/updates/persist_image_dimensions_from_file.md)

# Called by

- [process_1847us_page](../../../../functions/src/us1847/collector/process_1847us_page.md)