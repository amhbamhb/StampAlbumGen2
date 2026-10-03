---
type: Rust Function
title: insert_fixture_stamp
resource: tests/album_year_filter.rs#L18-L60
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

`fn insert_fixture_stamp(conn: &Connection, stamp: &FixtureStamp)`

# Called by

- [populate_test_db](../../../functions/tests/album_year_filter/populate_test_db.md)