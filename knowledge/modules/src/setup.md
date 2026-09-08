---
type: Rust Module
title: setup
resource: src/setup.rs#L1-L916
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-types-ostype
    resolved_by: tree-sitter
    confidence: exact
  - target: external/flexi-logger-cleanup-criterion-filespec-logger-naming
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs-openoptions
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-process-command-stdio
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-os-unix-fs-permissionsext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-align-process-working-dir-to-runtime-layout-find-runtime-root-from-looks-like-runtime-root
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [LinuxDesktopIntegrationReport](../../classes/src/setup/LinuxDesktopIntegrationReport.md)
- [WindowsDesktopIntegrationReport](../../classes/src/setup/WindowsDesktopIntegrationReport.md)
- [SetupError](../../classes/src/setup/SetupError.md)
- [fmt](../../functions/src/setup/SetupError/std-fmt-display/fmt.md)
- [from](../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [get_operating_system_type](../../functions/src/setup/get_operating_system_type.md)
- [setup_logging](../../functions/src/setup/setup_logging.md)
- [startup_probe_path](../../functions/src/setup/startup_probe_path.md)
- [write_startup_probe](../../functions/src/setup/write_startup_probe.md)
- [write_startup_probe_snapshot](../../functions/src/setup/write_startup_probe_snapshot.md)
- [validate_os](../../functions/src/setup/validate_os.md)
- [is_cargo_build_binary](../../functions/src/setup/is_cargo_build_binary.md)
- [desktop_exec_path](../../functions/src/setup/desktop_exec_path.md)
- [desktop_working_dir](../../functions/src/setup/desktop_working_dir.md)
- [looks_like_runtime_root](../../functions/src/setup/looks_like_runtime_root.md)
- [find_runtime_root_from](../../functions/src/setup/find_runtime_root_from.md)
- [fallback_runtime_root_from](../../functions/src/setup/fallback_runtime_root_from.md)
- [runtime_root_from_executable_path](../../functions/src/setup/runtime_root_from_executable_path.md)
- [align_process_working_dir_to_runtime_layout](../../functions/src/setup/align_process_working_dir_to_runtime_layout.md)
- [desktop_exec_value](../../functions/src/setup/desktop_exec_value.md)
- [command_exists](../../functions/src/setup/command_exists.md)
- [refresh_linux_desktop_caches](../../functions/src/setup/refresh_linux_desktop_caches.md)
- [user_home_dir](../../functions/src/setup/user_home_dir.md)
- [mark_desktop_file_trusted](../../functions/src/setup/mark_desktop_file_trusted.md)
- [app_state_dir](../../functions/src/setup/app_state_dir.md)
- [desktop_launcher_disable_marker_path](../../functions/src/setup/desktop_launcher_disable_marker_path.md)
- [is_desktop_launcher_auto_install_disabled](../../functions/src/setup/is_desktop_launcher_auto_install_disabled.md)
- [set_desktop_launcher_auto_install_disabled](../../functions/src/setup/set_desktop_launcher_auto_install_disabled.md)
- [bundled_icon_png](../../functions/src/setup/bundled_icon_png.md)
- [bundled_windows_icon_ico](../../functions/src/setup/bundled_windows_icon_ico.md)
- [escape_powershell_single_quoted](../../functions/src/setup/escape_powershell_single_quoted.md)
- [query_windows_desktop_dir](../../functions/src/setup/query_windows_desktop_dir.md)
- [query_windows_programs_dir](../../functions/src/setup/query_windows_programs_dir.md)
- [create_windows_shortcut](../../functions/src/setup/create_windows_shortcut.md)
- [ensure_windows_desktop_integration](../../functions/src/setup/ensure_windows_desktop_integration.md)
- [ensure_linux_desktop_integration](../../functions/src/setup/ensure_linux_desktop_integration.md)
- [runtime_root_detects_expected_markers](../../functions/src/setup/runtime_root_detects_expected_markers.md)
- [runtime_root_search_prefers_exe_directory_markers](../../functions/src/setup/runtime_root_search_prefers_exe_directory_markers.md)
- [align_runtime_dir_noops_when_already_root](../../functions/src/setup/align_runtime_dir_noops_when_already_root.md)
- [uninstall_linux_desktop_integration](../../functions/src/setup/uninstall_linux_desktop_integration.md)
- [install_linux_desktop_integration](../../functions/src/setup/install_linux_desktop_integration.md)

# Imports

- `crate::types::OSType`
- `flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming}`
- `std::fs::OpenOptions`
- `std::io::Write`
- `std::path::{Path, PathBuf}`
- `std::process::{Command, Stdio}`
- `std::os::unix::fs::PermissionsExt`
- `super::{
        align_process_working_dir_to_runtime_layout, find_runtime_root_from,
        looks_like_runtime_root,
    }`
- `tempfile::tempdir`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)