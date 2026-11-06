---
type: Rust Method
title: load
resource: src/native_pdf/commands/page/album_pages_decorative_border.rs#L122-L239
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_decorative_border/load_decorative_png
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load(path: &Path, mut policy: DecorativeBorderPolicy) -> Result<Self>`

# Calls

- [contains](../../../../../../../functions/src/native_pdf/stamp_image/contains.md)
- [load_decorative_png](../../../../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/load_decorative_png.md)