---
type: Rust Function
title: fetch_and_save_summary
resource: src/wikipedia/legacy.rs#L32-L38
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/legacy/fetch_and_save_summary_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn fetch_and_save_summary( topic: &str, album_dir: &str, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [fetch_and_save_summary_with_fetcher](../../../../functions/src/wikipedia/legacy/fetch_and_save_summary_with_fetcher.md)