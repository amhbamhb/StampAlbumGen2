---
type: Rust Function
title: load_calibration_map
resource: src/album_utils/dimensions.rs#L74-L93
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_utils/dimensions/calculate_stamp_dimensions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn load_calibration_map(path: &str) -> std::collections::HashMap<String, (f32, f32)>`

# Called by

- [calculate_stamp_dimensions](../../../../functions/src/album_utils/dimensions/calculate_stamp_dimensions.md)