---
type: Rust Function
title: pick_folder_via_dialog
resource: src/review_ui.rs#L2020-L2022
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/release_publication_shell/success
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pick_folder_via_dialog(title: &str, current: &str) -> Result<Option<String>, String>`

# Calls

- [success](../../../functions/tests/release_publication_shell/success.md)