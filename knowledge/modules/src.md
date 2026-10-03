---
type: Rust Module
title: src
resource: src/lib.rs#L1-L57
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/pub-use-crate-config-load-slug-overrides-config-headerformatting
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-db-utils-insert-stamp-insert-stamp-placeholder-migrate-fix-block-metadata-migrate-stamps-table
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-album-globals-generate-album-output
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-file-utils
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-album-utils
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-types-imagestatus-ostype-seriesinfo
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-config-load-effective-config
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Imports

- `pub use crate::config::{load_slug_overrides, Config, HeaderFormatting}`
- `pub use db_utils::{
    insert_stamp, insert_stamp_placeholder, migrate_fix_block_metadata, migrate_stamps_table,
}`
- `pub use album_globals::generate_album_output`
- `pub use file_utils::*`
- `pub use album_utils::*`
- `pub use types::{ImageStatus, OSType, SeriesInfo}`
- `pub use config::load_effective_config`

# Member of

- [stamp_album_gen2](../packages/stamp_album_gen2.md)