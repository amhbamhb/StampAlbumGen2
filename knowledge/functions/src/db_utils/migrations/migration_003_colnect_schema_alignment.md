---
type: Rust Function
title: migration_003_colnect_schema_alignment
resource: src/db_utils/migrations.rs#L342-L344
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/db_utils/migrations/ensure_columns
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn migration_003_colnect_schema_alignment(conn: &Connection) -> Result<()>`

# Calls

- [ensure_columns](../../../../functions/src/db_utils/migrations/ensure_columns.md)