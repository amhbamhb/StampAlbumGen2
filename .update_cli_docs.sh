#!/usr/bin/env bash
set -e

cat > docs/CLI_USAGE.md <<'EOF'
## CLI usage (generated from code)

The content below is authoritative help text produced by the program's clap-based CLI and reflects the current available options. It is generated from `src/main.rs` and should be refreshed when new CLI flags are added.

```text
EOF

cargo run -- --help >> docs/CLI_USAGE.md

cat >> docs/CLI_USAGE.md <<'EOF'
```

Notes:
- The `--end-year` default is set at compile time via `build.rs` (the `BUILD_YEAR` constant). On releases this will be the release year.
- Prefer regenerating this file from the code (e.g. `cargo run -- --help > docs/CLI_USAGE.md`) during release/update steps to avoid drift.

If you'd like, I can also add a small CI check that asserts `cargo run -- --help` matches `docs/CLI_USAGE.md` to prevent future divergence.
EOF
