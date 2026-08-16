---
type: Rust Function
title: init_db
resource: src/db_utils/init.rs#L13-L263
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/ensure_schema_migrations_table
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/run_pending_schema_migrations
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn init_db(db_path: &str) -> Result<Connection>`

# Calls

- [from](../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [ensure_schema_migrations_table](../../../../functions/src/db_utils/migrations/ensure_schema_migrations_table.md)
- [run_pending_schema_migrations](../../../../functions/src/db_utils/migrations/run_pending_schema_migrations.md)