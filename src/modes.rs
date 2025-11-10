use crate::progress::ProgressIndicator;

/// Run the database-only mode (generate album from existing database)
pub fn run_database_mode(
    cli: &crate::cli::Cli,
    show_progress: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Output-only mode: use the provided db_path and album_name
    let progress = ProgressIndicator::new_with_config(show_progress);
    progress.set_message("🔍 Initializing database mode...");

    let db_path = cli.db_path.as_ref().unwrap();
    progress.set_message(&format!("📁 Checking database path: {}", db_path));

    // Ensure parent directory exists and attempt to create the DB file if missing
    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        if !parent.exists() {
            progress.set_message(&format!(
                "📁 Creating parent directory: {}",
                parent.display()
            ));
            if let Err(e) = std::fs::create_dir_all(parent) {
                progress.finish_with_error(&format!("Failed to create parent directory: {}", e));
                eprintln!(
                    "Failed to create parent directory for DB '{}': {}",
                    parent.display(),
                    e
                );
                return Err(e.into());
            }
        }
    }
    if !std::path::Path::new(&db_path).exists() {
        progress.set_message("📝 Creating new database file...");
        match std::fs::File::create(&db_path) {
            Ok(_) => (),
            Err(e) => {
                progress.finish_with_error(&format!("Failed to create DB file: {}", e));
                eprintln!("Failed to create DB file '{}': {}", db_path, e);
                return Err(e.into());
            }
        }
    }

    // Try opening the DB now to detect corrupt/non-sqlite files early and
    // provide a friendly message with suggestions rather than panicking later.
    // Open the database once and hand a reference to the generator.
    progress.set_message("🔗 Opening database connection...");
    let conn = match crate::db_utils::init_db(&db_path) {
        Ok(c) => c,
        Err(e) => {
            progress.finish_with_error(&format!("Database connection failed: {}", e));
            eprintln!("Failed to open database '{}': {}", db_path, e);
            eprintln!("Possible causes:");
            eprintln!("  - The file is not a SQLite database or is corrupted");
            eprintln!("  - File permissions prevent opening the file");
            eprintln!("  - Path points to a directory rather than a file");
            eprintln!("Possible fixes:");
            eprintln!(
                "  - Check the file: 'file {}' or try 'sqlite3 {} .dump' to inspect",
                db_path, db_path
            );
            eprintln!(
                "  - Remove or move the corrupt file so the program can create a new DB: rm {}",
                db_path
            );
            eprintln!("  - Ensure the path is correct and you have read/write permissions");
            return Err(e.into());
        }
    };
    let album_name = cli
        .album_name
        .clone()
        .unwrap_or_else(|| "Album".to_string());
    progress.set_message(&format!("📖 Generating album output for: {}", album_name));

    // Temporary unconditional marker to help automated runs detect that
    // we've reached the album-generation step and to print the env flags
    // relevant to wiki metric emission. This is intentionally a stdout
    // println! so it will appear in captured logs even if the logger
    // is configured to write to files.
    println!(
        "FORCE_WIKI_PAGE_METRICS_RUN album={} WIKI_PAGE_DEBUG={} FORCE_WIKI_PAGE_METRICS={}",
        album_name,
        std::env::var("WIKI_PAGE_DEBUG").unwrap_or_else(|_| "".to_string()),
        std::env::var("FORCE_WIKI_PAGE_METRICS").unwrap_or_else(|_| "".to_string())
    );

    let output_path = crate::album_globals::generate_album_output_conn(
        &conn,
        &album_name,
        None::<fn(usize)>,
        Some(cli.start_year),
        Some(cli.end_year),
    );

    progress.finish_with_message(&format!(
        "Album '{}' generated successfully! 📄 File: {}",
        album_name,
        output_path.display()
    ));

    Ok(())
}

