---
type: Rust Function
title: migration_004_us1847_compat_columns
resource: src/db_utils/migrations.rs#L343-L358
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

`fn migration_004_us1847_compat_columns(conn: &Connection) -> Result<()>`

# Calls

- [ensure_columns](../../../../functions/src/db_utils/migrations/ensure_columns.md)