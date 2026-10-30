---
type: Rust Function
title: count_albumeasy_text_lines
resource: src/album_globals/generator.rs#L231-L235
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/selection/ColnectPdfFilters/matches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/count
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/stamp_content_height
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn count_albumeasy_text_lines(s: &str) -> usize`

# Calls

- [matches](../../../../functions/src/album_globals/selection/ColnectPdfFilters/matches.md)
- [count](../../../../functions/src/album_globals/selection/count.md)

# Called by

- [stamp_content_height](../../../../functions/src/album_globals/generator/stamp_content_height.md)