---
type: Rust Function
title: get_image_blocking
resource: src/commands_file_generator/image_utils.rs#L101-L168
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/commands_file_generator/image_utils/get_image
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn get_image_blocking(_url: &str, _name: &str, _file_path: &str) -> ImageStatus`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [status](../../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status.md)

# Called by

- [get_image](../../../../functions/src/commands_file_generator/image_utils/get_image.md)