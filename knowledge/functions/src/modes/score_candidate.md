---
type: Rust Function
title: score_candidate
resource: src/modes.rs#L590-L626
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/selection/count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/remap_colnect_sg_to_existing_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn score_candidate( col_tokens: &[String], cand_tokens: &[String], col_series: &str, cand_series: &str, cand_image_path: &str, ) -> i32`

# Calls

- [count](../../../functions/src/album_globals/selection/count.md)
- [contains](../../../functions/src/native_pdf/stamp_image/contains.md)

# Called by

- [remap_colnect_sg_to_existing_stamps](../../../functions/src/modes/remap_colnect_sg_to_existing_stamps.md)