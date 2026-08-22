---
type: Rust Function
title: get_cached_wikipedia_content
resource: src/wikipedia/legacy.rs#L17-L26
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/database/get_cached_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_cached_wikipedia_content( topic: &str, cache_db_path: &str, ) -> Result<Option<String>, Box<dyn std::error::Error>>`

# Calls

- [get_cached_content](../../../../functions/src/wikipedia/database/get_cached_content.md)