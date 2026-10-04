---
type: Rust Function
title: detect_duplicate_keys
resource: src/config.rs#L1714-L1783
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/duplicate_detection_ignores_same_key_in_different_sections
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/duplicate_detection_finds_duplicates_in_same_section
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn detect_duplicate_keys(content: &str) -> Option<Vec<String>>`

# Called by

- [load_config](../../../functions/src/config/load_config.md)
- [duplicate_detection_ignores_same_key_in_different_sections](../../../functions/src/config/duplicate_detection_ignores_same_key_in_different_sections.md)
- [duplicate_detection_finds_duplicates_in_same_section](../../../functions/src/config/duplicate_detection_finds_duplicates_in_same_section.md)