/// PDF generation utilities for the Review UI.
use tiny_http::{Header, Response, StatusCode};
use crate::review_ui::log_runtime_event;

#[allow(dead_code)]
#[derive(Debug, serde::Serialize)]
pub struct BootstrapGeneratePdfResponse {
    pub ok: bool,
    pub message: String,
    pub generated_output_path: Option<String>,
    pub generated_pdf_path: Option<String>,
}

/// Helper for returning error responses for PDF generation tasks.
pub fn review_pdf_error(message: &str, status: u16) -> Response<std::io::Cursor<Vec<u8>>> {
    log_runtime_event(&format!("Review PDF: {message}"));
    let body = serde_json::json!({
        "ok": false,
        "message": message,
        "generated_output_path": null,
        "generated_pdf_path": null
    })
    .to_string();
    let mut response = Response::from_string(body).with_status_code(StatusCode(status));
    if let Ok(header) = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]) {
        response = response.with_header(header);
    }
    response
}

// Future implementation: StampWorld PDF import utilities
// These will be used to bypass anti-bot measures by importing PDF exports directly.

