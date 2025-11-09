// Main entry point for StampAlbumGen2

use clap::{CommandFactory, Parser};
use stamp_album_gen2::{
    cli::Cli,
    config,
    modes::{run_database_mode, run_scraping_mode},
    setup::{setup_logging, validate_os},
    validation::{
        print_validation_error, validate_source, validate_user_input, validate_year_range,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Load config early to check for progress indicator setting
    let config = config::load_config("config.toml");
    let show_progress = config.show_progress_indicator.unwrap_or(true);

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
    if cli.db_path.is_some() {
        // Database-only mode
        run_database_mode(&cli, show_progress)?;
    } else if effective_source == "stampworld" {
        // Scraping mode
        run_scraping_mode(&cli, show_progress)?;
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
