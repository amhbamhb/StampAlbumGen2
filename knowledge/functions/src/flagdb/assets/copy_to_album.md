---
type: Rust Function
title: copy_to_album
resource: src/flagdb/assets.rs#L222-L231
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/flagdb/assets/valid_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/write_atomic
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/copy_flag_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/valid_svg_is_rendered_to_a_real_png_at_the_requested_size
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/album_local_fallback_checks_real_images_and_preserves_svg_extension
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn copy_to_album(source: &Path, dir: &Path) -> AssetResult<String>`

# Calls

- [valid_file](../../../../functions/src/flagdb/assets/valid_file.md)
- [write_atomic](../../../../functions/src/flagdb/assets/write_atomic.md)

# Called by

- [copy_flag_for_album](../../../../functions/src/flagdb/copy_flag_for_album.md)
- [valid_svg_is_rendered_to_a_real_png_at_the_requested_size](../../../../functions/src/flagdb/assets/valid_svg_is_rendered_to_a_real_png_at_the_requested_size.md)
- [album_local_fallback_checks_real_images_and_preserves_svg_extension](../../../../functions/src/flagdb/assets/album_local_fallback_checks_real_images_and_preserves_svg_extension.md)