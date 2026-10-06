---
type: Rust Function
title: source_checksums
resource: tests/external_album_backend.rs#L1270-L1304
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/external_album_backend/brazil_real_albumeasy_import_approval_and_page_edit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn source_checksums(source: &Path) -> std::collections::BTreeMap<PathBuf, String>`

# Calls

- [open](../../../functions/src/external_album/open.md)

# Called by

- [brazil_real_albumeasy_import_approval_and_page_edit](../../../functions/tests/external_album_backend/brazil_real_albumeasy_import_approval_and_page_edit.md)