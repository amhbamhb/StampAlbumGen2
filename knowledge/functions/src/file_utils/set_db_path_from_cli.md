---
type: Rust Function
title: set_db_path_from_cli
resource: src/file_utils.rs#L43-L45
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/engraving_column/test_engraving_column_emitted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/generate_conn_in_memory/test_generate_conn_in_memory_clean
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/header_date/test_series_header_includes_date
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/perforation_header/test_perforation_column_emitted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/perforation_header_negative/test_perforation_column_absent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/phase1_generate_conn/phase1_generate_with_in_memory_conn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set_db_path_from_cli(path: &str)`

# Called by

- [run_scraping_mode](../../../functions/src/modes/run_scraping_mode.md)
- [test_engraving_column_emitted](../../../functions/tests/engraving_column/test_engraving_column_emitted.md)
- [test_generate_conn_in_memory_clean](../../../functions/tests/generate_conn_in_memory/test_generate_conn_in_memory_clean.md)
- [test_series_header_includes_date](../../../functions/tests/header_date/test_series_header_includes_date.md)
- [test_perforation_column_emitted](../../../functions/tests/perforation_header/test_perforation_column_emitted.md)
- [test_perforation_column_absent](../../../functions/tests/perforation_header_negative/test_perforation_column_absent.md)
- [phase1_generate_with_in_memory_conn](../../../functions/tests/phase1_generate_conn/phase1_generate_with_in_memory_conn.md)