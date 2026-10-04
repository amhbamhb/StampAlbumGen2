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
| `--source <SOURCE>` | `stampworld` | Select a source: `stampworld`, `touchstamps`, `extractfile`, `colnectexport`, or `us1847`. |
| `--site-name <NAME>` | — | Country or site name used by scraping sources. |
| `--stampworld-country-url <URL>` | — | Override the StampWorld country page URL for this run without changing the country or album name. |
| `--album-name <NAME>` | — | Name used for the generated album. |
| `--db-path <PATH>` | — | Database path for generation from an existing database or for a Colnect import target. |
| `--base-url <URL>` | — | Override the constructed StampWorld base URL. |
| `--source-file <PATH>` | — | Parse a local source HTML file; compatibility shortcut for `--source extractfile`. |

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
