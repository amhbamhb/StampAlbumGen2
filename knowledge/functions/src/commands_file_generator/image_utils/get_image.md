---
type: Rust Function
title: get_image
resource: src/commands_file_generator/image_utils.rs#L82-L99
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/commands_file_generator/image_utils/get_image_blocking
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/commands_file_generator/generator/CommandsFileGenerator/collect_stampworld_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/image_utils/get_colnect_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/touchstamps/get_touchstamps_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_image(url: &str, name: &str, file_path: &str) -> ImageStatus`

# Calls

- [get_image_blocking](../../../../functions/src/commands_file_generator/image_utils/get_image_blocking.md)

# Called by

- [collect_stampworld_data](../../../../functions/src/commands_file_generator/generator/CommandsFileGenerator/collect_stampworld_data.md)
- [get_colnect_image](../../../../functions/src/commands_file_generator/image_utils/get_colnect_image.md)
- [get_touchstamps_stamps](../../../../functions/src/commands_file_generator/touchstamps/get_touchstamps_stamps.md)