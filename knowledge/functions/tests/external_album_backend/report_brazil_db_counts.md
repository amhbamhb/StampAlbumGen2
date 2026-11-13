---
type: Rust Function
title: report_brazil_db_counts
resource: tests/external_album_backend.rs#L1256-L1291
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/external_album_backend/brazil_database_roundtrip_and_repeated_placements
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/brazil_real_albumeasy_import_approval_and_page_edit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn report_brazil_db_counts(conn: &Connection, path: &Path)`

# Calls

- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [brazil_database_roundtrip_and_repeated_placements](../../../functions/tests/external_album_backend/brazil_database_roundtrip_and_repeated_placements.md)
- [brazil_real_albumeasy_import_approval_and_page_edit](../../../functions/tests/external_album_backend/brazil_real_albumeasy_import_approval_and_page_edit.md)