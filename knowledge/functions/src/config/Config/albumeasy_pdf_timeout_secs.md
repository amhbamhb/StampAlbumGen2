---
type: Rust Method
title: albumeasy_pdf_timeout_secs
resource: src/config.rs#L1606-L1609
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generated_artifacts_from_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn albumeasy_pdf_timeout_secs(&self) -> u32`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [generated_artifacts_from_path](../../../../functions/src/album_globals/generated_artifacts_from_path.md)