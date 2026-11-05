---
type: Rust Module
title: external_album
resource: src/external_album.rs#L1-L1480
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/rusqlite-params-connection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sha2-digest-sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-btreeset-hashmap-hashset
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs-self-file-openoptions
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-read
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-component-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-process-command-stdio
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration-instant
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-seek-seekfrom
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Argument](../../classes/src/external_album/Argument.md)
- [LayoutCommand](../../classes/src/external_album/LayoutCommand.md)
- [LayoutPage](../../classes/src/external_album/LayoutPage.md)
- [Inspection](../../classes/src/external_album/Inspection.md)
- [AlbumSummary](../../classes/src/external_album/AlbumSummary.md)
- [AlbumDetails](../../classes/src/external_album/AlbumDetails.md)
- [err](../../functions/src/external_album/err.md)
- [json](../../functions/src/external_album/json.md)
- [sql_integer](../../functions/src/external_album/sql_integer.md)
- [sql_usize](../../functions/src/external_album/sql_usize.md)
- [slug](../../functions/src/external_album/slug.md)
- [validate_slug](../../functions/src/external_album/validate_slug.md)
- [windows_reserved](../../functions/src/external_album/windows_reserved.md)
- [relative](../../functions/src/external_album/relative.md)
- [portable_path](../../functions/src/external_album/portable_path.md)
- [no_links](../../functions/src/external_album/no_links.md)
- [Inventory](../../classes/src/external_album/Inventory.md)
- [digest](../../functions/src/external_album/digest.md)
- [inventory](../../functions/src/external_album/inventory.md)
- [scan_inventory](../../functions/src/external_album/scan_inventory.md)
- [entry_relative](../../functions/src/external_album/entry_relative.md)
- [asset_errors](../../functions/src/external_album/asset_errors.md)
- [uploaded_image](../../functions/src/external_album/uploaded_image.md)
- [uploaded_hash](../../functions/src/external_album/uploaded_hash.md)
- [copy_uploaded_assets](../../functions/src/external_album/copy_uploaded_assets.md)
- [editable_asset_errors](../../functions/src/external_album/editable_asset_errors.md)
- [inspect](../../functions/src/external_album/inspect.md)
- [copy_tree](../../functions/src/external_album/copy_tree.md)
- [schema](../../functions/src/external_album/schema.md)
- [is_imported](../../functions/src/external_album/is_imported.md)
- [paths](../../functions/src/external_album/paths.md)
- [open](../../functions/src/external_album/open.md)
- [details](../../functions/src/external_album/details.md)
- [load_album](../../functions/src/external_album/load_album.md)
- [asset_directory](../../functions/src/external_album/asset_directory.md)
- [list_albums](../../functions/src/external_album/list_albums.md)
- [persist_pages](../../functions/src/external_album/persist_pages.md)
- [import_album](../../functions/src/external_album/import_album.md)
- [Rendered](../../classes/src/external_album/Rendered.md)
- [render_pair](../../functions/src/external_album/render_pair.md)
- [verify_source](../../functions/src/external_album/verify_source.md)
- [pdf_valid](../../functions/src/external_album/pdf_valid.md)
- [render](../../functions/src/external_album/render.md)
- [layout_hash](../../functions/src/external_album/layout_hash.md)
- [verify_layout](../../functions/src/external_album/verify_layout.md)
- [set_rendered](../../functions/src/external_album/set_rendered.md)
- [verify_uploaded](../../functions/src/external_album/verify_uploaded.md)
- [retry_album](../../functions/src/external_album/retry_album.md)
- [approve_album](../../functions/src/external_album/approve_album.md)
- [edit_page](../../functions/src/external_album/edit_page.md)
- [read_entry](../../functions/src/external_album/read_entry.md)
- [export_layout](../../functions/src/external_album/export_layout.md)
- [generate_artifacts](../../functions/src/external_album/generate_artifacts.md)

# Imports

- `rusqlite::{params, Connection}`
- `serde::{Deserialize, Serialize}`
- `sha2::{Digest, Sha256}`
- `std::collections::{BTreeSet, HashMap, HashSet}`
- `std::fs::{self, File, OpenOptions}`
- `std::io::Read`
- `std::path::{Component, Path, PathBuf}`
- `std::process::{Command, Stdio}`
- `std::time::{Duration, Instant}`
- `std::io::{Seek, SeekFrom}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)