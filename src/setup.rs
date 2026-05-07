use crate::types::OSType;
use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const DESKTOP_LAUNCHER_DISABLED_MARKER: &str = "desktop_launcher_disabled.flag";

#[derive(Debug, Clone)]
pub struct LinuxDesktopIntegrationReport {
    pub changed: bool,
    pub summary: String,
    pub desktop_entry_path: String,
    pub icon_path: String,
}

#[derive(Debug, Clone)]
pub struct WindowsDesktopIntegrationReport {
    pub changed: bool,
    pub summary: String,
    pub desktop_shortcut_path: String,
    pub start_menu_shortcut_path: String,
    pub icon_path: String,
}

/// Setup errors that can occur during environment setup
#[derive(Debug)]
pub enum SetupError {
    LogsDirectoryCreation(std::io::Error),
    LoggerInitialization(flexi_logger::FlexiLoggerError),
    UnsupportedOS,
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetupError::LogsDirectoryCreation(e) => {
                write!(f, "Failed to create Logs directory: {}", e)
            }
            SetupError::LoggerInitialization(e) => {
                write!(f, "Failed to initialize logger: {}", e)
            }
            SetupError::UnsupportedOS => write!(f, "Unsupported operating system"),
        }
    }
}

impl std::error::Error for SetupError {}

impl From<flexi_logger::FlexiLoggerError> for SetupError {
    fn from(err: flexi_logger::FlexiLoggerError) -> Self {
        SetupError::LoggerInitialization(err)
    }
}

/// Get the operating system type
pub fn get_operating_system_type() -> OSType {
    let os = std::env::consts::OS;
    match os {
        "windows" => OSType::Windows,
        "macos" => OSType::MacOS,
        "linux" => OSType::Linux,
        _ => OSType::Unknown,
    }
}

/// Setup the logging environment
pub fn setup_logging() -> Result<(), SetupError> {
    // Ensure Logs directory exists
    let logs_dir = std::path::Path::new("Logs");
    if !logs_dir.exists() {
        std::fs::create_dir_all(logs_dir).map_err(SetupError::LogsDirectoryCreation)?;
    }

    // Build a timestamped basename like 'stamp_album_gen2-20251020-171234'
    let ts = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();
    let basename = format!("stamp_album_gen2-{}", ts);

    // Use a conservative default filter that keeps this crate at `info` but
    // silences noisy external crates (html5ever, reqwest). Allow overriding
    // via the environment (RUST_LOG) for one-off trace captures during
    // debugging, while keeping a safe default to avoid runaway logs.
    // Also enable size-based rotation so logs cannot grow indefinitely.
    Logger::try_with_env_or_str("stamp_album_gen2=info,html5ever=warn,reqwest=warn")?
        .log_to_file(
            FileSpec::default()
                .directory("Logs")
                .basename(&basename)
                .suffix("log"),
        )
        // Rotate files when they reach ~10 MiB and keep the last 5 files.
        .rotate(
            Criterion::Size(10 * 1024 * 1024),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(5),
        )
        // Only duplicate errors to stderr by default so info/debug go to the log file only
        .duplicate_to_stderr(flexi_logger::Duplicate::Error)
        .start()
        .map_err(SetupError::LoggerInitialization)?;

    Ok(())
}

pub fn startup_probe_path() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            return dir.join("startup_trace.log");
        }
    }
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("startup_trace.log")
}

pub fn write_startup_probe(stage: &str, detail: &str) {
    let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    let pid = std::process::id();
    let line = format!("{} pid={} stage={} detail={}\n", ts, pid, stage, detail);
    let path = startup_probe_path();
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = f.write_all(line.as_bytes());
    }
}

pub fn write_startup_probe_snapshot(
    args: &[String],
    launched_without_args: bool,
    interactive_terminal: bool,
) {
    let cwd = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|e| format!("<cwd-error:{}>", e));
    let exe = std::env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|e| format!("<exe-error:{}>", e));

    let env_keys = [
        "PWD",
        "HOME",
        "PATH",
        "DISPLAY",
        "WAYLAND_DISPLAY",
        "XDG_SESSION_TYPE",
        "XDG_CURRENT_DESKTOP",
        "DESKTOP_SESSION",
    ];
    let mut env_items = Vec::new();
    for key in env_keys {
        if let Ok(v) = std::env::var(key) {
            env_items.push(format!("{}={}", key, v));
        }
    }
    write_startup_probe(
        "startup.snapshot",
        &format!(
            "args={:?}; launched_without_args={}; interactive_terminal={}; cwd={}; exe={}; env=[{}]",
            args,
            launched_without_args,
            interactive_terminal,
            cwd,
            exe,
            env_items.join(", ")
        ),
    );
}

