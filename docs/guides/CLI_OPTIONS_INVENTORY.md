# Command-Line Options

This reference reflects the options defined in `src/cli.rs`. To see the options
supported by the installed executable, run:

```text
stamp_album_gen2 --help
```

The configured end year defaults to the current year at runtime. The year shown
by `--help` is therefore expected to change over time.

## Source and album selection

| Option | Default | Purpose |
|--------|---------|---------|
| `--source <SOURCE>` | `stampworld` | Select a source: `stampworld`, `touchstamps`, `extractfile`, `colnectexport`, `us1847`, or `pdf` (offline StampWorld PDF/HTML import). |
| `--site-name <NAME>` | — | Country or site name used by scraping sources. |
| `--stampworld-country-url <URL>` | — | Override the StampWorld country page URL for this run without changing the country or album name. |
| `--album-name <NAME>` | — | Name used for the generated album. |
| `--db-path <PATH>` | — | Database path for generation from an existing database, a Colnect import target, or an offline StampWorld import target. With `--source pdf`, this imports rather than generating output. |
| `--base-url <URL>` | — | Override the constructed StampWorld base URL. |
| `--source-file <PATH>` | — | Local StampWorld HTML file. With `--source pdf`, import PDF/HTML metadata offline into the country database. |

### Offline StampWorld PDF/HTML import

```text
stamp_album_gen2 --source pdf --source-file "download-pdf/Great Britain - Postage stamps (1970 - 1979).pdf" --site-name "Great Britain" --db-path "Albums/Great-Britain/Great-Britain.db"
```

The same arguments work on Windows and Linux; use a local file path appropriate
to the OS. Without `--db-path`, the database is placed under the configured
Albums root in `<Country>/<Country>.db` (spaces in the folder name become hyphens).
`--album-name` can supply the country if `--site-name` is omitted.
`--start-year` and `--end-year` filter imported records.

The importer uses native Rust PDF extraction; Poppler/`pdftotext` and site
credentials are not required. Text-based StampWorld catalogue PDFs and saved
StampWorld HTML are supported. Scanned PDFs require OCR before import. Group
price-summary ranges are excluded from individual stamp rows. Series/year,
type, denomination, color, description, issued quantity, prices/currency,
watermark, designer and engraver are mapped to the database.
Embedded stamp images are not extracted; existing images are left untouched.

Existing country/series/catalogue-number records are skipped without overwriting
metadata. A database error rolls back the file and reports a failure rather than
counting it as a duplicate. The completion message reports found/imported/skipped
counts; skipped includes duplicates and records excluded by the year filter.
Import does not automatically generate an album; use the normal generation flow
after reviewing the imported data.

Both the startup **Import StampWorld - PDF / HTML Files** panel and the country
review **StampWorld Pull** panel offer **Browse & Import**. The selected file is
copied into the destination album's `stampworld` folder and imported into its
database. Selecting an existing file in that folder reuses it; a different file
with the same name is rejected rather than overwritten. Cancellation makes no
database changes. Import errors are displayed and logged; the copied source is
retained for diagnosis/retry. Country review enables **show imageless** after
import so metadata-only records appear immediately.

## Year range and scraping

| Option | Default | Purpose |
|--------|---------|---------|
| `--start-year <YEAR>` | `1840` | First year to scrape. Use `0` to include all years. |
| `--end-year <YEAR>` | Current year | Last year to scrape. |
| `--max-pages <COUNT>` | `0` | Maximum StampWorld pages to fetch; `0` means no explicit limit. |
| `--stop-after-no-hits <COUNT>` | `3` | Stop after this many consecutive pages with no matching stamps. |

## Output and confirmation

| Option | Default | Purpose |
|--------|---------|---------|
| `--overwrite-album` | Off | Allow existing album and image files to be overwritten. |
| `--overwrite-images` | Off | Allow existing image files to be overwritten. |
| `--yes`, `-y` | Off | Accept the top source-name correction automatically, useful in scripts. |

## Configuration

| Option | Default | Purpose |
|--------|---------|---------|
| `--init-config` | Off | Create `config.toml` from the bundled template or defaults, then exit. |
| `--force-init-config` | Off | Use with `--init-config` to replace an existing file after saving a timestamped backup. |

## Colnect import

| Option | Default | Purpose |
|--------|---------|---------|
| `--colnect-csv <PATH>` | — | Import one Colnect CSV file. |
| `--colnect-inbox <DIR>` | — | Recursively discover and process Colnect CSV exports by country. If omitted, `colnectexport` mode can use `colnect_inbox_path` from `config.toml`. |
| `--colnect-generate-after-import` | Off | Generate album output for countries with at least one successful import in this run. |
| `--colnect-auto-route` | Off | Move discovered files into `<inbox>/<country>/` before processing. |
| `--colnect-country <COUNTRY>` | — | Process only the named country; matching is case-insensitive. |
| `--colnect-create-country-db` | Off | Allow inbox processing to create a missing country database. |
| `--colnect-skip-duplicates` | On | Skip files previously imported successfully, using the file's SHA-256 checksum. |
| `--colnect-force-reimport` | Off | Import a file again even if its checksum is already in the import history. |

Inbox processing imports data only by default. Add
`--colnect-generate-after-import` when the same run should also generate pages.
Because `--colnect-auto-route` moves source files, enable it only when that
behavior is intended.

## Review UI, diagnostics, and desktop launcher

| Option | Default | Purpose |
|--------|---------|---------|
| `--review-ui` | Off | Launch the local Review UI for comparing existing and candidate records and applying selected database updates. |
| `--review-ui-bind <ADDRESS>` | `127.0.0.1:7878` | Set the local Review UI server bind address. |
| `--pdf-dx-logging` | Off | Write additional PDF-generation diagnostics to the `Logs/` directory. |
| `--install-desktop-launcher` | Off | Install or re-enable the Linux desktop launcher and icon. |
| `--uninstall-desktop-launcher` | Off | Remove the Linux desktop launcher and icon created by the application. |
| `--help`, `-h` | — | Display command-line help. |
| `--version`, `-V` | — | Display the application version. |

For complete workflows and examples, see [CLI_USAGE.md](../CLI_USAGE.md).
