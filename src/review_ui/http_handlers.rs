/// Phase 2 Modularization: HTTP Handlers
/// 
/// This module contains all HTTP request routing and response dispatch logic.
/// 
/// **Routing to extract from run_review_ui_bootstrap and run_review_ui:**
/// Bootstrap routes (run_review_ui_bootstrap):
/// - GET / → render_bootstrap_html
/// - GET /bootstrap-state → scan_bootstrap_countries
/// - GET /api/config-editor → build_config_editor_data
/// - POST /api/config-editor-save → save_full_config_values
/// - POST /api/config-editor-save-album → save_album_config_diff
/// - POST /api/config-editor-delete-album-override → delete override file
/// - POST /start-review → parse request, select country, transition to compare view
/// - POST /generate-pdf-now → PDF generation with GuardLock
/// - POST /shutdown → album generation and exit
/// 
/// Compare routes (run_review_ui):
/// - GET / → render_html(payload)
/// - GET /api/matcher → build matcher payload
/// - POST /api/apply-matches → apply_matches logic
/// - POST /api/add-colnect → add_colnect_stamp_to_stamps
/// - POST /api/delete-colnect → delete_colnect_candidate
/// - POST /api/undo-delete-colnect → undo_delete_colnect
/// - POST /api/clear-colnect-link → clear_colnect_link
/// - POST /api/save-left-stamp → save_left_stamp
/// - POST /api/add-left → add_left_stamp
/// - POST /api/delete-left → delete_left_stamp
/// - POST /api/colnect-import → colnect_import_data
/// - POST /api/default-catalog-config → update_default_catalog_code_for_album
/// - POST /return-to-start → return to bootstrap
/// - 404 fallback
/// 
/// **Dependencies:**
/// - Matcher service functions (once extracted)
/// - Config editor functions (extracted)
/// - View rendering functions (already extracted)
/// - Helper utilities: resolve_image_abs, scan_bootstrap_countries, etc.

pub struct HttpHandlers;

// TODO: Implement HTTP handlers here
