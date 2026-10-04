---
type: Rust Function
title: manual_rows_parse_and_replace_only_the_same_staged_identity
resource: tests/manual_import_template.rs#L65-L154
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/colnect/template/ensure_manual_import_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/parser/parse_csv
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/ingest/ingest_colnect_data_with_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn manual_rows_parse_and_replace_only_the_same_staged_identity()`

# Calls

- [ensure_manual_import_files](../../../functions/src/colnect/template/ensure_manual_import_files.md)
- [parse_csv](../../../functions/src/colnect/parser/parse_csv.md)
- [ingest_colnect_data_with_mode](../../../functions/src/colnect/ingest/ingest_colnect_data_with_mode.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)