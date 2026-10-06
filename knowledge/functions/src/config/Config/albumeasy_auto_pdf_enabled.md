---
type: Rust Method
title: albumeasy_auto_pdf_enabled
resource: src/config.rs#L1626-L1628
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generated_artifacts_from_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/generate_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn albumeasy_auto_pdf_enabled(&self) -> bool`

# Called by

- [generated_artifacts_from_path](../../../../functions/src/album_globals/generated_artifacts_from_path.md)
- [generate_artifacts](../../../../functions/src/external_album/generate_artifacts.md)
- [main](../../../../functions/src/main.md)