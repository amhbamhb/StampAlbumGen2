---
type: JavaScript Function
title: success
resource: tests/release_publication_shell.cjs#L17-L20
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/maybe_generate_album_pdf_with_settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/dimensions/calculate_stamp_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/try_rasterize_svg_to_png
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/start_bootstrap_stampworld_job
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_folder_via_dialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_albumeasy_cli_path_via_dialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_and_copy_colnect_file_to_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/command_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/query_windows_desktop_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/query_windows_programs_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/create_windows_shortcut
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/tool_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_args/help_shows_options
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_args/db_path_and_album_name_runs_ok
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_option_validation/test_ensure_binary_built
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_init_tests/test_first_run_without_terminal_creates_defaults
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/release_publication_shell/fixture
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function success(result)`

# Called by

- [maybe_generate_album_pdf_with_settings](../../../functions/src/album_globals/maybe_generate_album_pdf_with_settings.md)
- [calculate_stamp_dimensions](../../../functions/src/album_utils/dimensions/calculate_stamp_dimensions.md)
- [render](../../../functions/src/external_album/render.md)
- [try_rasterize_svg_to_png](../../../functions/src/flagdb/try_rasterize_svg_to_png.md)
- [start_bootstrap_stampworld_job](../../../functions/src/review_ui/start_bootstrap_stampworld_job.md)
- [pick_folder_via_dialog](../../../functions/src/review_ui/pick_folder_via_dialog.md)
- [pick_albumeasy_cli_path_via_dialog](../../../functions/src/review_ui/pick_albumeasy_cli_path_via_dialog.md)
- [pick_and_copy_colnect_file_to_dir](../../../functions/src/review_ui/pick_and_copy_colnect_file_to_dir.md)
- [command_exists](../../../functions/src/setup/command_exists.md)
- [query_windows_desktop_dir](../../../functions/src/setup/query_windows_desktop_dir.md)
- [query_windows_programs_dir](../../../functions/src/setup/query_windows_programs_dir.md)
- [create_windows_shortcut](../../../functions/src/setup/create_windows_shortcut.md)
- [tool_output](../../../functions/tests/albumeasy_baseline/tool_output.md)
- [help_shows_options](../../../functions/tests/cli_args/help_shows_options.md)
- [db_path_and_album_name_runs_ok](../../../functions/tests/cli_args/db_path_and_album_name_runs_ok.md)
- [test_ensure_binary_built](../../../functions/tests/cli_option_validation/test_ensure_binary_built.md)
- [test_first_run_without_terminal_creates_defaults](../../../functions/tests/config_init_tests/test_first_run_without_terminal_creates_defaults.md)
- [fixture](../../../functions/tests/release_publication_shell/fixture.md)