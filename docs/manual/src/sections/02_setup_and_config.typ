// ==============================================================================
// Section 2: Installation, Directory Layout & Configuration
// ==============================================================================

#pagebreak()
= 2. Installation, Directory Layout & Configuration

== 2.1 Installation & Startup

StampAlbumGen2 is distributed as a standalone binary for Windows (`stamp_album_gen2.exe`) and Linux (`stamp_album_gen2`). No complex external database servers or runtime dependencies are required.

When first launched in an empty working folder, the application automatically initializes its runtime environment:
- Copies `config_template.toml` to `config.toml` if missing.
- Extracts `StampAlbumGenerator2HelpManual.pdf` to the local folder.
- Creates `UserImport/manual_import_template.csv` and `UserImport/README.md`.
- Initializes local directories for output albums, flags, and logs.

== 2.2 Directory Structure

```
<Working Directory>/
├── config.toml                 # Active collector runtime configuration
├── config_template.toml        # Baseline reference template
├── StampAlbumGenerator2HelpManual.pdf # This manual (auto-updated from binary)
├── Albums/                     # Output destination for generated albums
│   └── <Country>/              # Per-country generated PDF and source files
├── Logs/                       # Timestamped diagnostic execution logs
├── UserImport/                 # Manual CSV imports for offline or custom stamps
│   ├── manual_import_template.csv
│   └── README.md
├── flags/                      # National flags and coat of arms cache
│   └── flag_codes.json         # Optional country-to-ISO code overrides
└── stamp_album.db              # SQLite relational database (catalog & user data)
```

== 2.3 Configuration Parameters (`config.toml`)

The application is configured through `config.toml`. All keys feature sensible defaults:

#table(
  columns: (1.5fr, 1fr, 3fr),
  fill: (x, y) => if y == 0 { rgb("#1B365D").lighten(85%) } else if calc.even(y) { rgb("#F8F9FA") } else { none },
  stroke: 0.5pt + rgb("#D0D5DD"),
  table.header([*Setting Key*], [*Default*], [*Description*]),
  [`albums_root`], [`"Albums"`], [Root directory where generated album folders and PDFs are written.],
  [`site_name`], [`"Switzerland"`], [Default country or catalog section to process.],
  [`album_name`], [`"Switzerland"`], [Display title printed on album title pages and running headers.],
  [`start_year` / `end_year`], [`1970` / `1970`], [Filter stamp issues by issue year range.],
  [`source`], [`"stampworld"`], [Primary catalog source (`"stampworld"`, `"colnect"`, `"us1847"`).],
  [`stampworld_fetch_delay_ms`], [`500`], [Polite delay in milliseconds between StampWorld web scraper requests.],
  [`albumeasy_pdf_timeout_secs`], [`300`], [Timeout in seconds when waiting for AlbumEasy generation to finish.],
  [`albumeasy_path`], [`"albumeasy"`], [Custom filesystem path to AlbumEasy executable if not in PATH.],
  [`review_ui_bind`], [`"127.0.0.1:7878"`], [Network address and port for the local Review UI HTTP server.],
  [`print_flag_cover`], [`true`], [Whether to generate a full-color country flag cover page.],
  [`flag_cover_overwrite_flag`], [`false`], [Force re-download of cached flag images from CDN repositories.],
)

== 2.4 Command Line Options

Run `stamp_album_gen2 --help` to inspect command-line arguments:

```bash
# Launch interactive Review UI (default when run without batch arguments):
stamp_album_gen2

# Batch generate an album from StampWorld data for Switzerland:
stamp_album_gen2 --source stampworld --site-name Switzerland --start-year 1970 --end-year 1975

# Launch with diagnostic vector logging enabled:
stamp_album_gen2 --pdf-dx-logging

# Specify custom Review UI network binding:
stamp_album_gen2 --review-ui-bind 0.0.0.0:8080
```
