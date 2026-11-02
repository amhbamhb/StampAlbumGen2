---
type: Rust Function
title: parse_content_into_sections
resource: src/album_utils/globals.rs#L1145-L1229
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_utils/globals/add_wikipedia_section
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_content_into_sections(content: &str) -> Vec<ContentSection>`

# Called by

- [add_wikipedia_section](../../../../functions/src/album_utils/globals/add_wikipedia_section.md)