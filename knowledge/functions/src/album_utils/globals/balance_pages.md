---
type: Rust Function
title: balance_pages
resource: src/album_utils/globals.rs#L1318-L1363
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_utils/globals/create_balanced_pages
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn balance_pages(pages: &mut [Vec<usize>], heights: &[f64], usable_height: f64)`

# Called by

- [create_balanced_pages](../../../../functions/src/album_utils/globals/create_balanced_pages.md)