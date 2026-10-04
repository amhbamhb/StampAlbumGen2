---
type: Rust Function
title: ensure_manual_import_files
resource: src/colnect/template.rs#L8-L40
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/manual_import_template/creates_missing_files_and_preserves_existing_copies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/manual_import_template/reports_conflicting_template_path_without_overwriting_it
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/manual_import_template/manual_rows_parse_and_replace_only_the_same_staged_identity
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ensure_manual_import_files(runtime_dir: &Path) -> io::Result<Vec<PathBuf>>`

# Called by

- [main](../../../../functions/src/main.md)
- [creates_missing_files_and_preserves_existing_copies](../../../../functions/tests/manual_import_template/creates_missing_files_and_preserves_existing_copies.md)
- [reports_conflicting_template_path_without_overwriting_it](../../../../functions/tests/manual_import_template/reports_conflicting_template_path_without_overwriting_it.md)
- [manual_rows_parse_and_replace_only_the_same_staged_identity](../../../../functions/tests/manual_import_template/manual_rows_parse_and_replace_only_the_same_staged_identity.md)