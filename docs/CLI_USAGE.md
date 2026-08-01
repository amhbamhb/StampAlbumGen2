# CLI Usage

Use this command to print the authoritative live help text:

```bash
./stamp_album_gen2 --help
```

Important: when running the binary directly, do not put an extra `--` after `./stamp_album_gen2`.

Correct:

```bash
./stamp_album_gen2 --source colnectexport --colnect-inbox /path/to/colnect_inbox
```

Incorrect:

```bash
./stamp_album_gen2 -- --source colnectexport
```

## Common Usage Examples

Generate album output from an existing database:

```bash
./stamp_album_gen2 --db-path Albums/Canada/Canada.db --album-name Canada
```

Scrape StampWorld for a country/year range:

```bash
./stamp_album_gen2 \
    --source stampworld \
    --site-name Great-Britain \
    --album-name "Great Britain" \
    --start-year 1900 \
    --end-year 1950
```

Scrape US1847 mode:

```bash
./stamp_album_gen2 \
    --source us1847 \
    --site-name United-States \
    --album-name "United States" \
    --start-year 1847 \
    --end-year 2026
```

Ingest one Colnect CSV (source must be `colnectexport`):

```bash
./stamp_album_gen2 \
    --source colnectexport \
    --colnect-csv /path/to/en_stamps_csv_list_country_206-Switzerland_year_1972.csv \
    --db-path Albums/Switzerland/Switzerland.db \
    --album-name Switzerland
```

Process a Colnect inbox recursively, auto-route files by country, and create missing country DBs:

```bash
./stamp_album_gen2 \
    --source colnectexport \
    --colnect-inbox /path/to/colnect_inbox \
    --colnect-auto-route \
    --colnect-create-country-db
```

Process a Colnect inbox and auto-generate album pages for countries with successful imports:

```bash
./stamp_album_gen2 \
    --source colnectexport \
    --colnect-inbox /path/to/colnect_inbox \
    --colnect-auto-route \
    --colnect-create-country-db \
    --colnect-generate-after-import
```

Note: by default, inbox mode is ingestion-only. It imports CSV data into country
DBs but does not generate AlbumEasy pages unless
`--colnect-generate-after-import` is set.

After inbox import, generate pages per country from the DB:

```bash
./stamp_album_gen2 --db-path Albums/Switzerland/Switzerland.db --album-name Switzerland
./stamp_album_gen2 --db-path Albums/Canada/Canada.db --album-name Canada
```

If `--colnect-inbox` is omitted for `--source colnectexport`, the program uses
`colnect_inbox_path` from `config.toml` and creates that directory on startup if
it does not exist.

Process only one country from an inbox:

```bash
./stamp_album_gen2 \
    --source colnectexport \
    --colnect-inbox /path/to/colnect_inbox \
    --colnect-country Switzerland
```

Force reimport even if CSV checksum was already imported:

```bash
./stamp_album_gen2 \
    --source colnectexport \
    --colnect-inbox /path/to/colnect_inbox \
    --colnect-force-reimport
```

Initialize local config from template:

```bash
./stamp_album_gen2 --init-config
```

Force overwrite existing config (creates backup first):

```bash
./stamp_album_gen2 --init-config --force-init-config
```

Enable PDF DX diagnostics for support runs:

```bash
./stamp_album_gen2 --pdf-dx-logging --review-ui --album-name "Great Britain"
```

## CLI Switch Reference

### Core

- `--source <SOURCE>`
    Allowed values: `stampworld`, `touchstamps`, `extractfile`, `colnectexport`, `us1847`.
- `--site-name <SITE_NAME>`
    Site/country slug for scraping sources.
- `--album-name <ALBUM_NAME>`
    Album name used for output generation.
- `--db-path <DB_PATH>`
    SQLite DB path for output mode or explicit Colnect target DB.
- `--base-url <BASE_URL>`
    Override default StampWorld URL construction.

### Year Range and Scrape Control

- `--start-year <START_YEAR>` (default: `1840`)
- `--end-year <END_YEAR>` (default: current year at runtime)
- `--max-pages <MAX_PAGES>`
    Maximum pages to fetch when scraping (`0` means no explicit cap).
- `--stop-after-no-hits <N>` (default: `3`)
    Stops after `N` consecutive pages without matching stamps.

### Overwrite and Confirmation

- `--overwrite-album`
    Overwrite generated album output files.
- `--overwrite-images`
    Overwrite downloaded/generated image files.
- `--yes` / `-y`
    Auto-accept top source correction when source appears misspelled.

### Config Bootstrapping

- `--init-config`
    Create `config.toml` from template/defaults and exit.
- `--force-init-config`
    Use with `--init-config` to overwrite existing `config.toml` after timestamped backup.

### Colnect Ingestion

- `--colnect-csv <PATH>`
    Ingest a single Colnect CSV file.
    When used with `--db-path`, this run also proceeds to album generation.
- `--colnect-inbox <DIR>`
    Recursively discover and process Colnect CSV files country-by-country.
    If omitted in `colnectexport` mode, falls back to `colnect_inbox_path` in
    `config.toml`.
    By default this mode is ingestion-only; run a follow-up
    `--db-path ... --album-name ...` command to generate pages, or use
    `--colnect-generate-after-import` to auto-generate in the same run.
- `--colnect-generate-after-import`
    After inbox ingestion, generate album output for each country with at least
    one successful import in this run.
- `--colnect-auto-route`
    Move discovered files into `<inbox>/<country>/` before processing.
- `--colnect-country <COUNTRY>`
    Case-insensitive country filter for inbox processing.
- `--colnect-create-country-db`
    Create missing `Albums/<Country>/<Country>.db` during inbox ingestion.
- `--colnect-skip-duplicates`
    Skip files already imported by SHA256 checksum (default behavior).
- `--colnect-force-reimport`
    Reimport even if checksum already exists in import history.

### Diagnostics

- `--pdf-dx-logging`
    Enable additional PDF generation diagnostics for support/debugging.
    Logs are written to `Logs/stamp_album_gen2-*.log`.

### Built-in Utility

- `--help`
- `--version`
