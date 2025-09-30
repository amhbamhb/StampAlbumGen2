use chrono::Datelike;

fn main() {
    // Emit the current year as an env var for use as the default end year.
    let year = chrono::Utc::now().year();
    println!("cargo:rustc-env=BUILD_YEAR={}", year);
}
