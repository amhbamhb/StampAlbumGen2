---
type: Rust Function
title: estimate_text_height
resource: src/wikipedia/utils.rs#L5-L59
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/selection/count
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_utils/globals/add_wikipedia_section
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn estimate_text_height(text: &str, config: &Config) -> f64`

# Calls

- [count](../../../../functions/src/album_globals/selection/count.md)

# Called by

- [add_wikipedia_section](../../../../functions/src/album_utils/globals/add_wikipedia_section.md)