---
type: Rust Module
title: src
resource: src/main.rs#L1-L617
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/clap-commandfactory-parser
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-cli-cli-config-modes-run-database-mode-run-scraping-mode-review-ui-log-runtime-event-run-review-ui-run-review-ui-bootstrap-matcherpayloadoptions-reviewuibootstrapoutcome-reviewuirunoutcome-reviewuiselected-setup-align-process-working-dir-to-runtime-layout-ensure-linux-desktop-integration-ensure-windows-desktop-integration-install-linux-desktop-integration-setup-logging-startup-probe-path-uninstall-linux-desktop-integration-validate-os-write-startup-probe-write-startup-probe-snapshot-validation-print-validation-error-validate-source-validate-user-input-validate-year-range
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-isterminal
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [main](../functions/src/main.md)
- [ReviewUiLaunchState](../classes/src/ReviewUiLaunchState.md)

# Imports

- `clap::{CommandFactory, Parser}`
- `stamp_album_gen2::{
    cli::Cli,
    config,
    modes::{run_database_mode, run_scraping_mode},
    review_ui::{
        log_runtime_event, run_review_ui, run_review_ui_bootstrap, MatcherPayloadOptions,
        ReviewUiBootstrapOutcome, ReviewUiRunOutcome, ReviewUiSelected,
    },
    setup::{
        align_process_working_dir_to_runtime_layout, ensure_linux_desktop_integration,
        ensure_windows_desktop_integration, install_linux_desktop_integration, setup_logging,
        startup_probe_path, uninstall_linux_desktop_integration, validate_os, write_startup_probe,
        write_startup_probe_snapshot,
    },
    validation::{
        print_validation_error, validate_source, validate_user_input, validate_year_range,
    },
}`
- `std::io::IsTerminal`

# Member of

- [stamp_album_gen2](../packages/stamp_album_gen2.md)