---
type: Rust Function
title: run_pending_schema_migrations
resource: src/db_utils/migrations.rs#L219-L270
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/db_utils/migrations/ensure_schema_migrations_table
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/enforce_runtime_schema_compatibility
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db_utils/init/init_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/run_pending_schema_migrations_adds_missing_columns_and_records_versions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/run_pending_schema_migrations_rebuilds_unexpected_stamps_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/run_pending_schema_migrations_repairs_preapplied_but_drifted_schema
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run_pending_schema_migrations(conn: &Connection) -> Result<()>`

# Calls

- [ensure_schema_migrations_table](../../../../functions/src/db_utils/migrations/ensure_schema_migrations_table.md)
- [enforce_runtime_schema_compatibility](../../../../functions/src/db_utils/migrations/enforce_runtime_schema_compatibility.md)

# Called by

- [init_db](../../../../functions/src/db_utils/init/init_db.md)
- [run_pending_schema_migrations_adds_missing_columns_and_records_versions](../../../../functions/src/db_utils/migrations/run_pending_schema_migrations_adds_missing_columns_and_records_versions.md)
- [run_pending_schema_migrations_rebuilds_unexpected_stamps_schema](../../../../functions/src/db_utils/migrations/run_pending_schema_migrations_rebuilds_unexpected_stamps_schema.md)
- [run_pending_schema_migrations_repairs_preapplied_but_drifted_schema](../../../../functions/src/db_utils/migrations/run_pending_schema_migrations_repairs_preapplied_but_drifted_schema.md)