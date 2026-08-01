use chrono::{Local, Utc};

/// Return a formatted timestamp using Local time when available, otherwise fall back to Utc.
/// This wraps `Local::now()` in `catch_unwind` to avoid process aborts when the
/// local timezone lookup fails (e.g., under Wine without tzdata).
pub fn formatted_now(fmt: &str) -> String {
    let res = std::panic::catch_unwind(|| Local::now().format(fmt).to_string());
    match res {
        Ok(s) => s,
        Err(_) => Utc::now().format(fmt).to_string(),
    }
}

/// Convenience: return `YYYYMMDD-HHMMSS` style timestamp.
pub fn timestamp_compact() -> String {
    formatted_now("%Y%m%d-%H%M%S")
}
