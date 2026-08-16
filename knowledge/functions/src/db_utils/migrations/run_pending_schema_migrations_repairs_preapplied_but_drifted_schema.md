---
type: Rust Function
title: run_pending_schema_migrations_repairs_preapplied_but_drifted_schema
resource: src/db_utils/migrations.rs#L631-L681
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
  - target: functions/src/db_utils/migrations/column_names
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn run_pending_schema_migrations_repairs_preapplied_but_drifted_schema()`

# Calls

- [ensure_schema_migrations_table](../../../../functions/src/db_utils/migrations/ensure_schema_migrations_table.md)
- [run_pending_schema_migrations](../../../../functions/src/db_utils/migrations/run_pending_schema_migrations.md)
- [column_names](../../../../functions/src/db_utils/migrations/column_names.md)