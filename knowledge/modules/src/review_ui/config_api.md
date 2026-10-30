---
type: Rust Module
title: config_api
resource: src/review_ui/config_api.rs#L1-L31
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/std-sync-arc-mutex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-task-context-poll
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ReviewHandlersStateRequests](../../../classes/src/review_ui/config_api/ReviewHandlersStateRequests.md)
- [UiState](../../../classes/src/review_ui/config_api/UiState.md)
- [UiStateInner](../../../classes/src/review_ui/config_api/UiStateInner.md)
- [check_and_clear_reload](../../../functions/src/review_ui/config_api/UiState/check_and_clear_reload.md)
- [check_and_clear_shutdown](../../../functions/src/review_ui/config_api/UiState/check_and_clear_shutdown.md)

# Imports

- `std::sync::{Arc, Mutex}`
- `std::task::{Context, Poll}`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)