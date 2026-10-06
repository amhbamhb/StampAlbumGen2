---
type: Rust Method
title: albumeasy_pdf_timeout_secs
resource: src/config.rs#L1631-L1634
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
  - target: functions/src/external_album/generate_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/renderer
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn albumeasy_pdf_timeout_secs(&self) -> u32`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [generated_artifacts_from_path](../../../../functions/src/album_globals/generated_artifacts_from_path.md)
- [generate_artifacts](../../../../functions/src/external_album/generate_artifacts.md)
- [renderer](../../../../functions/src/review_ui/external_album_api/renderer.md)