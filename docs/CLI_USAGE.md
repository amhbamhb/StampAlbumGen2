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
      --init-config
          Create a `config.toml` from the bundled template (or defaults) and exit. Use --init-config --force-init-config to overwrite an existing file (with backup)
      --force-init-config
          When used with --init-config, force overwriting an existing config.toml after creating a timestamped backup. Default: false
  -h, --help
          Print help
  -V, --version
          Print version
