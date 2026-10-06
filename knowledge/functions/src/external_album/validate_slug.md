---
type: Rust Function
title: validate_slug
resource: src/external_album.rs#L108-L121
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/windows_reserved
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/slug
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/list_albums
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn validate_slug(name: &str) -> Result<()>`

# Calls

- [windows_reserved](../../../functions/src/external_album/windows_reserved.md)

# Called by

- [slug](../../../functions/src/external_album/slug.md)
- [paths](../../../functions/src/external_album/paths.md)
- [list_albums](../../../functions/src/external_album/list_albums.md)