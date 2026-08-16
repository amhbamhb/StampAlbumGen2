---
type: Rust Function
title: format_wikipedia_content_for_album
resource: src/wikipedia/formatters.rs#L106-L108
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/formatters/format_content_for_album
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/test_wikipedia_vspace/paragraph_vspace_uses_configured_value
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn format_wikipedia_content_for_album(content: &str, config: &Config) -> Vec<String>`

# Calls

- [format_content_for_album](../../../../functions/src/wikipedia/formatters/format_content_for_album.md)

# Called by

- [paragraph_vspace_uses_configured_value](../../../../functions/tests/test_wikipedia_vspace/paragraph_vspace_uses_configured_value.md)