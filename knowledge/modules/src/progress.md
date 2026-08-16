---
type: Rust Module
title: progress
resource: src/progress.rs#L1-L135
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/indicatif-progressbar-progressstyle
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc-mutex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ProgressIndicator](../../classes/src/progress/ProgressIndicator.md)
- [new](../../functions/src/progress/ProgressIndicator/new.md)
- [new_with_config](../../functions/src/progress/ProgressIndicator/new_with_config.md)
- [set_message](../../functions/src/progress/ProgressIndicator/set_message.md)
- [set_message_format](../../functions/src/progress/ProgressIndicator/set_message_format.md)
- [finish_with_message](../../functions/src/progress/ProgressIndicator/finish_with_message.md)
- [finish_with_error](../../functions/src/progress/ProgressIndicator/finish_with_error.md)
- [clear](../../functions/src/progress/ProgressIndicator/clear.md)
- [drop](../../functions/src/progress/ProgressIndicator/drop/drop.md)
- [new_progress](../../functions/src/progress/new_progress.md)
- [new_progress_with_config](../../functions/src/progress/new_progress_with_config.md)

# Imports

- `indicatif::{ProgressBar, ProgressStyle}`
- `std::sync::{Arc, Mutex}`
- `std::time::Duration`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)