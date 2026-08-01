# Library re-exports

This project re-exports a small set of database helper functions at the crate root so they can be consumed by integration tests or other crates without reaching into internal modules.

Available re-exports (use as `stamp_album_gen2::<name>`):

- `insert_stamp` — insert or backfill a detailed stamp row into an album DB.
- `insert_stamp_placeholder` — insert a minimal placeholder row for grouped-range entries.
- `migrate_stamps_table` — migrate/normalize the `stamps` table to the canonical schema.
- `migrate_fix_block_metadata` — migration helper that extracts block metadata into explicit columns.

Usage example:

```rust
use stamp_album_gen2::insert_stamp_placeholder;

// call the helper passing a rusqlite::Connection and stamp values
```

Rationale: exporting a small curated set of helpers simplifies integration testing and makes the crate a lightweight library for other tooling in the stamping ecosystem. The intent is to keep the public API small and stable; internal helpers remain in `src/db_utils.rs` and are not exported unless explicitly chosen here.
