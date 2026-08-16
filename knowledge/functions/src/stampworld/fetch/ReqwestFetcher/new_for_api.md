---
type: Rust Method
title: new_for_api
resource: src/stampworld/fetch.rs#L81-L97
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/flagdb/ensure_coat_of_arms
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/legacy/fetch_and_save_summary
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn new_for_api() -> Self`

# Called by

- [ensure_coat_of_arms](../../../../../functions/src/flagdb/ensure_coat_of_arms.md)
- [run_scraping_mode](../../../../../functions/src/modes/run_scraping_mode.md)
- [fetch_and_save_summary](../../../../../functions/src/wikipedia/legacy/fetch_and_save_summary.md)