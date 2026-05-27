# StampAlbumGen2 — Before-Refactor Logic Diagrams

Date: 2026-05-26  
Scope: "Before picture" of the full application logic, created as a stable reference
prior to the `views.rs` modularization described in `../views-rs-refactor-plan.md`.

---

## File

`app-logic-before.drawio` — open with [draw.io](https://app.diagrams.net) or the
VS Code draw.io extension (hediet.vscode-drawio).

---

## Pages

| # | Page Name              | What it shows |
|---|------------------------|---------------|
| 1 | System Overview        | Bird's-eye context: all major subsystems, data stores, and how they connect. Good starting point. |
| 2 | App Startup            | `main.rs` flowchart — CLI parsing, config loading, auto-detect mode, mode dispatch. |
| 3 | Bootstrap UI           | `run_review_ui_bootstrap()` server loop — all ~14 HTTP endpoints, StampWorld job thread, browser open, enum outcomes. |
| 4 | Review Compare UI      | `run_review_ui()` server loop — data loading from SQLite, `views.rs` call, all ~25+ HTTP endpoints, enum outcomes. |
| 5 | HTTP API Reference     | Tabular endpoint map for both servers, grouped by functional category (health, config, data, PDF, etc.). |
| 6 | Data Layer & Modes     | SQLite schema snapshot, scraping pipeline (`run_scraping_mode`), Colnect CSV ingest (`run_database_mode`). |
| 7 | CLI Switch Map         | Every CLI flag from `src/cli.rs`, grouped by startup/control, scraping, Colnect ingest, review UI, and dispatch effects. |

---

## Color Legend

| Color     | Meaning                              |
|-----------|--------------------------------------|
| Green     | Startup / Config / File system       |
| Blue      | Bootstrap UI (server + browser)      |
| Purple    | Review Compare UI (server + browser) |
| Pink/Red  | HTTP API endpoints                   |
| Orange    | Database / SQLite / Data layer       |
| Yellow    | CLI / User input / External entities |
| Cyan      | Scraping / Network fetch             |
| Gray      | Navigation buttons / misc labels     |

---

## Navigation

Every page has colored navigation buttons in the top-left that link to the other
pages using draw.io's internal page-link feature.  Click a button to jump.

---

## Key Observations (captured for refactor planning)

### views.rs is called from exactly two places

- `render_bootstrap_html(inbox_hint)` — called by the Bootstrap HTTP server at `GET /`
- `render_html(&payload)` — called by the Review HTTP server at `GET /`
- `render_app_footer_html()` — helper called by both of the above

### Two independent HTTP servers share nearly identical shared endpoints

Both Bootstrap and Review servers expose:

- `GET /api/ping`
- `GET /runtime-log`
- `GET /health`
- `GET /api/stampworld-status`
- `GET /api/stampworld-log-tail`
- `POST /api/stampworld-run`
- `GET/POST /api/config-editor`

These are the primary duplication targets for Phase 5 of the refactor plan.

### StampWorld job runs as a child process

`start_bootstrap_stampworld_job()` spawns the same binary with `--source stampworld`
args, captures stdout/stderr into `log_runtime_event`, and tracks state via
`StampWorldJobState` (Idle / Running / Finished).

### ReviewUiBootstrapOutcome drives the main loop in main.rs

```
AlreadyRunning  → hand off to existing instance, exit
Exit            → normal close
ReloadRequested → loop back to Bootstrap (config changed)
Selected        → transition to Review Compare UI (server socket reused)
```

### ReviewUiRunOutcome drives the main loop in main.rs

```
Exit                  → generate album if requested, exit
ReloadRequested       → loop back within Review UI
ReturnToStartRequested → loop back to Bootstrap UI
```
