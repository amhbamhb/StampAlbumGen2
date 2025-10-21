# CI converter scripts

This folder contains a small fallback converter used by CI to produce JUnit XML test reports from Cargo's JSON output.

Why this exists

- Some CI environments (or cargo versions) don't yield JSON test-suite events in the stream that community tools like `cargo2junit` expect. When that happens `cargo2junit` reports "No test suite results were found." and produces an empty JUnit output.
- To ensure GitLab (or other CI systems) always get a JUnit report, we capture Cargo's output and run a simple fallback parser that extracts test names, statuses, and failure stdout blocks from the captured log.

Files

- `cargo_json_to_junit.py` — a minimal, best-effort converter that parses both JSON `reason:test` events (when present) and plain-text test lines like `test foo ... ok`.

How to run locally

1. Run tests and capture Cargo's JSON output to a file:

```bash
cargo test --workspace --all-features --message-format=json 2>&1 | tee target/cargo-json.log || true
```

2. Convert the log to JUnit XML:

```bash
python3 scripts/cargo_json_to_junit.py target/cargo-json.log target/junit-report.xml
```

3. Inspect the resulting JUnit XML:

```bash
xmllint --format target/junit-report.xml | sed -n '1,200p'
# or
head -n 50 target/junit-report.xml
```

Notes and limitations

- This is intentionally conservative: it attempts to avoid false positives and prefers a simple complete JUnit file over perfect fidelity with cargo internal timings and nuance.
- The script does not currently capture exact test durations or per-crate grouping; it records test names and marks failures where possible. If you need richer metadata, consider improving the parser or using a more capable converter if your environment supports it.
- The CI job tries `cargo2junit` first for environments where it's available and known to work, then falls back to this converter automatically.

If you improve the converter, please add tests and update this README with any required runtime dependencies (e.g. Python version).
