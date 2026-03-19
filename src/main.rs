// Main entry point for StampAlbumGen2

// use tracing::{info, error, warn, instrument};
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// // Setup once at startup
// pub fn init_logging() {
//     tracing_subscriber::registry()
//         .with(
//             tracing_subscriber::EnvFilter::try_from_default_env()
//                 .unwrap_or_else(|_| "info".into())
//         )
//         .with(tracing_subscriber::fmt::layer().json())
//         .init();
// }

use clap::{CommandFactory, Parser};
use stamp_album_gen2::{
    cli::Cli,
    config,
    modes::{run_database_mode, run_scraping_mode},
    review_ui::run_review_ui,
    setup::{setup_logging, validate_os},
    validation::{
        print_validation_error, validate_source, validate_user_input, validate_year_range,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI arguments
    let mut cli = Cli::parse();

    // Review UI mode is intentionally independent from normal source/mode
    // validation to keep the workflow lightweight.
    if cli.review_ui {
        validate_os()?;

        let db_path = if let Some(path) = cli.db_path.clone() {
            path
        } else if let Some(album_name) = cli.album_name.clone() {
            let album_dir = album_name.trim().replace(' ', "-");
            format!("Albums/{0}/{0}.db", album_dir)
        } else {
            eprintln!("Review UI requires --db-path or --album-name.");
            eprintln!("Example: --review-ui --db-path Albums/Malta/Malta.db");
            std::process::exit(1);
        };

        run_review_ui(
            &db_path,
            cli.album_name.as_deref(),
            cli.review_ui_bind.as_deref(),
        )?;
        return Ok(());
    }

    // If requested, initialize (or force-initialize) the config file and exit.
    if cli.init_config {
        // This will create config.toml from config_template.toml if present,
        // or serialize defaults. When force_init_config is true it will
        // overwrite an existing file after creating a timestamped backup.
        match config::create_config_from_template("config.toml", cli.force_init_config) {
            Ok(_) => {
                println!("Configuration file created/updated at 'config.toml'.");
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Failed to create config file: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Load config early to check for progress indicator setting
    let config = config::load_config("config.toml");
    let show_progress = config.show_progress_indicator.unwrap_or(true);

    // For Colnect inbox mode, allow config.toml to provide a default inbox
    // path when the CLI omits --colnect-inbox.
    if cli.source.eq_ignore_ascii_case("colnectexport")
        && cli.colnect_inbox.is_none()
        && cli.colnect_csv.is_none()
    {
        if let Some(default_inbox) = config.effective_colnect_inbox_path() {
            eprintln!(
                "No --colnect-inbox provided; using config default colnect_inbox_path='{}'.",
                default_inbox
            );
            cli.colnect_inbox = Some(default_inbox.to_string());
        }
    }

    // Validate user input
    if let Err(_error) = validate_user_input(&cli) {
        // Print help and exit - this is handled in validate_user_input
        Cli::command().print_help()?;
        println!();
        std::process::exit(1);
    }

    // Validate year range
    if let Err(error) = validate_year_range(cli.start_year, cli.end_year) {
        print_validation_error(&error);
        std::process::exit(1);
    }

    // Setup logging
    setup_logging()?;

    // Validate OS
    validate_os()?;

    // Validate source and get effective source
    let effective_source = match validate_source(&cli) {
        Ok(source) => source,
        Err(error) => {
            print_validation_error(&error);
            Cli::command().print_help()?;
            println!();
            std::process::exit(1);
        }
    };

    // Execute appropriate mode
    if cli.db_path.is_some()
        || (effective_source == "colnectexport"
            && (cli.colnect_csv.is_some() || cli.colnect_inbox.is_some()))
    {
        // Database-only mode (with explicit db-path or colnectexport with CSV that can infer path)
        run_database_mode(&cli, show_progress)?;
    } else if effective_source == "stampworld" || effective_source == "us1847" {
        // Scraping mode
        // Tests may set `SKIP_NETWORK_SCRAPE` to avoid performing live
        // network operations, but we must still validate required CLI
        // options (like --site-name) and print the same helpful error
        // messages as the real scraping path would. If SKIP_NETWORK_SCRAPE
        // is set we will perform validation but skip executing the network
        // heavy `run_scraping_mode`.
        if std::env::var("SKIP_NETWORK_SCRAPE").is_ok() {
            // Mirror the missing site-name handling from `run_scraping_mode`
            if cli.site_name.is_none() && cli.album_name.is_none() {
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
            }
            println!(
                "Skipping {} scraping because SKIP_NETWORK_SCRAPE is set",
                if effective_source == "stampworld" {
                    "StampWorld"
                } else {
                    "US1847"
                }
            );
        } else {
            run_scraping_mode(&cli, show_progress, &effective_source)?;
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
