---
type: Rust Function
title: fetch_grokipedia_content
resource: src/wikipedia/fetchers.rs#L4-L36
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/parsers/parse_grokipedia_html
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/wikipedia/database/fetch_and_cache_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn fetch_grokipedia_content( topic: &str, fetcher: &dyn Fetcher, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [contains](../../../../functions/src/native_pdf/stamp_image/contains.md)
- [parse_grokipedia_html](../../../../functions/src/wikipedia/parsers/parse_grokipedia_html.md)

# Called by

- [fetch_and_cache_content](../../../../functions/src/wikipedia/database/fetch_and_cache_content.md)