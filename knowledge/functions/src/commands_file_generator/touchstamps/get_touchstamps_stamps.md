---
type: Rust Function
title: get_touchstamps_stamps
resource: src/commands_file_generator/touchstamps.rs#L11-L139
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/touchstamps/get_touchstamps_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/image_utils/get_image
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_touchstamps_stamps( link: &str, year: &str, month: &str, day: &str, desc: &str, album_name: &str, images_directory: &str, ) -> Vec<String>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [html](../../../../functions/src/review_ui/views/compare/html.md)
- [get_touchstamps_image](../../../../functions/src/commands_file_generator/touchstamps/get_touchstamps_image.md)
- [get_image](../../../../functions/src/commands_file_generator/image_utils/get_image.md)