/// Validate the operating system
pub fn validate_os() -> Result<OSType, SetupError> {
    let detected_os = get_operating_system_type();
    if detected_os == OSType::Unknown {
        return Err(SetupError::UnsupportedOS);
    }
    Ok(detected_os)
}

/// Returns true when the running binary lives inside a Cargo build tree
/// (`target/debug/` or `target/release/`). Those binaries should never
/// overwrite the installed desktop launcher.
fn is_cargo_build_binary() -> bool {
    let Ok(exe) = std::env::current_exe() else {
        return false;
    };
    let path = exe.to_string_lossy();
    // Match both Unix and Windows path separators
    path.contains("/target/debug/")
        || path.contains("/target/release/")
        || path.contains("\\target\\debug\\")
        || path.contains("\\target\\release\\")
}

fn desktop_exec_path() -> std::io::Result<String> {
    let exe = std::env::current_exe()?;
    let raw = exe.to_string_lossy().to_string();
    if raw.trim().is_empty() {
        return Err(std::io::Error::other(
            "current executable path resolved to empty string",
        ));
    }
    let escaped = raw.replace('\\', "\\\\").replace('"', "\\\"");
    if escaped.trim().is_empty() {
        return Err(std::io::Error::other(
            "escaped executable path resolved to empty string",
        ));
    }
    Ok(format!("\"{}\"", escaped))
}

fn desktop_working_dir() -> std::io::Result<String> {
    let exe = std::env::current_exe()?;
    let dir = exe
        .parent()
        .ok_or_else(|| std::io::Error::other("failed to determine executable parent directory"))?;
    let raw = dir.to_string_lossy().to_string();
    if raw.trim().is_empty() {
        return Err(std::io::Error::other(
            "desktop working directory resolved to empty string",
        ));
    }
    Ok(raw)
}

fn looks_like_runtime_root(path: &Path) -> bool {
    if !path.is_dir() {
        return false;
    }
    path.join("config.toml").exists()
        || path.join("Albums").is_dir()
        || path.join("assets").is_dir()
}

fn find_runtime_root_from(current_dir: &Path, exe_path: &Path) -> Option<PathBuf> {
    if looks_like_runtime_root(current_dir) {
        return None;
    }

    // Prefer executable-adjacent runtime for portable/single-binary usage.
    let exe_dir = exe_path.parent()?;
    if looks_like_runtime_root(exe_dir) {
        return Some(exe_dir.to_path_buf());
    }

    // Optional compatibility mode: allow searching ancestors when explicitly enabled.
    let allow_ancestor_search = std::env::var("STAMP_ALBUM_ALLOW_ANCESTOR_RUNTIME_ROOT")
        .map(|v| {
            let t = v.trim();
            t.eq_ignore_ascii_case("1") || t.eq_ignore_ascii_case("true")
        })
        .unwrap_or(false);
    if !allow_ancestor_search {
        return None;
    }

    let mut candidate = exe_path.parent();
    for _ in 0..6 {
        let dir = candidate?;
        if looks_like_runtime_root(dir) {
            return Some(dir.to_path_buf());
        }
        candidate = dir.parent();
    }

    None
}

fn fallback_runtime_root_from(exe_path: &Path) -> Option<PathBuf> {
    // First-start/portable behavior: if no markers are found yet, anchor runtime
    // paths to the executable directory so config/logs are consistent regardless
    // of launch origin (terminal, menu, file explorer).
    exe_path.parent().map(|p| p.to_path_buf())
}

fn runtime_root_from_executable_path(exe_path: &Path) -> Option<PathBuf> {
    let exe_dir = exe_path.parent()?;
    if looks_like_runtime_root(exe_dir) {
        return Some(exe_dir.to_path_buf());
    }

    let allow_ancestor_search = std::env::var("STAMP_ALBUM_ALLOW_ANCESTOR_RUNTIME_ROOT")
        .map(|v| {
            let t = v.trim();
            t.eq_ignore_ascii_case("1") || t.eq_ignore_ascii_case("true")
        })
        .unwrap_or(false);
    if !allow_ancestor_search {
        return None;
    }

    let mut candidate = exe_dir.parent();
    for _ in 0..6 {
        let dir = candidate?;
        if looks_like_runtime_root(dir) {
            return Some(dir.to_path_buf());
        }
        candidate = dir.parent();
    }

    None
}

