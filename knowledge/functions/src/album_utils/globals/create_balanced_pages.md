---
type: Rust Function
title: create_balanced_pages
resource: src/album_utils/globals.rs#L1183-L1244
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_utils/globals/balance_pages
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_utils/globals/add_wikipedia_section
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn create_balanced_pages( sections: &[ContentSection], heights: &[f64], usable_height: f64, _cfg: &crate::config::Config, ) -> Vec<Vec<usize>>`

# Calls

- [balance_pages](../../../../functions/src/album_utils/globals/balance_pages.md)

# Called by

- [add_wikipedia_section](../../../../functions/src/album_utils/globals/add_wikipedia_section.md)