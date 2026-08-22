---
type: Rust Module
title: backfill_image_dims
resource: src/bin/backfill_image_dims.rs#L1-L172
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/flexi-logger-duplicate-filespec-logger
    resolved_by: tree-sitter
    confidence: exact
  - target: external/log-debug-error-info-warn
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-env
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [print_usage](../../../functions/src/bin/backfill_image_dims/print_usage.md)
- [resolve_dimensions](../../../functions/src/bin/backfill_image_dims/resolve_dimensions.md)
- [process_db](../../../functions/src/bin/backfill_image_dims/process_db.md)
- [main](../../../functions/src/bin/backfill_image_dims/main.md)

# Imports

- `flexi_logger::{Duplicate, FileSpec, Logger}`
- `log::{debug, error, info, warn}`
- `std::env`
- `std::path::{Path, PathBuf}`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)