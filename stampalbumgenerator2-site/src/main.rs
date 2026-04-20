use axum::extract::{DefaultBodyLimit, Path as AxumPath};
use axum::http::header;
use axum::http::{HeaderName, HeaderValue, StatusCode};
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use chrono::{Datelike, Utc};
use std::env;
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::path::Path;
use tower_http::compression::CompressionLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::services::{ServeDir, ServeFile};
use tracing::info;

const COPYRIGHT_START_YEAR: i32 = 2025;
const APP_VERSION_FILE: &str = include_str!("../APP_VERSION");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init();

    let app = build_app();

    let port = env::var("PORT")
        .ok()
        .and_then(|raw| raw.parse::<u16>().ok())
        .unwrap_or(4205);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(error) if error.kind() == ErrorKind::AddrInUse => {
            eprintln!(
                "Port {} is already in use. Stop the previous site process or run with another port, e.g. PORT=4300.",
                port
            );
            std::process::exit(1);
        }
        Err(error) => {
            eprintln!("Failed to bind TCP listener on {}: {}", addr, error);
            std::process::exit(1);
        }
    };
    info!("stampalbumgenerator2-site listening on http://{}", addr);

    axum::serve(listener, app)
        .await
        .expect("web server terminated unexpectedly");
}

fn build_app() -> Router {
    let static_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("static");
    let index_file = static_root.join("index.html");
    let permissions_policy = HeaderName::from_static("permissions-policy");
    let csp = HeaderName::from_static("content-security-policy");

    Router::new()
        .route_service("/", ServeFile::new(index_file.clone()))
        .route("/healthz", get(healthz))
        .route("/__heartbeat", get(heartbeat))
        .route("/api/version", get(version))
        .route("/release/{platform}", get(release_placeholder))
        .fallback_service(ServeDir::new(static_root).not_found_service(ServeFile::new(index_file)))
        .layer(DefaultBodyLimit::max(16 * 1024))
        .layer(CompressionLayer::new())
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::REFERRER_POLICY,
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            permissions_policy,
            HeaderValue::from_static(
                "accelerometer=(), camera=(), geolocation=(), gyroscope=(), magnetometer=(), microphone=(), payment=(), usb=()",
            ),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            csp,
            HeaderValue::from_static(
                "default-src 'self'; style-src 'self' https://fonts.googleapis.com; font-src 'self' https://fonts.gstatic.com; script-src 'self'; img-src 'self' data:; object-src 'none'; base-uri 'none'; frame-ancestors 'none'; form-action 'self'",
            ),
        ))
}

async fn healthz() -> &'static str {
    "ok"
}

async fn heartbeat() -> &'static str {
    "alive"
}

async fn version() -> &'static str {
    app_version()
}

async fn release_placeholder(AxumPath(platform): AxumPath<String>) -> (StatusCode, Html<String>) {
    let friendly = match platform.as_str() {
        "linux" => "Linux",
        "windows" => "Windows",
        _ => "Requested platform",
    };

    let binary_link = match platform.as_str() {
        "linux" => "/downloads/current/stamp_album_gen2",
        "windows" => "/downloads/current/stamp_album_gen2.exe",
        _ => "/downloads/current/",
    };

    let current_year = current_utc_year();

    let body = format!(
        "<h1>{friendly} build</h1><p>Download this platform binary here: <a href=\"{binary_link}\">{binary_link}</a></p><p>Use SHA-256 as the primary integrity check. Per-file checksums are available at <a href=\"/downloads/current/stamp_album_gen2.sha256\">stamp_album_gen2.sha256</a> and <a href=\"/downloads/current/stamp_album_gen2.exe.sha256\">stamp_album_gen2.exe.sha256</a>.</p><p>Additional manifests: <a href=\"/downloads/current/SHA256SUMS.txt\">SHA256SUMS.txt</a> | <a href=\"/downloads/current/CRC32SUMS.txt\">CRC32SUMS.txt</a></p><p><a href=\"/pages/download-verification.html\">Read the download verification guide</a></p><p><a href=\"/\">Back to home</a></p><hr><p>Copyright © {COPYRIGHT_START_YEAR} - {current_year} Laser Spider LLC. All rights reserved.</p><p>Stamp Album Generator 2 | Version {}</p>",
        app_version()
    );

    (StatusCode::OK, Html(body))
}

fn current_utc_year() -> i32 {
    Utc::now().year()
}

fn app_version() -> &'static str {
    APP_VERSION_FILE.trim()
}

#[cfg(test)]
mod tests {
    use super::build_app;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn security_headers_are_present_on_root() {
        let app = build_app();
        let response = app
            .oneshot(Request::builder().uri("/").body(axum::body::Body::empty()).expect("request build failed"))
            .await
            .expect("app request failed");

        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.headers().get("content-security-policy").is_some());
        assert!(response.headers().get("x-content-type-options").is_some());
        assert!(response.headers().get("x-frame-options").is_some());
        assert!(response.headers().get("permissions-policy").is_some());
        assert!(response.headers().get("referrer-policy").is_some());
    }

    #[test]
    fn current_year_is_reasonable() {
        let year = super::current_utc_year();
        assert!((2025..=2100).contains(&year));
    }

    #[test]
    fn app_version_is_not_empty() {
        assert!(!super::app_version().is_empty());
    }
}
