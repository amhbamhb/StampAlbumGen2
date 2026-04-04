use crate::types::OSType;
use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};
use std::process::Command;

const DESKTOP_LAUNCHER_DISABLED_MARKER: &str = "desktop_launcher_disabled.flag";

#[derive(Debug, Clone)]
pub struct LinuxDesktopIntegrationReport {
    pub changed: bool,
    pub summary: String,
    pub desktop_entry_path: String,
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

/// Validate the operating system
pub fn validate_os() -> Result<OSType, SetupError> {
    let detected_os = get_operating_system_type();
    if detected_os == OSType::Unknown {
        return Err(SetupError::UnsupportedOS);
    }
    Ok(detected_os)
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

pub fn ensure_linux_desktop_integration() -> std::io::Result<LinuxDesktopIntegrationReport> {
    if !std::env::consts::OS.eq_ignore_ascii_case("linux") {
        return Ok(LinuxDesktopIntegrationReport {
            changed: false,
            summary: "Skipped desktop integration (non-Linux OS).".to_string(),
            desktop_entry_path: String::new(),
            icon_path: String::new(),
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
                .join(".local/share/icons/hicolor/scalable/apps/stamp_album_gen2.png")
                .to_string_lossy()
                .to_string(),
        });
    }

    let applications_dir = home.join(".local/share/applications");
    let icons_dir = home.join(".local/share/icons/hicolor/scalable/apps");
    std::fs::create_dir_all(&applications_dir)?;
    std::fs::create_dir_all(&icons_dir)?;

    let desktop_path = applications_dir.join("stamp-album-gen2.desktop");
    let icon_path = icons_dir.join("stamp_album_gen2.png");
    let legacy_svg_icon_path = icons_dir.join("stamp_album_gen2.svg");

    let exec_path = desktop_exec_path()?;
    if exec_path.trim().is_empty() {
        return Err(std::io::Error::other(
            "desktop launcher command cannot be empty",
        ));
    }
    let desktop_contents = format!(
        "[Desktop Entry]\nType=Application\nVersion=1.0\nName=Stamp Album Gen2\nComment=Open Stamp Album Gen2 Review UI\nExec={}\nIcon={}\nTerminal=false\nCategories=Graphics;Education;Utility;\nStartupNotify=true\n",
        format!("{} --review-ui", exec_path),
        icon_path.to_string_lossy()
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

    let bundled_icon = bundled_icon_png();
    let existing_icon = std::fs::read(&icon_path).ok();
    if existing_icon.as_deref() != Some(bundled_icon) {
        std::fs::write(&icon_path, bundled_icon)?;
        changed = true;
    }

    if legacy_svg_icon_path.exists() {
        std::fs::remove_file(&legacy_svg_icon_path)?;
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
        icon_path: icon_path.to_string_lossy().to_string(),
    })
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
