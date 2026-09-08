---
type: Rust Function
title: get_image
resource: src/commands_file_generator/image_utils.rs#L78-L145
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
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

`pub fn get_image(_url: &str, _name: &str, _file_path: &str) -> ImageStatus`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [collect_stampworld_data](../../../../functions/src/commands_file_generator/generator/CommandsFileGenerator/collect_stampworld_data.md)
- [get_colnect_image](../../../../functions/src/commands_file_generator/image_utils/get_colnect_image.md)
- [get_touchstamps_stamps](../../../../functions/src/commands_file_generator/touchstamps/get_touchstamps_stamps.md)