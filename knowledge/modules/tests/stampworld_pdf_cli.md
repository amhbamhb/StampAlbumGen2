---
type: Rust Module
title: stampworld_pdf_cli
resource: tests/stampworld_pdf_cli.rs#L1-L97
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/clap-parser
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-cli-cli-modes-run-stampworld-file-import
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [cli_import_targets_explicit_db_instead_of_output_only_mode](../../functions/tests/stampworld_pdf_cli/cli_import_targets_explicit_db_instead_of_output_only_mode.md)
- [cli_import_requires_a_file_and_destination_country_before_writing](../../functions/tests/stampworld_pdf_cli/cli_import_requires_a_file_and_destination_country_before_writing.md)
- [executable_routes_pdf_with_db_path_to_import_not_album_generation](../../functions/tests/stampworld_pdf_cli/executable_routes_pdf_with_db_path_to_import_not_album_generation.md)

# Imports

- `clap::Parser`
- `stamp_album_gen2::{cli::Cli, modes::run_stampworld_file_import}`
- `std::path::Path`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)