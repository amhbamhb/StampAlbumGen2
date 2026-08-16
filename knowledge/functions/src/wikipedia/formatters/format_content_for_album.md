---
type: Rust Function
title: format_content_for_album
resource: src/wikipedia/formatters.rs#L5-L103
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/wikipedia/formatters/format_wikipedia_content_for_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn format_content_for_album( content: &str, source: &ContentSource, config: &Config, ) -> Vec<String>`

# Called by

- [format_wikipedia_content_for_album](../../../../functions/src/wikipedia/formatters/format_wikipedia_content_for_album.md)