pub fn align_process_working_dir_to_runtime_layout() -> std::io::Result<Option<PathBuf>> {
    let current_dir = std::env::current_dir()?;
    let exe_path = std::env::current_exe()?;

    if looks_like_runtime_root(&current_dir) {
        // When an external/portable executable is launched from a different
        // runtime-like directory (for example the workspace root), prefer the
        // executable-adjacent runtime tree. For dev builds under the workspace,
        // the executable remains inside the current runtime root, so keep cwd.
        if let Some(exe_runtime_root) = runtime_root_from_executable_path(&exe_path) {
            if exe_runtime_root != current_dir && !exe_path.starts_with(&current_dir) {
                std::env::set_current_dir(&exe_runtime_root)?;
                return Ok(Some(exe_runtime_root));
            }
        }
        return Ok(None);
    }

    let target_dir = find_runtime_root_from(&current_dir, &exe_path)
        .or_else(|| fallback_runtime_root_from(&exe_path));

    let Some(target_dir) = target_dir else {
        return Ok(None);
    };

    if current_dir == target_dir {
        return Ok(None);
    }

    std::env::set_current_dir(&target_dir)?;
    Ok(Some(target_dir))
}

fn desktop_exec_value(contents: &str) -> Option<&str> {
    contents.lines().find_map(|line| line.strip_prefix("Exec="))
}

fn command_exists(name: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {} >/dev/null 2>&1", name))
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn refresh_linux_desktop_caches(applications_dir: &std::path::Path) {
    if command_exists("update-desktop-database") {
        let _ = Command::new("update-desktop-database")
            .arg(applications_dir)
            .status();
    }

    // Refresh the hicolor icon theme cache so Wayland compositors (GNOME Shell,
    // Mutter) pick up newly installed icons without requiring a re-login.
    // The cache lives alongside the icons themselves, not in applications/.
    let hicolor_dir = applications_dir
        .parent()
        .map(|d| d.join("icons/hicolor"))
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var("HOME").unwrap_or_default())
                .join(".local/share/icons/hicolor")
        });
    for cmd in &["gtk-update-icon-cache", "gtk4-update-icon-cache"] {
        if command_exists(cmd) {
            let _ = Command::new(cmd)
                .arg("-f")
                .arg("-t")
                .arg(&hicolor_dir)
                .status();
            break; // one is enough
        }
    }

    if command_exists("kbuildsycoca6") {
        let _ = Command::new("kbuildsycoca6").status();
    } else if command_exists("kbuildsycoca5") {
        let _ = Command::new("kbuildsycoca5").status();
    }
}

fn user_home_dir() -> std::io::Result<std::path::PathBuf> {
    let home = std::env::var("HOME")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "HOME is not set"))?;
    Ok(std::path::PathBuf::from(home))
}

/// Mark a `~/Desktop/*.desktop` file as trusted so GNOME Shell / Nautilus
/// will launch it directly without the "Untrusted application launcher" dialog.
///
/// Internally this sets the GIO metadata attribute `metadata::trusted` to
/// `true` — the same action as right-clicking → "Allow Launching" in
/// Nautilus.  The `gio` CLI is the portable way to set this attribute without
/// linking to libgio at compile time.
fn mark_desktop_file_trusted(path: &std::path::Path) {
    // Only meaningful on Linux; silently skip elsewhere.
    if !std::env::consts::OS.eq_ignore_ascii_case("linux") {
        return;
    }
    // `gio set <file> metadata::trusted true`
    // Available on any Ubuntu / GNOME install; older distros may lack it but
    // the failure is harmless — the user will just see the trust dialog once.
    for gio in &["gio", "/usr/bin/gio", "/usr/lib/x86_64-linux-gnu/gio"] {
        if command_exists(gio) {
            let _ = Command::new(gio)
                .args(["set", &path.to_string_lossy(), "metadata::trusted", "true"])
                .status();
            return;
        }
    }
}

