// Main entry point for StampAlbumGen2

use clap::CommandFactory;
use clap::Parser;
use flexi_logger::{FileSpec, Logger};
use log::{error, info};
use stamp_album_gen2::commands_file_generator::CommandsFileGenerator;
use stamp_album_gen2::progress::ProgressIndicator;
use stamp_album_gen2::types::OSType;
use strsim::jaro_winkler;

fn get_operating_system_type() -> OSType {
    let os = std::env::consts::OS;
    match os {
        "windows" => OSType::Windows,
        "macos" => OSType::MacOS,
        "linux" => OSType::Linux,
        _ => OSType::Unknown,
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "stamp_album_gen2",
    author,
    version,
    about = "Stamp Album Generator"
)]
struct Cli {
    /// Source type: stampworld, touchstamps, extractfile, colnectexport
    #[arg(long, default_value = "stampworld")]
    source: String,

    /// Site name for scraping (e.g. 'Great-Britain', 'Canada')
    #[arg(long)]
    site_name: Option<String>,

    /// Album name to generate
    #[arg(long)]
    album_name: Option<String>,

    /// Database path (for output-only mode)
    #[arg(long)]
    db_path: Option<String>,

    /// Optional base URL to use when scraping StampWorld (overrides default construction)
    #[arg(long)]
    base_url: Option<String>,

    #[arg(long, default_value_t = 1840)]
    start_year: i32,

    // Default end-year set at compile time to the current year via build.rs -> BUILD_YEAR
    #[arg(long, default_value = env!("BUILD_YEAR"))]
    end_year: i32,

    #[arg(long, default_value_t = false)]
    overwrite_album: bool,

    #[arg(long, default_value_t = false)]
    overwrite_images: bool,

    /// Maximum number of pages to fetch when scraping StampWorld (0 = no explicit cap)
    #[arg(long, default_value_t = 0)]
    max_pages: i32,

    /// Stop scraping after this many consecutive pages that yielded zero matching stamps (default 3)
    #[arg(long, default_value_t = 3)]
    stop_after_no_hits: i32,

    /// When source appears misspelled, accept the top suggestion automatically
    /// (useful in scripts). This is the same as answering 'yes' to a prompt.
    #[arg(long = "yes", short = 'y', default_value_t = false)]
    accept_top_suggestion: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI but handle common user errors with friendlier messages.
    let cli = Cli::parse();

    // Load config early to check for progress indicator setting
    let config = stamp_album_gen2::config::load_config("config.toml");
    let show_progress = config.show_progress_indicator.unwrap_or(true);

    // NOTE: avoid noisy debug printing in normal runs.

    // Friendly guidance: if the user didn't provide any meaningful input, print help
    // and a short English explanation. We consider 'album_name' and 'db_path' as
    // minimal meaningful inputs for typical runs.
    let source_non_default = cli.source.to_lowercase() != "stampworld";
    let user_provided_any = cli.db_path.is_some()
        || cli.album_name.is_some()
        || cli.site_name.is_some()
        || cli.base_url.is_some()
        || source_non_default
        || cli.start_year != 0
        || cli.end_year != 9999
        || cli.overwrite_images
        || cli.overwrite_album
        || cli.max_pages != 0
        || cli.stop_after_no_hits != 3;
    if !user_provided_any {
        eprintln!(
            "It looks like you didn't provide any command options. Here's a short guide to get started:"
        );
        eprintln!(
            "  - To generate an album from an existing database: provide --db-path and --album-name"
        );
        eprintln!(
            "  - To scrape data from StampWorld and generate an album: use --source stampworld --site-name <Site> --album-name <Album> --start-year <YYYY> --end-year <YYYY>"
        );
        eprintln!("If you're unsure, the full help below has all options available :\n");
        // Print clap-generated help
        Cli::command().print_help()?;
        println!();
        std::process::exit(1);
    }

    // Validate year range early: start_year must not be greater than end_year.
    if cli.start_year > cli.end_year {
        eprintln!(
            "Invalid year range: --start-year ({}) is greater than --end-year ({}).",
            cli.start_year, cli.end_year
        );
        std::process::exit(1);
    }

