---
type: Rust Function
title: create_greedy_pages
resource: src/album_utils/globals.rs#L1281-L1313
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

`fn create_greedy_pages( sections: &[ContentSection], heights: &[f64], usable_height: f64, ) -> Vec<Vec<usize>>`

# Called by

- [add_wikipedia_section](../../../../functions/src/album_utils/globals/add_wikipedia_section.md)