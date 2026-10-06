---
type: Rust Function
title: parse_grokipedia_html
resource: src/wikipedia/parsers.rs#L4-L41
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/parser/parse
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/wikipedia/fetchers/fetch_grokipedia_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_grokipedia_html(html: &str) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [parse](../../../../functions/src/external_album/parser/parse.md)

# Called by

- [fetch_grokipedia_content](../../../../functions/src/wikipedia/fetchers/fetch_grokipedia_content.md)