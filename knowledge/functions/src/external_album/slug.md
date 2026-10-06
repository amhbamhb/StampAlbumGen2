---
type: Rust Function
title: slug
resource: src/external_album.rs#L88-L106
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/validate_slug
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn slug(name: &str) -> Result<String>`

# Calls

- [validate_slug](../../../functions/src/external_album/validate_slug.md)

# Called by

- [import_album](../../../functions/src/external_album/import_album.md)