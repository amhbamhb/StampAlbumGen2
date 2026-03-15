use chrono::Datelike;

/// Runtime default end year for CLI and validation checks.
/// Falls back to a conservative future cap if the system clock is unavailable.
pub fn default_end_year() -> i32 {
    let year = chrono::Utc::now().year();
    if year > 0 {
        year
    } else {
        9999
    }
}
