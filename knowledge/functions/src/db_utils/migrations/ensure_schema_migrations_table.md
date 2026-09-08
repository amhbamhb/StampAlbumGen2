---
type: Rust Function
title: ensure_schema_migrations_table
resource: src/db_utils/migrations.rs#L210-L220
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/db_utils/init/init_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/run_pending_schema_migrations
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

`pub fn ensure_schema_migrations_table(conn: &Connection) -> Result<()>`

# Called by

- [init_db](../../../../functions/src/db_utils/init/init_db.md)
- [run_pending_schema_migrations](../../../../functions/src/db_utils/migrations/run_pending_schema_migrations.md)
- [run_pending_schema_migrations_adds_missing_columns_and_records_versions](../../../../functions/src/db_utils/migrations/run_pending_schema_migrations_adds_missing_columns_and_records_versions.md)
- [run_pending_schema_migrations_rebuilds_unexpected_stamps_schema](../../../../functions/src/db_utils/migrations/run_pending_schema_migrations_rebuilds_unexpected_stamps_schema.md)
- [run_pending_schema_migrations_repairs_preapplied_but_drifted_schema](../../../../functions/src/db_utils/migrations/run_pending_schema_migrations_repairs_preapplied_but_drifted_schema.md)