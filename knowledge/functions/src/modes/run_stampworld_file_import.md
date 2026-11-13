---
type: Rust Function
title: run_stampworld_file_import
resource: src/modes.rs#L1450-L1486
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/modes/canonical_album_dir_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/import_stampworld_file
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/stampworld_pdf_cli/cli_import_targets_explicit_db_instead_of_output_only_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run_stampworld_file_import(cli: &crate::cli::Cli) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [canonical_album_dir_name](../../../functions/src/modes/canonical_album_dir_name.md)
- [import_stampworld_file](../../../functions/src/review_ui/pdf_parser/import_stampworld_file.md)

# Called by

- [main](../../../functions/src/main.md)
- [run_scraping_mode](../../../functions/src/modes/run_scraping_mode.md)
- [cli_import_targets_explicit_db_instead_of_output_only_mode](../../../functions/tests/stampworld_pdf_cli/cli_import_targets_explicit_db_instead_of_output_only_mode.md)