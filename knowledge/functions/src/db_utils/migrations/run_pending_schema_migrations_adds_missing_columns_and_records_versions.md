---
type: Rust Function
title: run_pending_schema_migrations_adds_missing_columns_and_records_versions
resource: src/db_utils/migrations.rs#L769-L824
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/db_utils/migrations/ensure_schema_migrations_table
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/run_pending_schema_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/column_names
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn run_pending_schema_migrations_adds_missing_columns_and_records_versions()`

# Calls

- [ensure_schema_migrations_table](../../../../functions/src/db_utils/migrations/ensure_schema_migrations_table.md)
- [run_pending_schema_migrations](../../../../functions/src/db_utils/migrations/run_pending_schema_migrations.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [column_names](../../../../functions/src/db_utils/migrations/column_names.md)