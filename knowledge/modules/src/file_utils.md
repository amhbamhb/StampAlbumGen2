---
type: Rust Module
title: file_utils
resource: src/file_utils.rs#L1-L121
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-types-album-db-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/log-debug
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-env
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-types-overwrite-album
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [resolve_runtime_albums_root](../../functions/src/file_utils/resolve_runtime_albums_root.md)
- [resolve_album_dir](../../functions/src/file_utils/resolve_album_dir.md)
- [resolve_album_image_path](../../functions/src/file_utils/resolve_album_image_path.md)
- [set_db_path_from_cli](../../functions/src/file_utils/set_db_path_from_cli.md)
- [get_db_path_from_cli](../../functions/src/file_utils/get_db_path_from_cli.md)
- [find_config_path](../../functions/src/file_utils/find_config_path.md)
- [image_exists](../../functions/src/file_utils/image_exists.md)
- [set_overwrite_album_from_cli](../../functions/src/file_utils/set_overwrite_album_from_cli.md)
- [get_overwrite_album_from_cli](../../functions/src/file_utils/get_overwrite_album_from_cli.md)
- [get_image_path_or_placeholder](../../functions/src/file_utils/get_image_path_or_placeholder.md)

# Imports

- `crate::types::ALBUM_DB_PATH`
- `log::debug`
- `std::path::PathBuf`
- `std::env`
- `crate::types::OVERWRITE_ALBUM`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)