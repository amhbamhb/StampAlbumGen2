---
type: Rust Function
title: estimate_header_height
resource: src/wikipedia/utils.rs#L62-L84
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_utils/globals/add_wikipedia_section
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn estimate_header_height(header_text: &str, config: &Config) -> f64`

# Called by

- [add_wikipedia_section](../../../../functions/src/album_utils/globals/add_wikipedia_section.md)