---
type: Rust Function
title: resolve_image_path
resource: src/bin/inspect_image_paths.rs#L5-L16
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/bin/inspect_image_paths/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_image_path(images_dir: &Path, image_path: &str) -> PathBuf`

# Called by

- [main](../../../../functions/src/bin/inspect_image_paths/main.md)