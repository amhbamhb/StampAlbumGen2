# Phase 2 Modularization - Comprehensive Execution Plan

**STATUS**: Phase 1 Complete ✅ | Phase 2 Ready for Execution

## Phase 1 Completed

- ✅ Created `src/review_ui/views.rs` (2,708 lines) - All HTML/CSS/JS rendering extracted
- ✅ Reduced `src/review_ui.rs`: 7,945 → 5,243 lines  
- ✅ Build validated with `cargo check`

## Phase 2 Strategy (Ready to Execute)

### Stage A: Config Editor Module (READY)

**File created**: `src/review_ui/config_editor.rs` (1,076 lines)

**Status**: ✅ Fully extracted and ready to integrate

**Integration steps**:

1. Open `src/review_ui.rs`
2. Add module declarations (after line 11):

```rust
mod config_editor;
use config_editor::{
    render_config_editor_assets, build_config_editor_data_for_album, 
    save_album_config_diff, build_config_editor_data, save_full_config_values,
    update_default_catalog_code_for_album, load_current_config_values,
};
```

1. **Remove from `src/review_ui.rs` these line ranges:**
   - Lines 2807-2847: `update_default_catalog_code_for_album()` function
   - Lines 3237-3318: `build_config_editor_data()` function  
   - Lines 3415-3448: `save_full_config_values()` function
   - Lines 3454-3500: `build_config_editor_data_for_album()` function
   - Lines 3505-3609: `save_album_config_diff()` function
   - Lines 3611-3817: `render_config_editor_assets()` function

2. Test build: `cargo check` (should pass clean)

**Expected result**:

- `src/review_ui.rs`: 5,243 → 4,734 lines (~509 lines removed)
- No orphaned braces or syntax errors

---

### Stage B: Matcher Service Module (TO BE FILLED)

**File**: `src/review_ui/matcher_service.rs` (stub created)

**Functions to extract** (~1,200 lines total):

- `sync_processed_colnect_candidates` (line 719)
- `mark_colnect_candidate_processed` (line 737)
- `load_left_stamps` (line 819)
- `load_right_candidates` (line 986)
- `apply_matches` (line 1141)
- `load_left_stamp_by_rowid` (line 1504)
- `load_right_candidate_by_colnect_id` (line 1617)
- `add_colnect_stamp_to_stamps` (line 1768)
- `restore_colnect_candidate_to_queue` (line 2020)
- `save_left_stamp` (line 2093)
- `add_left_stamp` (line 2271)
- `delete_left_stamp` (line 2427)
- `delete_colnect_candidate` (line 2578)
- `clear_colnect_link` (search for this)
- `undo_delete_colnect` (search for this)

**Approach**:

1. Implement in `matcher_service.rs` using module setup from views/config_editor
2. Import needed types from parent: `LeftStamp`, `RightCandidate`, `CatalogCodePair`
3. Update `use` statements to import public functions
4. Test with `cargo check`

---

### Stage C: HTTP Handlers Module (TO BE FILLED)

**File**: `src/review_ui/http_handlers.rs` (stub created

)

**Routes to extract** (~500 lines):

- **Bootstrap handler** (`run_review_ui_bootstrap` refactor):
  - GET / → `render_bootstrap_html`
  - GET /bootstrap-state → `scan_bootstrap_countries`
  - GET /api/config-editor → `build_config_editor_data`  
  - POST /api/config-editor-save → `save_full_config_values`
  - POST /api/config-editor-save-album → `save_album_config_diff`
  - POST /api/config-editor-delete-album-override
  - POST /start-review → select country, transition
  - POST /generate-pdf-now → PDF generation
  - POST /shutdown → cleanup & exit

- **Compare handler** (`run_review_ui` refactor):
  - GET / → `render_html(payload)`
  - GET /api/matcher → build payload
  - POST /api/apply-matches → apply matches
  - POST /api/add-colnect → add colnect stamp
  - POST /api/delete-colnect → delete candidate
  - POST /api/undo-delete-colnect → undo delete
  - POST /api/clear-colnect-link → clear link
  - POST /api/save-left-stamp → save stamp
  - POST /api/add-left → add new left stamp
  - POST /api/delete-left → delete left stamp
  - POST /api/colnect-import → import from colnect
  - POST /api/default-catalog-config → update catalog
  - POST /return-to-start → return to bootstrap
  - 404 fallback

---

## Module Visibility Rules

All three modules use `pub(super)` for functions callable from parent (`review_ui.rs`):

```rust
// In each module:
pub(super) fn function_name(...) -> Result {
    // implementation
}
```

This ensures:

- Functions are public to the parent module
- Completely private outside the `review_ui` module tree
- No namespace pollution in crate root

---

## Testing Checkpoints

After each stage, run:

```bash
cargo check
cargo build --release
cargo test --lib review_ui
```

---

## Key Dependencies to Preserve

- **Types** (keep in `review_ui.rs`):
  - `LeftStamp`, `RightCandidate`, `CatalogCodePair`
  - Request structs: `AddColnectStampRequest`, `SaveLeftStampRequest`, etc.
  - Response structs: `AddColnectStampResponse`, `DeleteLeftStampResponse`, etc.
  - `ConfigEditorField`, `ConfigEditorSection`

- **Shared helpers** (keep in `review_ui.rs`):
  - `resolve_image_abs()`
  - `load_stamp_all_fields()`
  - `value_ref_to_string()`
  - `scan_bootstrap_countries()`
  - `album_dir_for_ui()`
  - `load_album_config()`

- **Utilities** (keep in `review_ui.rs`):
  - `parse_query_param()`
  - `parse_query_flag()`
  - `build_matcher_payload()`
  - `PdfGenerationGuard`

---

## Final Expected State

| File | Lines | Purpose |
|------|-------|---------|
| `src/review_ui.rs` | ~4,700 | Core types, helpers, HTTP dispatch |
| `src/review_ui/views.rs` | 2,708 | HTML/CSS/JS rendering (Phase 1) |
| `src/review_ui/config_editor.rs` | 1,076 | Config editor logic |
| `src/review_ui/matcher_service.rs` | ~1,200 | Stamp matching & mutation |
| `src/review_ui/http_handlers.rs` | ~500 | Route handlers |
| **TOTAL** | ~10,000 | (from 7,945 original + overhead) |

**Key metrics**:

- Reduced main file from 7,945 to ~4,700 lines (41% smaller)
- Clear separation of concerns
- Each module ~1,000-2,700 lines (maintainable)
- All tests passing

---

## Next Immediate Action

1. ✅ Ready: **Integrate config_editor.rs** (Stage A)  
2. 📋 Ready: **Extract matcher_service.rs** (Stage B)
3. 📋 Ready: **Extract http_handlers.rs** (Stage C)
