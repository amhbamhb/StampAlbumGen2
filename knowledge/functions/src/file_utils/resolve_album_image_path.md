---
type: Rust Function
title: resolve_album_image_path
resource: src/file_utils.rs#L34-L40
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/file_utils/resolve_album_dir
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/estimate_and_ensure_placeholders
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/dimensions/calculate_stamp_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/file_utils/image_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_downloader/download_stamp_image_if_missing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_downloader/download_missing_stamp_images
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/resolve_image_abs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/local_image_file_exists
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn resolve_album_image_path(album_name: &str, image_path: &str) -> PathBuf`

# Calls

- [resolve_album_dir](../../../functions/src/file_utils/resolve_album_dir.md)

# Called by

- [generate_album_output_conn_prepared](../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [estimate_and_ensure_placeholders](../../../functions/src/album_globals/generator_new/helpers/estimate_and_ensure_placeholders.md)
- [calculate_stamp_dimensions](../../../functions/src/album_utils/dimensions/calculate_stamp_dimensions.md)
- [image_exists](../../../functions/src/file_utils/image_exists.md)
- [download_stamp_image_if_missing](../../../functions/src/image_downloader/download_stamp_image_if_missing.md)
- [download_missing_stamp_images](../../../functions/src/image_downloader/download_missing_stamp_images.md)
- [resolve_image_abs](../../../functions/src/review_ui/resolve_image_abs.md)
- [local_image_file_exists](../../../functions/src/review_ui/http_handlers/local_image_file_exists.md)