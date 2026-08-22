---
type: Rust Function
title: table_column_names
resource: src/db_utils/migrations.rs#L123-L132
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/db_utils/migrations/table_column_name_set
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn table_column_names(conn: &Connection, table_name: &str) -> Result<Vec<String>>`

# Called by

- [table_column_name_set](../../../../functions/src/db_utils/migrations/table_column_name_set.md)