---
type: Rust Function
title: main
resource: src/bin/inspect_image_paths.rs#L18-L97
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/inspect_image_paths/resolve_image_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn main() -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [from](../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [resolve_image_path](../../../../functions/src/bin/inspect_image_paths/resolve_image_path.md)