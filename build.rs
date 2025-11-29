use chrono::Datelike;

fn main() {
    // Emit the current year as an env var for use as the default end year.
    let year = chrono::Utc::now().year();
    println!("cargo:rustc-env=BUILD_YEAR={}", year);

    // For MUSL targets, configure OpenSSL to be built from source (vendored)
    // This ensures static linking works correctly across all Linux systems
    if std::env::var("TARGET")
        .map(|t| t.contains("musl"))
        .unwrap_or(false)
    {
        // Tell openssl-sys to use vendored OpenSSL
        println!("cargo:rustc-env=OPENSSL_STATIC=1");
        println!("cargo:rustc-link-search=native=/usr/lib");
        println!("cargo:rustc-link-search=native=/usr/local/lib");
    }
}
