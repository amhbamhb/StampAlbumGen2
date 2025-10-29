# Refactor plan: `src/stampworld/data_collector.rs`

Goal
- Incrementally extract concerns from `src/stampworld/data_collector.rs` so each responsibility is isolated, unit-testable, and small.
- Keep changes small and reversible. Each extraction should compile on its own and be covered by at least one focused test.

Contract (for this module)
- Inputs: data_collector currently processes HTML pages (strings), does network fetches, parses stamp blocks, and calls DB insert helpers.
- Outputs: structured Stamp metadata (StampStub), network fetch results (raw HTML or bytes), and calls into DB insertion helpers.
- Error modes: network errors, parsing inconsistencies (missing expected nodes), DB constraint errors. Each extraction must surface errors via Result<T, E> and not panic.

Edge cases to cover
- Empty or truncated HTML pages
- Non-200 HTTP responses and redirects
- Missing fields (no `sg_number`, missing `stamps_issued`) — preserve best-effort parsing
- Multi-perforation / multi-stamp blocks (the code uses `multi_perf`) — preserve existing behavior
- Image download timeouts and invalid URLs

Phased small steps (one-file, one-change each)
1) Parser helpers extracted (DONE)
   - File: `src/stampworld/parser.rs`
   - Rationale: isolated, deterministic, pure-string logic; easy to unit test.

2) Extract network fetch layer (recommended next)
   - File: `src/stampworld/fetch.rs` (or `net.rs`)
   - What to extract: blocking HTTP GET/POST wrappers, retries, headers, timeouts, local caching hook (if present).
   - Contract: expose `fetch_html(url: &str) -> Result<String, FetchError>` and `fetch_bytes(url: &str) -> Result<Vec<u8>, FetchError>`.
   - Why: makes it trivial to unit-test parsing by injecting HTML, and to mock network in integration tests.
   - Tests: add unit tests that mock the HTTP client or provide a small trait that can be swapped in tests.

3) Extract persistence adapter (thin delegation)
   - File: `src/stampworld/persist.rs` or reuse the existing `db_utils` wrapper to create a `stampworld::persist` that calls into `crate::db_utils`.
   - What to extract: functions that transform parsed Stamp data into DB-friendly forms and call insert functions.
   - Contract: synchronous, `persist_stamp(conn, stamp: StampStub) -> Result<(), PersistError>`.
   - Tests: use an in-memory DB via `stamp_album_db::init_db(":memory:")` for tests.

4) Extract block-level orchestration & small helpers
   - File: `src/stampworld/orchestrator.rs` (or keep in data_collector but shrink)
   - What to extract: high-level logic that iterates pages/blocks and coordinates fetch->parse->persist

5) Follow-ups
   - Add integration tests that run a small end-to-end flow using recorded HTML fixtures
   - Add a feature-flag for real network calls versus fixtures
   - Add documentation and update `docs/` and README

Verification (for each step)
- Run: `cargo build --release`
- Run: `cargo test -p stamp_album_db` for DB-related tests
- For stampworld module: `cargo test --lib` or `cargo test -p stamp_album_gen2 --lib` as appropriate

How I'll proceed next (unless you tell me otherwise)
- Create this plan file (done).
- Implement Step 2 (extract network fetch layer) as a small change:
  - Create `src/stampworld/fetch.rs` with a small wrapper around `reqwest::blocking::get` (or the existing client usage), exposing `fetch_html` and `fetch_bytes`.
  - Replace direct network calls in `data_collector.rs` with calls to the new `fetch` module.
  - Add one unit test for `fetch` that validates error propagation (mocking may be deferred; simplest test: invalid URL returns Err).
  - Run `cargo test --lib` and `cargo test -p stamp_album_db`.

If you'd prefer a different next extraction (persistence adapter or orchestrator), tell me and I will implement that instead.

---

Files changed/created by this plan (expected)
- `refactor-plans/stampworld_data_collector.md` (this file)
- `src/stampworld/fetch.rs` (next implementation)
- small edits to `src/stampworld/data_collector.rs` to call into `stampworld::fetch`

Commands I will run after each small change (to validate)
```bash
cargo build --release
cargo test -p stamp_album_db
cargo test --lib
```

