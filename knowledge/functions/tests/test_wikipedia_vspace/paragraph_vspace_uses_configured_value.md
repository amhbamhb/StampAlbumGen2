---
type: Rust Function
title: paragraph_vspace_uses_configured_value
resource: tests/test_wikipedia_vspace.rs#L5-L39
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/Config/with_defaults
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/formatters/format_wikipedia_content_for_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn paragraph_vspace_uses_configured_value()`

# Calls

- [with_defaults](../../../functions/src/config/Config/with_defaults.md)
- [format_wikipedia_content_for_album](../../../functions/src/wikipedia/formatters/format_wikipedia_content_for_album.md)