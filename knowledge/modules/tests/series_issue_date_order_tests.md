---
type: Rust Module
title: series_issue_date_order_tests
resource: tests/series_issue_date_order_tests.rs#L1-L181
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/serial-test-serial
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-album-globals-generate-album-output-conn
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-db-utils
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-file-utils
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-env
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [CwdRestore](../../classes/tests/series_issue_date_order_tests/CwdRestore.md)
- [new](../../functions/tests/series_issue_date_order_tests/CwdRestore/new.md)
- [drop](../../functions/tests/series_issue_date_order_tests/CwdRestore/drop/drop.md)
- [write_config](../../functions/tests/series_issue_date_order_tests/write_config.md)
- [insert_stamp](../../functions/tests/series_issue_date_order_tests/insert_stamp.md)
- [read_stamp_diag_order](../../functions/tests/series_issue_date_order_tests/read_stamp_diag_order.md)
- [find_generated_album_txt](../../functions/tests/series_issue_date_order_tests/find_generated_album_txt.md)
- [test_series_sort_uses_calendar_month_then_day](../../functions/tests/series_issue_date_order_tests/test_series_sort_uses_calendar_month_then_day.md)
- [test_series_order_uses_earliest_issue_date](../../functions/tests/series_issue_date_order_tests/test_series_order_uses_earliest_issue_date.md)

# Imports

- `serial_test::serial`
- `stamp_album_gen2::album_globals::generate_album_output_conn`
- `stamp_album_gen2::db_utils`
- `stamp_album_gen2::file_utils`
- `std::env`
- `std::fs`
- `tempfile::tempdir`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)