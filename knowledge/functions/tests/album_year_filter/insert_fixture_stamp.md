---
type: Rust Function
title: insert_fixture_stamp
resource: tests/album_year_filter.rs#L5-L47
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/album_year_filter/populate_test_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn insert_fixture_stamp( conn: &Connection, year: i32, month: &str, month_num: i32, day: &str, sg_number: &str, denomination: &str, color: &str, description: &str, image_path: &str, )`

# Called by

- [populate_test_db](../../../functions/tests/album_year_filter/populate_test_db.md)