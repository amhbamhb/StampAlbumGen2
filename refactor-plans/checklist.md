## Split-and-extract checklist

Purpose

- Provide a repeatable, low-risk checklist for extracting a function/file/module into a new module or crate.

Before you start

- Create a short design note (1–2 paragraphs) describing the intended public API, responsibilities, and failure modes.
- Ensure there are focused unit tests covering the behaviour you plan to move (happy path + 1-2 edge cases).
- Choose a branch name: `refactor/<file-or-area>` (we created `refactor/continue` for ongoing work).

Checklist (small, reversible steps)

1. Add a TODO/checklist entry for this specific extraction describing the goal and success criteria.
2. Create a new module or crate skeleton (file + tests) and add a minimal public API surface (function signatures) with TODO stubs returning unimplemented!() or delegating to the original code.
3. Compile and run tests (should still pass if you haven't removed original behaviour). If the project builds, continue.
4. Incrementally move small functions (and their tests) into the new module. For each function moved:
   - Update callers to import from the new module.
   - Run `cargo test` locally and fix any type or lifetime issues immediately.
5. When moving data types, prefer adding `pub(crate)` visibility first and widen to `pub` only after callers stabilize.
6. Add focused unit tests in the new module to exercise its public API directly.
7. Keep commits small and atomic (one logical change per commit). Use conventional commit messages (e.g., `refactor(module): move parse_x to stampworld::parser`).
8. Add a thin compatibility shim in the original location that delegates to the new module when necessary. Mark the shim with a clear TODO and a FIXME comment pointing to the refactor plan.
9. Run full test suite (`cargo test`) and a quick `cargo build --release` smoke test. If failures appear, revert the last step/change and investigate.
10. When stable, remove the shim in a follow-up PR after ensuring all callers are updated and tests pass.

Module contract template (fill per-extraction)

- Name: `crate::path::module`
- Responsibilities: (one-sentence)
- Public API (functions/types):
  - fn foo(args) -> Result<T, E> — description, inputs/outputs, errors
  - struct Bar { ... } — invariants and serialization/DB mapping notes
- Error modes: list expected error types and how callers should handle them
- Tests included: list of unit and integration tests added

Edge cases and checks

- Null/empty inputs
- Large inputs / streaming / memory use
- I/O and network failures (timeouts, retries)
- Concurrency: ensure shared state is synchronized or marked Send/Sync expectations

Validation and quality gates

- Build: `cargo build --all`
- Tests: `cargo test --all`
- Format: `cargo fmt -- --check`
- Lint: `cargo clippy -- -D warnings` (optional; fix incrementally)

Example quick commands

```sh
# run tests
cargo test

# check formatting
cargo fmt -- --check
```

Notes

- When extracting to a new crate, update the workspace `Cargo.toml` and prefer path dependencies during development.
- If an extraction causes large diffs or churn, break it into multiple smaller PRs each moving one small piece.
