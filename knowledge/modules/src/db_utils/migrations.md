---
type: Rust Module
title: migrations
resource: src/db_utils/migrations.rs#L1-L918
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/log-debug-error
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-params-connection-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashset
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-ensure-schema-migrations-table-run-pending-schema-migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-connection
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [table_column_names](../../../functions/src/db_utils/migrations/table_column_names.md)
- [table_column_name_set](../../../functions/src/db_utils/migrations/table_column_name_set.md)
- [ensure_columns](../../../functions/src/db_utils/migrations/ensure_columns.md)
- [cleanup_legacy_stampworld_identity_duplicates](../../../functions/src/db_utils/migrations/cleanup_legacy_stampworld_identity_duplicates.md)
- [ensure_schema_migrations_table](../../../functions/src/db_utils/migrations/ensure_schema_migrations_table.md)
- [run_pending_schema_migrations](../../../functions/src/db_utils/migrations/run_pending_schema_migrations.md)
- [migration_001_stamps_schema_alignment](../../../functions/src/db_utils/migrations/migration_001_stamps_schema_alignment.md)
- [migration_002_stamps_uniq_key_and_indexes](../../../functions/src/db_utils/migrations/migration_002_stamps_uniq_key_and_indexes.md)
- [migration_003_colnect_schema_alignment](../../../functions/src/db_utils/migrations/migration_003_colnect_schema_alignment.md)
- [migration_004_us1847_compat_columns](../../../functions/src/db_utils/migrations/migration_004_us1847_compat_columns.md)
- [enforce_runtime_schema_compatibility](../../../functions/src/db_utils/migrations/enforce_runtime_schema_compatibility.md)
- [migrate_stamps_table](../../../functions/src/db_utils/migrations/migrate_stamps_table.md)
- [column_names](../../../functions/src/db_utils/migrations/column_names.md)
- [run_pending_schema_migrations_adds_missing_columns_and_records_versions](../../../functions/src/db_utils/migrations/run_pending_schema_migrations_adds_missing_columns_and_records_versions.md)
- [run_pending_schema_migrations_rebuilds_unexpected_stamps_schema](../../../functions/src/db_utils/migrations/run_pending_schema_migrations_rebuilds_unexpected_stamps_schema.md)
- [run_pending_schema_migrations_repairs_preapplied_but_drifted_schema](../../../functions/src/db_utils/migrations/run_pending_schema_migrations_repairs_preapplied_but_drifted_schema.md)
- [migrate_fix_block_metadata](../../../functions/src/db_utils/migrations/migrate_fix_block_metadata.md)
- [normalize_currency_token_inner](../../../functions/src/db_utils/migrations/normalize_currency_token_inner.md)

# Imports

- `log::{debug, error}`
- `rusqlite::{params, Connection, Result}`
- `std::collections::HashSet`
- `super::{ensure_schema_migrations_table, run_pending_schema_migrations}`
- `rusqlite::Connection`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)