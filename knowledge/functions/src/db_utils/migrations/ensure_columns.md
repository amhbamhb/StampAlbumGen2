---
type: Rust Function
title: ensure_columns
resource: src/db_utils/migrations.rs#L141-L155
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/db_utils/migrations/table_column_name_set
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db_utils/migrations/migration_001_stamps_schema_alignment
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/migration_003_colnect_schema_alignment
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/migration_004_us1847_compat_columns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/enforce_runtime_schema_compatibility
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn ensure_columns(conn: &Connection, table_name: &str, columns: &[(&str, &str)]) -> Result<()>`

# Calls

- [table_column_name_set](../../../../functions/src/db_utils/migrations/table_column_name_set.md)

# Called by

- [migration_001_stamps_schema_alignment](../../../../functions/src/db_utils/migrations/migration_001_stamps_schema_alignment.md)
- [migration_003_colnect_schema_alignment](../../../../functions/src/db_utils/migrations/migration_003_colnect_schema_alignment.md)
- [migration_004_us1847_compat_columns](../../../../functions/src/db_utils/migrations/migration_004_us1847_compat_columns.md)
- [enforce_runtime_schema_compatibility](../../../../functions/src/db_utils/migrations/enforce_runtime_schema_compatibility.md)