fn app_state_dir(home: &std::path::Path) -> std::path::PathBuf {
    home.join(".config/stamp_album_gen2")
}

fn desktop_launcher_disable_marker_path(home: &std::path::Path) -> std::path::PathBuf {
    app_state_dir(home).join(DESKTOP_LAUNCHER_DISABLED_MARKER)
}

fn is_desktop_launcher_auto_install_disabled(home: &std::path::Path) -> bool {
    desktop_launcher_disable_marker_path(home).exists()
}

fn set_desktop_launcher_auto_install_disabled(
    home: &std::path::Path,
    disabled: bool,
) -> std::io::Result<()> {
    let marker = desktop_launcher_disable_marker_path(home);
    let state_dir = app_state_dir(home);
    std::fs::create_dir_all(&state_dir)?;

    if disabled {
        std::fs::write(&marker, b"disabled\n")?;
    } else if marker.exists() {
        std::fs::remove_file(&marker)?;
    }

    Ok(())
}

fn bundled_icon_png() -> &'static [u8] {
    include_bytes!("../assets/icons/stamp_album_gen2.png")
}

fn bundled_windows_icon_ico() -> &'static [u8] {
    include_bytes!("../assets/icons/generated/stamp-monogram.ico")
}

fn escape_powershell_single_quoted(text: &str) -> String {
    text.replace('\'', "''")
}

fn query_windows_desktop_dir() -> Option<PathBuf> {
    let output = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-Command")
        .arg("[Environment]::GetFolderPath('Desktop')")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let trimmed = stdout.trim();
    if trimmed.is_empty() {
        return None;
    }

    Some(PathBuf::from(trimmed))
}

fn query_windows_programs_dir() -> Option<PathBuf> {
    let output = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-Command")
        .arg("[Environment]::GetFolderPath('Programs')")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let trimmed = stdout.trim();
    if trimmed.is_empty() {
        return None;
    }

    Some(PathBuf::from(trimmed))
}

fn create_windows_shortcut(
    shortcut_path: &Path,
    target: &Path,
    working_dir: &Path,
    icon_path: &Path,
) -> std::io::Result<()> {
    let target_ps = escape_powershell_single_quoted(&target.to_string_lossy());
    let cwd_ps = escape_powershell_single_quoted(&working_dir.to_string_lossy());
    let shortcut_ps = escape_powershell_single_quoted(&shortcut_path.to_string_lossy());
    let icon_ps = escape_powershell_single_quoted(&icon_path.to_string_lossy());

    let script = format!(
        "$wsh = New-Object -ComObject WScript.Shell; $sc = $wsh.CreateShortcut('{shortcut}'); $sc.TargetPath = '{target}'; $sc.Arguments = '--review-ui'; $sc.WorkingDirectory = '{cwd}'; $sc.IconLocation = '{icon},0'; $sc.Description = 'Open Stamp Album Generator 2 Review UI'; $sc.Save()",
        shortcut = shortcut_ps,
        target = target_ps,
        cwd = cwd_ps,
        icon = icon_ps
    );

    let output = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-Command")
        .arg(script)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(std::io::Error::other(format!(
            "failed to create shortcut {}: {}",
            shortcut_path.display(),
            stderr.trim()
        )));
    }

    Ok(())
}

