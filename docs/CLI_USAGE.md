# CLI Usage — Stamp Album Generator

When running the Stamp Album Generator during development or automated runs please use the canonical CLI. The output must be identical whether the program is run in "scrape mode" using `--source` or in "db-only/output" mode by providing `--db-path`.

Stamp Album Generator

Usage: stamp_album_gen2 [OPTIONS]

Options:
      --source <SOURCE>
          Source type: stampworld, touchstamps, extractfile, colnectexport [default: stampworld]
      --site-name <SITE_NAME>
          Site name for scraping (e.g. 'Great-Britain', 'Canada')
      --album-name <ALBUM_NAME>
          Album name to generate
      --db-path <DB_PATH>
          Database path (for output-only mode)
      --base-url <BASE_URL>
          Optional base URL to use when scraping StampWorld (overrides default construction)
      --start-year <START_YEAR>
          [default: 1840]
      --end-year <END_YEAR>
          [default: 2025]
      --overwrite-album

      --overwrite-images
          
      --max-pages <MAX_PAGES>
          Maximum number of pages to fetch when scraping StampWorld (0 = no explicit cap) [default: 0]
      --stop-after-no-hits <STOP_AFTER_NO_HITS>
          Stop scraping after this many consecutive pages that yielded zero matching stamps (default 3) [default: 3]
  -y, --yes
          When source appears misspelled, accept the top suggestion automatically (useful in scripts). This is the same as answering 'yes' to a prompt
  -h, --help
          Print help
  -V, --version
          Print version