    // Ensure Logs directory exists and configure flexi_logger to write to a
    // uniquely-named file (timestamped) plus duplicate to stderr.
    let logs_dir = std::path::Path::new("Logs");
    if !logs_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(logs_dir) {
            eprintln!("Failed to create Logs directory: {}", e);
            std::process::exit(1);
        }
    }
    // Build a timestamped basename like 'stamp_album_gen2-20251020-171234'
    let ts = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();
    let basename = format!("stamp_album_gen2-{}", ts);
    // Use a conservative default filter that keeps this crate at `debug` but
    // silences noisy external crates (html5ever, reqwest). Allow overriding
    // via the environment (RUST_LOG) for one-off trace captures during
    // debugging, while keeping a safe default to avoid runaway logs.
    Logger::try_with_env_or_str("stamp_album_gen2=debug,html5ever=warn,reqwest=warn")?
        .log_to_file(
            FileSpec::default()
                .directory("Logs")
                .basename(&basename)
                .suffix("log"),
        )
        // Only duplicate errors to stderr by default so debug/info go to the log file only
        .duplicate_to_stderr(flexi_logger::Duplicate::Error)
        .start()?;

    let detected_os = get_operating_system_type();
    if detected_os == OSType::Unknown {
        eprintln!("Unsupported operating system");
        std::process::exit(1);
    }

    // If a database path is explicitly provided, treat this as output-only mode
    // (do not perform a scrape). Otherwise, when source == "stampworld" run
    // the scraper; for other source values require --db-path. If the source
    // value looks unknown, suggest similar known values to help the end user.
    let known_sources = ["stampworld", "touchstamps", "extractfile", "colnectexport"];
    // Use an "effective" source variable that may be auto-corrected below when
    // the user passes --yes. This avoids mutating the parsed `cli` struct.
    let mut effective_source = cli.source.to_lowercase();
    if !known_sources
        .iter()
        .any(|s| s == &effective_source.as_str())
    {
        // Compute fuzzy similarity scores to suggest the best matches (Jaro-Winkler)
        let provided = effective_source.as_str();
        let mut scored: Vec<(&str, f64)> = known_sources
            .iter()
            .map(|s| (*s, jaro_winkler(provided, s)))
            .collect();
        // Sort descending by score
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // If the user asked to accept the top suggestion automatically, only
        // auto-accept when the similarity is very high to avoid false accepts.
        // For borderline scores we'll fall back to prompting the user (if
        // running interactively) so they can confirm. Threshold is conservative.
        if cli.accept_top_suggestion {
            if let Some((best, score)) = scored.first() {
                let threshold = 0.85_f64;
                if *score > threshold {
                    // High confidence: accept silently
                    effective_source = best.to_string();
                } else {
                    // Lower confidence: if interactive, prompt the user; if not
                    // interactive (likely running in a script), do not accept.
                    if atty::is(atty::Stream::Stdin) && atty::is(atty::Stream::Stdout) {
                        use std::io::{self, Write};
                        eprint!(
                            "Top suggestion is '{}' with score={:.3}. Accept? [y/N]: ",
                            best, score
                        );
                        io::stdout().flush().ok();
                        let mut input = String::new();
                        if io::stdin().read_line(&mut input).is_ok() {
                            let resp = input.trim().to_lowercase();
                            if resp == "y" || resp == "yes" {
                                effective_source = best.to_string();
                            }
                        }
                    }
                }
            }
        }

        // If we still don't recognize the source (no auto-accept or score low),
        // print suggestions and exit with help.
        if !known_sources
            .iter()
            .any(|s| s == &effective_source.as_str())
        {
            // Prepare a human-friendly top-3 list with numeric Jaro-Winkler scores.
            let mut top3: Vec<(&str, f64)> = scored
                .iter()
                .take(3)
                .map(|(s, score)| (*s, *score))
                .collect();
            // Filter low scores and keep only those > 0.0 (we'll control messaging)
            top3.retain(|(_, score)| *score > 0.0);

            eprintln!("Unrecognized source '{}'.", cli.source);
            if !top3.is_empty() {
                eprintln!("Top suggestions:");
                for (name, score) in top3.iter() {
                    eprintln!("  - {} (score={:.3})", name, score);
                }
                eprintln!(
                    "(Suggestions computed using Jaro–Winkler similarity; higher is better.)"
                );
            } else {
                eprintln!("Valid sources are: {}", known_sources.join(", "));
            }
            eprintln!("\nHere are quick examples you can copy-paste:");
            eprintln!("  # Generate from an existing DB (db-only):");
            eprintln!("  stamp_album_gen2 --db-path Albums/Canada/Canada.db --album-name Canada");
            eprintln!("\n  # Scrape StampWorld and generate album (scrape-only):");
            eprintln!(
                "  stamp_album_gen2 --source stampworld --site-name 'Great-Britain' --album-name 'Great Britain' --start-year 1900 --end-year 1950\n"
            );
            eprintln!("Full help is shown below:\n");
            Cli::command().print_help()?;
            println!();
            std::process::exit(1);
        }
    }

    if cli.db_path.is_some() {
        // Output-only mode: use the provided db_path and album_name
        let progress = ProgressIndicator::new_with_config(show_progress);
        progress.set_message("🔍 Initializing database mode...");

        let db_path = cli.db_path.unwrap();
        progress.set_message(&format!("📁 Checking database path: {}", db_path));

        // Ensure parent directory exists and attempt to create the DB file if missing
        if let Some(parent) = std::path::Path::new(&db_path).parent() {
            if !parent.exists() {
                progress.set_message(&format!(
                    "📁 Creating parent directory: {}",
                    parent.display()
                ));
                if let Err(e) = std::fs::create_dir_all(parent) {
                    progress
                        .finish_with_error(&format!("Failed to create parent directory: {}", e));
                    eprintln!(
                        "Failed to create parent directory for DB '{}': {}",
                        parent.display(),
                        e
                    );
                    std::process::exit(1);
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
                    std::process::exit(1);
                }
            }
        }

        // Try opening the DB now to detect corrupt/non-sqlite files early and
        // provide a friendly message with suggestions rather than panicking later.
        // Open the database once and hand a reference to the generator.
        progress.set_message("🔗 Opening database connection...");
        let conn = match stamp_album_gen2::db_utils::init_db(&db_path) {
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
                std::process::exit(1);
            }
        };
        let album_name = cli.album_name.unwrap_or_else(|| "Album".to_string());
        progress.set_message(&format!("📖 Generating album output for: {}", album_name));

        let output_path = stamp_album_gen2::album_globals::generate_album_output_conn(
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
    } else if effective_source == "stampworld" {
        // Scrape mode
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
            eprintln!(
                "You started the program in scraping mode but did not provide the site name."
            );
            eprintln!(
                "Please provide the site using --site-name (for example: --site-name 'Great-Britain') or supply --album-name so the program can infer the site."
            );
            eprintln!("If you're not sure what to put, here's the full help text:\n");
            Cli::command().print_help()?;
            println!();
            std::process::exit(1);
        };
        let album_name = cli.album_name.unwrap_or_else(|| site_name.clone());

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
        let mut cfg = CommandsFileGenerator::new();
        cfg.site_name = site_name.clone();
        cfg.images_directory = format!("Albums/{}/images", album_name.replace(' ', "-"));
        cfg.album_directory = format!("Albums/{}", album_name.replace(' ', "-"));
        cfg.album_name = album_name.clone();
        cfg.start_year = cli.start_year;
        cfg.end_year = cli.end_year;

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
            let overrides = stamp_album_gen2::config::load_slug_overrides("slug_overrides.toml");
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
        let cfg_file = stamp_album_gen2::config::load_config("config.toml");

        // Compose the DB path early and set it for the collector so both scraper
        // and generator operate on the same DB file under Albums/{Album}/{Album}.db.
        progress.set_message("💾 Setting up database path...");
        let db_path = format!(
            "{}/{}.db",
            cfg.album_directory,
            album_name.replace('/', "-")
        );
        stamp_album_gen2::file_utils::set_db_path_from_cli(&db_path);
        // Propagate the CLI overwrite flag into global state so generators
        // can create backups when the user explicitly requested overwrites.
        stamp_album_gen2::file_utils::set_overwrite_album_from_cli(cli.overwrite_album);
        progress.set_message(&format!("Stamp Database path set to {}", db_path));

        progress.set_message(&format!("🌐 Starting data collection from: {}", base_url));
        let pages = cfg.collect_stampworld_data(
            &base_url,
            cli.overwrite_images,
            None::<&dyn Fn(usize)>,
            cli.max_pages,
            cli.stop_after_no_hits,
        );
        progress.set_message(&format!(
            "✅ Scraping complete! Found {} new stamp entries from web page(s).",
            pages.len()
        ));
        // println!("Scrape added {} new stamp(s)", pages.len());

        // After scraping, generate album output from the same DB
        // Open the DB that the collector used and call the conn-taking API so
        // both scraper and generator share the same connection lifecycle.
        progress.set_message("🔗 Opening database for album generation...");
        match stamp_album_gen2::db_utils::init_db(&db_path) {
            Ok(conn) => {
                // Attempt to copy a flag into the album images dir if enabled in config
                let print_flag = cfg_file.print_flag_cover.unwrap_or(true);
                if print_flag {
                    progress.set_message("🏳️ Downloading country flag...");
                    let album_images = cfg.images_directory.clone();
                    match stamp_album_gen2::flagdb::copy_flag_for_album(
                        &site_name,
                        &album_images,
                        "w320",
                    ) {
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
                    let fetcher =
                        stamp_album_gen2::stampworld::fetch::ReqwestFetcher::new_for_api();
                    match stamp_album_gen2::wikipedia::fetch_and_cache_content(
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

                progress.set_message(&format!("📖 Generating album output for: {}", album_name));
                let output_path = stamp_album_gen2::album_globals::generate_album_output_conn(
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
                std::process::exit(1);
            }
        }
    } else {
        // No db_path and not stampworld: require db_path
        eprintln!(
            "You requested a source other than 'stampworld' but did not provide a database path to use."
        );
        eprintln!(
            "For non-scraping sources please provide --db-path and --album-name so the program can generate the album output from your existing database."
        );
        eprintln!("Full help is shown below:\n");
        Cli::command().print_help()?;
        println!();
        std::process::exit(1);
    }

    Ok(())
}
