Stamp Album Generator 2

Usage: stamp_album_gen2 [OPTIONS]

Options:
      --source <SOURCE>
          Source type: stampworld, touchstamps, extractfile, colnectexport, us1847 [default: stampworld]
      --site-name <SITE_NAME>
          Site name for scraping (e.g. 'Great-Britain', 'Canada')
      --stampworld-country-url <STAMPWORLD_COUNTRY_URL>
          Override only the StampWorld source URL; country and album names are unchanged
      --album-name <ALBUM_NAME>
          Album name to generate
      --db-path <DB_PATH>
          Database path (for output-only mode)
      --base-url <BASE_URL>
          Optional base URL to use when scraping StampWorld (overrides default construction)
      --start-year <START_YEAR>
          Start year for scraping (default 1840). Use --start-year 0 to scrape all years [default: 1840]
      --end-year <END_YEAR>
          [default: 2026]
      --overwrite-album
          When true, overwrite existing album and image files. Default: false
      --overwrite-images
          When true, overwrite existing image files. Default: false
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
      --colnect-csv <COLNECT_CSV>
          Path to Colnect CSV file for ingestion
      --source-file <SOURCE_FILE>
          Path to a local source HTML file to parse in scraping mode (compatibility wrapper for --source extractfile)
      --colnect-inbox <COLNECT_INBOX>
          Path to a Colnect inbox folder. When set, matching CSV files are discovered recursively and processed country-by-country. By default this mode ingests data only; use --colnect-generate-after-import to auto-generate pages in the same run
      --colnect-generate-after-import
          After Colnect inbox ingestion succeeds, generate album output for each country that had at least one successful import in this run
      --colnect-auto-route
          If true, discovered Colnect CSV files are moved into <inbox>/<country>/ before processing
      --colnect-country <COLNECT_COUNTRY>
          Optional country filter for inbox processing (case-insensitive)
      --colnect-create-country-db
          Allow creating missing Albums/<Country>/<Country>.db when ingesting inbox files. If false, missing country DBs are skipped
      --colnect-skip-duplicates
          Skip files already imported (by SHA256 checksum) into the target country DB. Default true
      --colnect-force-reimport
          Reimport even if checksum already exists in import history
      --review-ui
          Launch local review UI for side-by-side existing/candidate comparison and selective DB updates
      --review-ui-bind <REVIEW_UI_BIND>
          Bind address for review UI server (default: 127.0.0.1:7878)
      --pdf-dx-logging
          Enable additional PDF generation diagnostics (DX/support logging)
      --uninstall-desktop-launcher
          Remove Linux desktop launcher and icon created by Stamp Album Generator 2, then exit
      --install-desktop-launcher
          Re-enable and install Linux desktop launcher and icon, then exit
  -h, --help
          Print help
  -V, --version
          Print version
