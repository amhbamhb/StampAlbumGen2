---
type: Rust Function
title: column_names
resource: src/db_utils/migrations.rs#L759-L766
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/db_utils/migrations/run_pending_schema_migrations_adds_missing_columns_and_records_versions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/run_pending_schema_migrations_rebuilds_unexpected_stamps_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/run_pending_schema_migrations_repairs_preapplied_but_drifted_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/load_stamp_all_fields
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn column_names(conn: &Connection, table_name: &str) -> Vec<String>`

# Called by

- [run_pending_schema_migrations_adds_missing_columns_and_records_versions](../../../../functions/src/db_utils/migrations/run_pending_schema_migrations_adds_missing_columns_and_records_versions.md)
- [run_pending_schema_migrations_rebuilds_unexpected_stamps_schema](../../../../functions/src/db_utils/migrations/run_pending_schema_migrations_rebuilds_unexpected_stamps_schema.md)
- [run_pending_schema_migrations_repairs_preapplied_but_drifted_schema](../../../../functions/src/db_utils/migrations/run_pending_schema_migrations_repairs_preapplied_but_drifted_schema.md)
- [load_stamp_all_fields](../../../../functions/src/review_ui/load_stamp_all_fields.md)