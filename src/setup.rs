use crate::types::OSType;
use flexi_logger::{FileSpec, Logger};

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

    // Use a conservative default filter that keeps this crate at `debug` but
    // silences noisy external crates (html5ever, reqwest). Allow overriding
    // via the environment (RUST_LOG) for one-off trace captures during
    // debugging, while keeping a safe default to avoid runaway logs.
    Logger::try_with_env_or_str("stamp_album_gen2=debug,html5ever=warn,reqwest=warn")?
        .log_to_file(
            FileSpec::default()
                .directory("Logs")
                .basename(&basename)
                .suffix("log"),
        )
        // Only duplicate errors to stderr by default so debug/info go to the log file only
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
