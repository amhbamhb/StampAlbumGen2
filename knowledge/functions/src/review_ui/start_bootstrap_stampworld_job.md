---
type: Rust Function
title: start_bootstrap_stampworld_job
resource: src/review_ui.rs#L881-L1132
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stampworld/country_url/load_country_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_stampworld_username
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_stampworld_password
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/resolve_bootstrap_albums_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/stampworld_job_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/build_stampworld_args
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/args
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/set_stampworld_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/release_publication_shell/success
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn start_bootstrap_stampworld_job(req: BootstrapStampWorldRunRequest) -> Result<String, String>`

# Calls

- [load_country_url](../../../functions/src/stampworld/country_url/load_country_url.md)
- [effective_stampworld_username](../../../functions/src/config/Config/effective_stampworld_username.md)
- [effective_stampworld_password](../../../functions/src/config/Config/effective_stampworld_password.md)
- [resolve_bootstrap_albums_root](../../../functions/src/review_ui/resolve_bootstrap_albums_root.md)
- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [stampworld_job_state](../../../functions/src/review_ui/stampworld_job_state.md)
- [build_stampworld_args](../../../functions/src/review_ui/build_stampworld_args.md)
- [log_runtime_event](../../../functions/src/review_ui/log_runtime_event.md)
- [args](../../../functions/tests/native_pdf_page_settings/args.md)
- [set_stampworld_state](../../../functions/src/review_ui/set_stampworld_state.md)
- [success](../../../functions/tests/release_publication_shell/success.md)

# Called by

- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)