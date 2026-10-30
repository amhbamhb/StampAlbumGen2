---
type: Rust Function
title: list_albums
resource: src/external_album.rs#L656-L698
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/validate_slug
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/no_links
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/is_imported
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/details
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn list_albums(root: &Path) -> Result<Vec<AlbumSummary>>`

# Calls

- [err](../../../functions/src/external_album/err.md)
- [validate_slug](../../../functions/src/external_album/validate_slug.md)
- [no_links](../../../functions/src/external_album/no_links.md)
- [is_imported](../../../functions/src/external_album/is_imported.md)
- [details](../../../functions/src/external_album/details.md)