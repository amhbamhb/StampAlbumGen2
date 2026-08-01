## Migration Phase 1 — Open DB in `main` and use conn-taking API

### Goal

Make a minimal, low-risk change so the program opens a single SQLite connection in `main` and passes a `&rusqlite::Connection` to the core generator API (`generate_album_output_conn`). This improves error handling and makes future refactors (extracting `stamp_album_db` or using a connection pool) simpler.

### Why

- Early friendly DB validation (detect corrupt files with helpful messaging).
- Avoid repeatedly opening/closing the DB when the caller already has one.
- Simplifies ownership for future crate extraction.

### Scope

- Edit `src/main.rs` to call `stamp_album_gen2::db_utils::init_db(&db_path)` once and pass the resulting `Connection` to `stamp_album_gen2::album_globals::generate_album_output_conn`.
- Keep the backwards-compatible wrapper `generate_album_output(db_path, ...)` in place.
- Add an in-memory DB test that calls `generate_album_output_conn` to ensure the surface works with a live `Connection`.

### Rollbacks

- Changes are small and localized to `src/main.rs` and a new test and doc file. Revert by resetting those files.

### Next steps after Phase 1

1. Consider creating `stamp_album_db` crate and moving `db_utils.rs` into it.

2. Create `stamp_album_core` and move `album_globals.rs` there.

3. Move `src/main.rs` into `stamp_album_cli` binary crate that depends on core + db crates.

### Notes

- Keep this file in the repo as a single source-of-truth while further refactors happen. If unrelated bugfixes are applied to `main.rs` during other work, re-open this doc and mark progress.
# Migration Phase 1 — Open DB in `main` and use conn-taking API

## Goal
Make a minimal, low-risk change so the program opens a single SQLite connection in `main` and passes a `&rusqlite::Connection` to the core generator API (`generate_album_output_conn`). This improves error handling and makes future refactors (extracting `stamp_album_db` or using a connection pool) simpler.

# Why
- Early friendly DB validation (detect corrupt files with helpful messaging).
- Avoid repeatedly opening/closing the DB when the caller already has one.
- Simplifies ownership for future crate extraction.

# Scope
- Edit `src/main.rs` to call `stamp_album_gen2::db_utils::init_db(&db_path)` once and pass the resulting `Connection` to `stamp_album_gen2::album_globals::generate_album_output_conn`.
- Keep the backwards-compatible wrapper `generate_album_output(db_path, ...)` in place.
- Add an in-memory DB test that calls `generate_album_output_conn` to ensure the surface works with a live `Connection`.

# Rollbacks
- Changes are small and localized to `src/main.rs` and a new test and doc file. Revert by resetting those files.

# Next steps after Phase 1
1. Consider creating `stamp_album_db` crate and moving `db_utils.rs` into it.
2. Create `stamp_album_core` and move `album_globals.rs` there.
3. Move `src/main.rs` into `stamp_album_cli` binary crate that depends on core + db crates.

# Notes
Keep this file in the repo as a single source-of-truth while further refactors happen. If unrelated bugfixes are applied to `main.rs` during other work, re-open this doc and mark progress.
