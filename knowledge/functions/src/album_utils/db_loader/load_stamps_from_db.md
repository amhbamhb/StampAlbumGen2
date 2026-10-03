---
type: Rust Function
title: load_stamps_from_db
resource: src/album_utils/db_loader.rs#L2-L79
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/formatting/format_stamps_issued
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load_stamps_from_db(db_path: &str) -> Vec<crate::db_utils::StampStub>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [format_stamps_issued](../../../../functions/src/db_utils/formatting/format_stamps_issued.md)