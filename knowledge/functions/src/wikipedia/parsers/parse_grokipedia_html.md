---
type: Rust Function
title: parse_grokipedia_html
resource: src/wikipedia/parsers.rs#L4-L41
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/wikipedia/fetchers/fetch_grokipedia_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_grokipedia_html(html: &str) -> Result<String, Box<dyn std::error::Error>>`

# Called by

- [fetch_grokipedia_content](../../../../functions/src/wikipedia/fetchers/fetch_grokipedia_content.md)