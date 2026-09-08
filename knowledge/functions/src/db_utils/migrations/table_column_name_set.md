---
type: Rust Function
title: table_column_name_set
resource: src/db_utils/migrations.rs#L137-L139
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/db_utils/migrations/table_column_names
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db_utils/migrations/ensure_columns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/migration_001_stamps_schema_alignment
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn table_column_name_set(conn: &Connection, table_name: &str) -> Result<HashSet<String>>`

# Calls

- [table_column_names](../../../../functions/src/db_utils/migrations/table_column_names.md)

# Called by

- [ensure_columns](../../../../functions/src/db_utils/migrations/ensure_columns.md)
- [migration_001_stamps_schema_alignment](../../../../functions/src/db_utils/migrations/migration_001_stamps_schema_alignment.md)