---
type: Rust Function
title: build_stampworld_args
resource: src/review_ui.rs#L562-L579
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/preset_flags
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/parse_extra_flags
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/start_bootstrap_stampworld_job
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_stampworld_args(req: &BootstrapStampWorldRunRequest) -> Vec<String>`

# Calls

- [preset_flags](../../../functions/src/review_ui/preset_flags.md)
- [parse_extra_flags](../../../functions/src/review_ui/parse_extra_flags.md)

# Called by

- [start_bootstrap_stampworld_job](../../../functions/src/review_ui/start_bootstrap_stampworld_job.md)