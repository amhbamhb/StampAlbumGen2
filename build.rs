use chrono::Datelike;

fn main() {
    // Only add Windows resources when building for Windows
    if std::env::var("CARGO_CFG_TARGET_OS").map(|v| v == "windows").unwrap_or(false) {
        let mut res = winres::WindowsResource::new();

        // Use provided ICO at assets/app.ico if present
        let ico_path = std::path::Path::new("assets/app.ico");
        if ico_path.exists() {
            res.set_icon("assets/app.ico");
        }

        let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".into());
        let year = chrono::Utc::now().year();

        res.set_language(0x0409); // English - United States
        res.set("FileDescription", "Stamp Album Generator");
        res.set("CompanyName", "Laser Spider LLC");
        res.set("ProductName", "Stamp Album Gen2");
        res.set("LegalCopyright", &format!("Copyright {} Laser Spider LLC", year));
        res.set("FileVersion", &version);
        res.set("ProductVersion", &version);

        // Ensure compilation errors are visible during build
        if let Err(e) = res.compile() {
            println!("cargo:warning=winres compile failed: {}", e);
        }
    }
}
