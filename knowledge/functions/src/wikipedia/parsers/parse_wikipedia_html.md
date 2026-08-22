---
type: Rust Function
title: parse_wikipedia_html
resource: src/wikipedia/parsers.rs#L44-L90
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/wikipedia/fetchers/fetch_wikipedia_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_wikipedia_html(html: &str) -> Result<String, Box<dyn std::error::Error>>`

# Called by

- [fetch_wikipedia_content](../../../../functions/src/wikipedia/fetchers/fetch_wikipedia_content.md)