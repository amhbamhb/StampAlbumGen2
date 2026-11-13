---
type: Rust Function
title: set_stampworld_state
resource: src/review_ui.rs#L602-L606
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/stampworld_job_state
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/start_bootstrap_stampworld_job
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn set_stampworld_state(new_state: StampWorldJobState)`

# Calls

- [stampworld_job_state](../../../functions/src/review_ui/stampworld_job_state.md)

# Called by

- [start_bootstrap_stampworld_job](../../../functions/src/review_ui/start_bootstrap_stampworld_job.md)