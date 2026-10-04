---
type: Rust Method
title: wikipedia_ingest_enabled
resource: src/config.rs#L1568-L1570
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn wikipedia_ingest_enabled(&self) -> bool`

# Called by

- [run_scraping_mode](../../../../functions/src/modes/run_scraping_mode.md)