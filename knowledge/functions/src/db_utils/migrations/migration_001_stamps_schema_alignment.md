---
type: Rust Function
title: migration_001_stamps_schema_alignment
resource: src/db_utils/migrations.rs#L275-L293
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/db_utils/migrations/ensure_columns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/table_column_name_set
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn migration_001_stamps_schema_alignment(conn: &Connection) -> Result<()>`

# Calls

- [ensure_columns](../../../../functions/src/db_utils/migrations/ensure_columns.md)
- [table_column_name_set](../../../../functions/src/db_utils/migrations/table_column_name_set.md)