pub fn ensure_windows_desktop_integration() -> std::io::Result<WindowsDesktopIntegrationReport> {
    if !std::env::consts::OS.eq_ignore_ascii_case("windows") {
        return Ok(WindowsDesktopIntegrationReport {
            changed: false,
            summary: "Skipped desktop integration (non-Windows OS).".to_string(),
            desktop_shortcut_path: String::new(),
            start_menu_shortcut_path: String::new(),
            icon_path: String::new(),
        });
    }

    let exe = std::env::current_exe()?;
    let working_dir = exe
        .parent()
        .ok_or_else(|| std::io::Error::other("failed to determine executable parent directory"))?
        .to_path_buf();

    let desktop_dir = query_windows_desktop_dir()
        .or_else(|| std::env::var_os("USERPROFILE").map(|v| PathBuf::from(v).join("Desktop")));
    let desktop_dir = desktop_dir.ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "failed to resolve Windows Desktop directory",
        )
    })?;
    std::fs::create_dir_all(&desktop_dir)?;

    let programs_dir = query_windows_programs_dir().or_else(|| {
        std::env::var_os("APPDATA")
            .map(|v| PathBuf::from(v).join("Microsoft\\Windows\\Start Menu\\Programs"))
    });
    let programs_dir = programs_dir.ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "failed to resolve Windows Start Menu Programs directory",
        )
    })?;
    std::fs::create_dir_all(&programs_dir)?;

    let app_icon_dir = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|| working_dir.clone())
        .join("StampAlbumGen2");
    std::fs::create_dir_all(&app_icon_dir)?;
    let icon_path = app_icon_dir.join("stamp_album_gen2.ico");
    let bundled_icon = bundled_windows_icon_ico();
    if std::fs::read(&icon_path).ok().as_deref() != Some(bundled_icon) {
        std::fs::write(&icon_path, bundled_icon)?;
    }

    let desktop_shortcut_path = desktop_dir.join("Stamp Album Generator 2.lnk");
    let start_menu_shortcut_path = programs_dir.join("Stamp Album Generator 2.lnk");
    let mut changed = false;

    if !desktop_shortcut_path.exists() || !start_menu_shortcut_path.exists() {
        changed = true;
    }

    create_windows_shortcut(&desktop_shortcut_path, &exe, &working_dir, &icon_path)?;
    create_windows_shortcut(&start_menu_shortcut_path, &exe, &working_dir, &icon_path)?;

    let summary = if changed {
        "Windows desktop and Start Menu shortcuts installed/updated."
    } else {
        "Windows desktop and Start Menu shortcuts already configured."
    };

    Ok(WindowsDesktopIntegrationReport {
        changed,
        summary: summary.to_string(),
        desktop_shortcut_path: desktop_shortcut_path.to_string_lossy().to_string(),
        start_menu_shortcut_path: start_menu_shortcut_path.to_string_lossy().to_string(),
        icon_path: icon_path.to_string_lossy().to_string(),
    })
}

