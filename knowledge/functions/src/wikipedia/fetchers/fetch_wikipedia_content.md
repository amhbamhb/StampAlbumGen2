---
type: Rust Function
title: fetch_wikipedia_content
resource: src/wikipedia/fetchers.rs#L39-L82
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/parsers/parse_wikipedia_html
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/wikipedia/database/fetch_and_cache_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn fetch_wikipedia_content( topic: &str, fetcher: &dyn Fetcher, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [parse_wikipedia_html](../../../../functions/src/wikipedia/parsers/parse_wikipedia_html.md)

# Called by

- [fetch_and_cache_content](../../../../functions/src/wikipedia/database/fetch_and_cache_content.md)