---
type: Rust Function
title: get_touchstamps_image
resource: src/commands_file_generator/touchstamps.rs#L142-L151
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/commands_file_generator/touchstamps/get_touchstamps_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_touchstamps_image(image_url: &str, _image_id: &str) -> ImageStatus`

# Called by

- [get_touchstamps_stamps](../../../../functions/src/commands_file_generator/touchstamps/get_touchstamps_stamps.md)