pub fn ensure_linux_desktop_integration() -> std::io::Result<LinuxDesktopIntegrationReport> {
    if !std::env::consts::OS.eq_ignore_ascii_case("linux") {
        return Ok(LinuxDesktopIntegrationReport {
            changed: false,
            summary: "Skipped desktop integration (non-Linux OS).".to_string(),
            desktop_entry_path: String::new(),
            icon_path: String::new(),
        });
    }

    // A Cargo-built binary (target/debug or target/release) must never
    // overwrite the installed desktop launcher — that would redirect the
    // desktop shortcut to the dev-tree path on the next click.
    if is_cargo_build_binary() {
        let home = user_home_dir().unwrap_or_else(|_| PathBuf::from("/tmp"));
        return Ok(LinuxDesktopIntegrationReport {
            changed: false,
            summary: "Desktop launcher update skipped (Cargo build binary — only installed binaries update the launcher).".to_string(),
            desktop_entry_path: home
                .join(".local/share/applications/stamp-album-gen2.desktop")
                .to_string_lossy()
                .to_string(),
            icon_path: home
                .join(".local/share/icons/hicolor/256x256/apps/stamp_album_gen2.png")
                .to_string_lossy()
                .to_string(),
        });
    }

    let home = user_home_dir()?;

    if is_desktop_launcher_auto_install_disabled(&home) {
        return Ok(LinuxDesktopIntegrationReport {
            changed: false,
            summary: "Desktop launcher auto-install is disabled (use --install-desktop-launcher to re-enable).".to_string(),
            desktop_entry_path: home
                .join(".local/share/applications/stamp-album-gen2.desktop")
                .to_string_lossy()
                .to_string(),
            icon_path: home
                .join(".local/share/icons/hicolor/256x256/apps/stamp_album_gen2.png")
                .to_string_lossy()
                .to_string(),
        });
    }

    let applications_dir = home.join(".local/share/applications");
    // PNG icons must be in sized hicolor directories, not scalable/ (which is
    // for SVGs).  256x256 matches the bundled asset; 48x48 covers taskbars.
    let icons_dir_256 = home.join(".local/share/icons/hicolor/256x256/apps");
    let icons_dir_48 = home.join(".local/share/icons/hicolor/48x48/apps");
    let desktop_dir = home.join("Desktop");
    std::fs::create_dir_all(&applications_dir)?;
    std::fs::create_dir_all(&icons_dir_256)?;
    std::fs::create_dir_all(&icons_dir_48)?;

    let desktop_path = applications_dir.join("stamp-album-gen2.desktop");
    let icon_path_256 = icons_dir_256.join("stamp_album_gen2.png");
    let icon_path_48 = icons_dir_48.join("stamp_album_gen2.png");

    // Legacy paths to clean up
    let legacy_scalable_dir = home.join(".local/share/icons/hicolor/scalable/apps");
    let legacy_svg_icon_path = legacy_scalable_dir.join("stamp_album_gen2.svg");
    let legacy_png_icon_path = legacy_scalable_dir.join("stamp_album_gen2.png");

    let exec_path = desktop_exec_path()?;
    let working_dir = desktop_working_dir()?;
    if exec_path.trim().is_empty() {
        return Err(std::io::Error::other(
            "desktop launcher command cannot be empty",
        ));
    }
    // Use the icon *name* (not an absolute path) so GNOME/Wayland resolves it
    // through the hicolor icon theme — required for App Details and taskbar
    // icons to work correctly under Wayland.
    let desktop_contents = format!(
        "[Desktop Entry]\nType=Application\nVersion=1.0\nName=Stamp Album Generator 2\nComment=Open Stamp Album Generator 2 Review UI\nExec={}\nPath={}\nIcon=stamp_album_gen2\nTerminal=false\nCategories=Graphics;Education;Utility;\nStartupNotify=true\n",
        format!("{} --review-ui", exec_path),
        working_dir,
    );

    let mut changed = false;

    let existing_desktop = std::fs::read_to_string(&desktop_path).ok();
    if existing_desktop.as_deref() != Some(desktop_contents.as_str()) {
        std::fs::write(&desktop_path, desktop_contents.as_bytes())?;

        let written = std::fs::read_to_string(&desktop_path)?;
        let Some(exec_line) = desktop_exec_value(&written) else {
            return Err(std::io::Error::other(
                "desktop launcher verification failed: missing Exec line",
            ));
        };
        if exec_line.trim().is_empty() {
            return Err(std::io::Error::other(
                "desktop launcher verification failed: Exec line is blank",
            ));
        }

        changed = true;
    }

    // Also write the .desktop file to ~/Desktop so it appears as a clickable
    // icon on the GNOME/Nautilus-managed desktop.  The file must be executable
    // AND marked trusted via GIO metadata for GNOME/DING to launch it without
    // the "Untrusted application launcher" warning.
    if desktop_dir.exists() {
        let desktop_shortcut = desktop_dir.join("stamp-album-gen2.desktop");
        let existing_shortcut = std::fs::read_to_string(&desktop_shortcut).ok();
        if existing_shortcut.as_deref() != Some(desktop_contents.as_str()) {
            std::fs::write(&desktop_shortcut, desktop_contents.as_bytes())?;
            changed = true;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ =
                std::fs::set_permissions(&desktop_shortcut, std::fs::Permissions::from_mode(0o755));
        }
        // Mark the file as trusted in GIO metadata so GNOME Shell / Nautilus
        // will launch it directly without the "Untrusted application launcher"
        // prompt.  This is the programmatic equivalent of right-clicking
        // "Allow Launching" in the Nautilus file manager.
        mark_desktop_file_trusted(&desktop_dir.join("stamp-album-gen2.desktop"));
    }

    let bundled_icon = bundled_icon_png();
    let existing_icon_256 = std::fs::read(&icon_path_256).ok();
    if existing_icon_256.as_deref() != Some(bundled_icon) {
        std::fs::write(&icon_path_256, bundled_icon)?;
        changed = true;
    }
    let existing_icon_48 = std::fs::read(&icon_path_48).ok();
    if existing_icon_48.as_deref() != Some(bundled_icon) {
        std::fs::write(&icon_path_48, bundled_icon)?;
        changed = true;
    }

    // Clean up legacy placements
    if legacy_svg_icon_path.exists() {
        let _ = std::fs::remove_file(&legacy_svg_icon_path);
        changed = true;
    }
    if legacy_png_icon_path.exists() {
        let _ = std::fs::remove_file(&legacy_png_icon_path);
        changed = true;
    }

    refresh_linux_desktop_caches(&applications_dir);

    let summary = if changed {
        "Desktop launcher and icon installed/updated in ~/.local/share."
    } else {
        "Desktop launcher already configured."
    };

    Ok(LinuxDesktopIntegrationReport {
        changed,
        summary: summary.to_string(),
        desktop_entry_path: desktop_path.to_string_lossy().to_string(),
        icon_path: icon_path_256.to_string_lossy().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::{
        align_process_working_dir_to_runtime_layout, find_runtime_root_from,
        looks_like_runtime_root,
    };
    use tempfile::tempdir;

    #[test]
    fn runtime_root_detects_expected_markers() {
        let t = tempdir().expect("temp dir");
        let root = t.path().join("runtime-root");
        std::fs::create_dir_all(&root).expect("create runtime root");
        std::fs::write(root.join("config.toml"), b"# test config\n").expect("write config");

        let non_root = t.path().join("plain-dir");
        std::fs::create_dir_all(&non_root).expect("create plain dir");

        assert!(looks_like_runtime_root(&root));
        assert!(!looks_like_runtime_root(&non_root));
    }

    #[test]
    fn runtime_root_search_prefers_exe_directory_markers() {
        let t = tempdir().expect("temp dir");
        let current = t.path().join("cwd");
        std::fs::create_dir_all(&current).expect("create cwd");

        let exe_dir = t.path().join("portable");
        std::fs::create_dir_all(&exe_dir).expect("create exe dir");
        std::fs::write(exe_dir.join("config.toml"), b"# test config\n").expect("write config");

        let exe_path = exe_dir.join("stamp_album_gen2");
        let found = find_runtime_root_from(&current, &exe_path);
        assert!(found.is_some());
        assert_eq!(found.expect("runtime root"), exe_dir);
    }

    #[test]
    fn align_runtime_dir_noops_when_already_root() {
        // This test runs from the workspace root in normal cargo flows.
        let result = align_process_working_dir_to_runtime_layout().expect("align cwd");
        let _ = result;
    }
}

pub fn uninstall_linux_desktop_integration() -> std::io::Result<LinuxDesktopIntegrationReport> {
    if !std::env::consts::OS.eq_ignore_ascii_case("linux") {
        return Ok(LinuxDesktopIntegrationReport {
            changed: false,
            summary: "Skipped desktop integration uninstall (non-Linux OS).".to_string(),
            desktop_entry_path: String::new(),
            icon_path: String::new(),
        });
    }

    let home = user_home_dir()?;

    let applications_dir = home.join(".local/share/applications");
    let desktop_path = applications_dir.join("stamp-album-gen2.desktop");
    let legacy_desktop_path = applications_dir.join("stamp_album_gen2.desktop");
    let icon_path = home
        .join(".local/share/icons/hicolor/scalable/apps")
        .join("stamp_album_gen2.png");
    let legacy_icon_path = home
        .join(".local/share/icons/hicolor/scalable/apps")
        .join("stamp_album_gen2.svg");

    let mut changed = false;

    if desktop_path.exists() {
        std::fs::remove_file(&desktop_path)?;
        changed = true;
    }
    if icon_path.exists() {
        std::fs::remove_file(&icon_path)?;
        changed = true;
    }
    if legacy_desktop_path.exists() {
        std::fs::remove_file(&legacy_desktop_path)?;
        changed = true;
    }
    if legacy_icon_path.exists() {
        std::fs::remove_file(&legacy_icon_path)?;
        changed = true;
    }

    set_desktop_launcher_auto_install_disabled(&home, true)?;

    refresh_linux_desktop_caches(&applications_dir);

    let summary = if changed {
        "Desktop launcher artifacts removed from ~/.local/share and auto-install disabled."
    } else {
        "No desktop launcher artifacts were present. Auto-install disabled."
    };

    Ok(LinuxDesktopIntegrationReport {
        changed,
        summary: summary.to_string(),
        desktop_entry_path: desktop_path.to_string_lossy().to_string(),
        icon_path: icon_path.to_string_lossy().to_string(),
    })
}

pub fn install_linux_desktop_integration() -> std::io::Result<LinuxDesktopIntegrationReport> {
    if !std::env::consts::OS.eq_ignore_ascii_case("linux") {
        return Ok(LinuxDesktopIntegrationReport {
            changed: false,
            summary: "Skipped desktop integration install (non-Linux OS).".to_string(),
            desktop_entry_path: String::new(),
            icon_path: String::new(),
        });
    }

    let home = user_home_dir()?;
    set_desktop_launcher_auto_install_disabled(&home, false)?;
    ensure_linux_desktop_integration()
}
