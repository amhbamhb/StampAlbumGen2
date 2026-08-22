---
type: Rust Module
title: cli_args
resource: tests/cli_args.rs#L1-L49
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/assert-cmd-prelude
    resolved_by: tree-sitter
    confidence: exact
  - target: external/predicates-prelude
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-process-command
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [help_shows_options](../../functions/tests/cli_args/help_shows_options.md)
- [no_args_prints_guidance_and_help](../../functions/tests/cli_args/no_args_prints_guidance_and_help.md)
- [scrape_mode_missing_site_name_reports_error](../../functions/tests/cli_args/scrape_mode_missing_site_name_reports_error.md)
- [db_path_and_album_name_runs_ok](../../functions/tests/cli_args/db_path_and_album_name_runs_ok.md)

# Imports

- `assert_cmd::prelude::*`
- `predicates::prelude::*`
- `std::process::Command`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)