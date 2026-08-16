---
type: Rust Function
title: fetch_and_cache_wikipedia_content
resource: src/wikipedia/legacy.rs#L8-L14
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/database/fetch_and_cache_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn fetch_and_cache_wikipedia_content( topic: &str, cache_db_path: &str, fetcher: &dyn Fetcher, ) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [fetch_and_cache_content](../../../../functions/src/wikipedia/database/fetch_and_cache_content.md)