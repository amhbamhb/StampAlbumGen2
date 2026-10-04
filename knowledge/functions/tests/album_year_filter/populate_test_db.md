---
type: Rust Function
title: populate_test_db
resource: tests/album_year_filter.rs#L63-L109
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/album_year_filter/insert_fixture_stamp
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/album_year_filter/test_generate_album_filters_by_year_exact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/album_year_filter/test_generate_album_filters_by_year_open_ended
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn populate_test_db() -> Connection`

# Calls

- [insert_fixture_stamp](../../../functions/tests/album_year_filter/insert_fixture_stamp.md)

# Called by

- [test_generate_album_filters_by_year_exact](../../../functions/tests/album_year_filter/test_generate_album_filters_by_year_exact.md)
- [test_generate_album_filters_by_year_open_ended](../../../functions/tests/album_year_filter/test_generate_album_filters_by_year_open_ended.md)