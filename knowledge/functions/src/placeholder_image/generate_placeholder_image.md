---
type: Rust Function
title: generate_placeholder_image
resource: src/placeholder_image.rs#L457-L463
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/placeholder_image/generate_placeholder_image_with_options
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/placeholder_image/test_generate_placeholder_image
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_placeholder_image( width: u32, height: u32, output_path: &Path, ) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [generate_placeholder_image_with_options](../../../functions/src/placeholder_image/generate_placeholder_image_with_options.md)

# Called by

- [test_generate_placeholder_image](../../../functions/tests/placeholder_image/test_generate_placeholder_image.md)