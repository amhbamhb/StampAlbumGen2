---
type: Rust Function
title: paths
resource: src/external_album.rs#L546-L556
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/validate_slug
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/no_links
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn paths(root: &Path, name: &str) -> Result<(PathBuf, PathBuf)>`

# Calls

- [validate_slug](../../../functions/src/external_album/validate_slug.md)
- [err](../../../functions/src/external_album/err.md)
- [no_links](../../../functions/src/external_album/no_links.md)

# Called by

- [open](../../../functions/src/external_album/open.md)