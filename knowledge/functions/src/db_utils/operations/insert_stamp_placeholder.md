---
type: Rust Function
title: insert_stamp_placeholder
resource: src/db_utils/operations.rs#L139-L180
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/api_reexports/insert_stamp_placeholder_via_crate_reexport
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_values/make_test_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_values_label/make_test_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_values_suite/make_test_db_multi
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tmp_file_flushed/make_test_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn insert_stamp_placeholder( conn: &Connection, country: &str, series: &str, year: i32, sg_number: &str, ) -> Result<bool>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [collect_stampworld_data_with_fetcher](../../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)
- [insert_stamp_placeholder_via_crate_reexport](../../../../functions/tests/api_reexports/insert_stamp_placeholder_via_crate_reexport.md)
- [make_test_db](../../../../functions/tests/config_values/make_test_db.md)
- [make_test_db](../../../../functions/tests/config_values_label/make_test_db.md)
- [make_test_db_multi](../../../../functions/tests/config_values_suite/make_test_db_multi.md)
- [make_test_db](../../../../functions/tests/tmp_file_flushed/make_test_db.md)