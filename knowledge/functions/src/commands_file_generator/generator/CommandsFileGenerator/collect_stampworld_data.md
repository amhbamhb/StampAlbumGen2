---
type: Rust Method
title: collect_stampworld_data
resource: src/commands_file_generator/generator.rs#L140-L209
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_utils/text_utils/sanitize_filename
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/image_utils/get_image
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn collect_stampworld_data<'a>( &mut self, params: CollectStampworldCallParams<'a>, ) -> Result<Vec<String>, Box<dyn std::error::Error>>`

# Calls

- [sanitize_filename](../../../../../functions/src/album_utils/text_utils/sanitize_filename.md)
- [get_image](../../../../../functions/src/commands_file_generator/image_utils/get_image.md)