/// Run the scraping mode (scrape StampWorld and generate album)
pub fn run_scraping_mode(
    cli: &crate::cli::Cli,
    show_progress: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::commands_file_generator::CommandsFileGenerator;
    use log::{error, info};

    let progress = ProgressIndicator::new_with_config(show_progress);
    progress.set_message("🌐 Preparing StampWorld scraping mode...");

    // Require site_name for scraping. If the caller explicitly passed
    // --site-name but set it to the literal value 'stampworld' (the
    // default/source name), prefer the provided --album-name when
    // available. This prevents constructing URLs like
    // '/en/stamps/stampworld/...' when the intended album slug is the
    // album name (e.g. 'New-Brunswick'). If neither is provided, exit.
    let site_name = if let Some(s) = cli.site_name.clone() {
        if s.to_lowercase() == "stampworld" {
            if let Some(a) = cli.album_name.clone() {
                a.clone()
            } else {
                s
            }
        } else {
            s
        }
    } else if let Some(a) = cli.album_name.clone() {
        a.clone()
    } else {
        progress.finish_with_error("Missing site name for scraping");
        eprintln!("You started the program in scraping mode but did not provide the site name.");
        eprintln!(
            "Please provide the site using --site-name (for example: --site-name 'Great-Britain') or supply --album-name so the program can infer the site."
        );
        eprintln!("If you're not sure what to put, here's the full help text:\n");
        // This would need to be handled by the caller
        return Err("Missing site name for scraping".into());
    };
    let album_name = cli.album_name.clone().unwrap_or_else(|| site_name.clone());

    progress.set_message(&format!(
        "📋 Setting up scrape: {} → {} ({}..{})",
        site_name, album_name, cli.start_year, cli.end_year
    ));
    progress.set_message(
        &format!(
            "Starting StampWorld scrape for site='{}' album='{}' years={}..{} overwrite_images={} overwrite_album={}",
            site_name,
            album_name,
            cli.start_year,
            cli.end_year,
            cli.overwrite_images,
            cli.overwrite_album
        )
    );

    // Build a CommandsFileGenerator and invoke the collector
    progress.set_message("⚙️ Configuring data collector...");
    let mut cfg = CommandsFileGenerator::default();
    cfg.album_name = album_name.clone();
    cfg.country = site_name.clone();
    cfg.album_path = format!("Albums/{}", album_name);
    cfg.image_path = format!("Albums/{}/images", album_name);
    cfg.year = cli.start_year.to_string();

    // Build base URL for stampworld pages.
    // When the caller does not provide --base-url, construct the canonical
    // StampWorld listing URL that includes the Postage stamps year-range
    // so the collector can append a page number. Example produced:
    // https://www.stampworld.com/en/stamps/Bahamas/Postage%20stamps/1859-1863?page=
    progress.set_message("🔗 Building StampWorld URL...");
    let base_url = if let Some(b) = &cli.base_url {
        b.clone()
    } else {
        // Allow users to provide explicit overrides via `slug_overrides.toml`.
        // This file (looked up like `config.toml`) may contain a table
        // `[slug_overrides]` mapping the provided site_name to a canonical
        // StampWorld slug (which may contain '!' or other punctuation).
        let overrides = crate::config::load_slug_overrides("slug_overrides.toml");
        // Helper to percent-encode a slug for a path component
        // Prefer hyphen-separated slugs for multi-word site/album names
        let encode = |s: &str| s.replace(' ', "-").replace('!', "%21").replace('/', "%2F");

        // If the caller requested the full open range (0..9999) treat this
        // as an intent to fetch the album/main slug page rather than the
        // 'Postage stamps' year-range listing. In that case construct URLs
        // like: https://www.stampworld.com/en/stamps/<slug>/?page=
        let use_main_slug = cli.start_year == 0 && cli.end_year == 9999;

        if let Some(canon) = overrides.get(&site_name) {
            // If there's an explicit override, use it verbatim (but still
            // percent-encode reserved characters for the URL path).
            let encoded = encode(canon);
            // Avoid printing slug override details in normal runs.
            if use_main_slug {
                format!("https://www.stampworld.com/en/stamps/{}/?page=", encoded)
            } else {
                format!(
                    "https://www.stampworld.com/en/stamps/{}/Postage%20stamps/{}-{}?page=",
                    encoded, cli.start_year, cli.end_year
                )
            }
        } else {
            // Minimal percent-encoding for common problematic characters
            // that appear in StampWorld site names (space, '!' and '/').
            let encoded_site = encode(&site_name);
            if use_main_slug {
                format!(
                    "https://www.stampworld.com/en/stamps/{}/?page=",
                    encoded_site
                )
            } else {
                format!(
                    "https://www.stampworld.com/en/stamps/{}/Postage%20stamps/{}-{}?page=",
                    encoded_site, cli.start_year, cli.end_year
                )
            }
        }
    };
    // Load full config (so flag/wiki toggles are available)
    progress.set_message("⚙️ Loading configuration file...");
    let cfg_file = crate::config::load_config("config.toml");

    // Compose the DB path early and set it for the collector so both scraper
    // and generator operate on the same DB file under Albums/{Album}/{Album}.db.
    progress.set_message("💾 Setting up database path...");
    let db_path = format!("{}/{}.db", cfg.album_path, album_name.replace('/', "-"));
    crate::file_utils::set_db_path_from_cli(&db_path);
    // Propagate the CLI overwrite flag into global state so generators
    // can create backups when the user explicitly requested overwrites.
    crate::file_utils::set_overwrite_album_from_cli(cli.overwrite_album);
    progress.set_message(&format!("Stamp Database path set to {}", db_path));

    progress.set_message(&format!("🌐 Starting data collection from: {}", base_url));
    let pages = cfg.collect_stampworld_data(
        &base_url,
        cli.overwrite_images,
        None::<&dyn Fn(usize)>,
        cli.max_pages,
        cli.stop_after_no_hits,
        cli.start_year,
        cli.end_year,
    );
    match pages {
        Ok(pages) => {
            progress.set_message(&format!(
                "✅ Scraping complete! Found {} new stamp entries from web page(s).",
                pages.len()
            ));

            // After scraping, generate album output from the same DB
            // Open the DB that the collector used and call the conn-taking API so
            // both scraper and generator share the same connection lifecycle.
            progress.set_message("🔗 Opening database for album generation...");
            match crate::db_utils::init_db(&db_path) {
                Ok(conn) => {
                    // Attempt to copy a flag into the album images dir if enabled in config
                    let print_flag = cfg_file.print_flag_cover.unwrap_or(true);
                    if print_flag {
                        progress.set_message("🏳️ Downloading country flag...");
                        let album_images = cfg.image_path.clone();
                        match crate::flagdb::copy_flag_for_album(&site_name, &album_images, "w320")
                        {
                            Ok(p) => info!("Copied flag into album images: {}", p),
                            Err(e) => info!(
                                "No flag found for '{}' or download failed; continuing. error={}",
                                site_name, e
                            ),
                        }
                    }
                    // Optional: fetch Wikipedia summary if requested by config
                    let fetch_wiki = cfg_file
                        .fetch_wikipedia
                        .unwrap_or_else(|| cfg_file.include_wikipedia_content.unwrap_or(false));
                    if fetch_wiki {
                        progress.set_message("📚 Fetching Wikipedia content...");
                        let cache_db_path = cfg_file
                            .wikipedia_cache_db_path
                            .as_deref()
                            .unwrap_or("wikipedia_cache.db");
                        let fetcher = crate::stampworld::fetch::ReqwestFetcher::new_for_api();
                        match crate::wikipedia::fetch_and_cache_content(
                            &site_name,
                            cache_db_path,
                            &fetcher,
                        ) {
                            Ok(_) => info!(
                                "✅ Successfully cached Wikipedia content for '{}'",
                                site_name
                            ),
                            Err(e) => {
                                info!("❌ Wikipedia fetch failed for {}: {}", site_name, e);
                                // Log the full error for debugging
                                error!("Wikipedia fetch error details: {:?}", e);
                            }
                        }
                    }

                    progress
                        .set_message(&format!("📖 Generating album output for: {}", album_name));
                    let output_path = crate::album_globals::generate_album_output_conn(
                        &conn,
                        &album_name,
                        None::<fn(usize)>,
                        Some(cli.start_year),
                        Some(cli.end_year),
                    );

                    progress.finish_with_message(&format!(
                        "Album '{}' completed successfully! 🎉 📄 File: {}",
                        album_name,
                        output_path.display()
                    ));
                }
                Err(e) => {
                    progress.finish_with_error(&format!("Database connection failed: {}", e));
                    eprintln!(
                        "Failed to open database for generation '{}': {}",
                        db_path, e
                    );
                    return Err(e.into());
                }
            }
        }
        Err(e) => {
            progress.finish_with_error(&format!("Data collection failed: {}", e));
            eprintln!("Failed to collect stampworld data: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
