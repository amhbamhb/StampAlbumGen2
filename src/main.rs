use indicatif::{ProgressBar, ProgressStyle};
// Main entry point for StampAlbumGen2

use flexi_logger::{Duplicate, FileSpec, Logger};
use stamp_album_gen2::album_globals::OSType;
use stamp_album_gen2::commands_file_generator::CommandsFileGenerator;
use clap::Parser;

mod db_utils;
// mod album_text_generator;

fn get_operating_system_type() -> OSType {
    let os = std::env::consts::OS;
    match os {
        "windows" => OSType::Windows,
        "macos" => OSType::MacOS,
        "linux" => OSType::Linux,
        _ => OSType::Unknown,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // If site_name is provided, use it for country
    let country = if let Some(ref site) = cli.site_name {
        site.clone()
    } else {
        cli.country.clone()
    };
    
    // Construct album name from country
    let album_name = cli.album_name.unwrap_or_else(|| country.replace(' ', "-"));
    
    // Construct site name from country if not provided
    let site_name = cli.site_name.unwrap_or_else(|| country.clone());
    
    // Construct DB path if not provided - use album_name for database folder/file
    let db_path = cli.db_path.unwrap_or_else(|| format!("Albums/{}/{}.db", album_name, album_name));
    
    // Set the DB path for all modules to use
    stamp_album_gen2::album_globals::set_db_path_from_cli(&db_path);
    /// Stamp Album Generator CLI
    #[derive(Parser, Debug)]
    #[command(
        author,
        version,
        about = "Stamp Album Generator: Scrape, filter, and output AlbumEasy files from stamp DBs.",
        long_about = "Examples:\n\
  1. Output AlbumEasy from an existing DB:\n\
     cargo run -- --output-only --db-path Albums/{country}/{country}.db --album-name '{country}'\n\
  2. Scrape and generate AlbumEasy for Great Britain, 1902-1910:\n\
     cargo run -- --source stampworld --site-name '{country}' --album-name '{country}' --start-year 1902 --end-year 1910\n\
Options:\n\
    --output-only           Output AlbumEasy from an existing DB (requires --db-path and --album-name)\n\
    --db-path <path>        Path to the SQLite database (default: Albums/{country}/{country}.db)\n\
    --album-name <name>     Album name (used for output and folder) (default: {country})\n\
    --source <type>         Data source: stampworld, touchstamps, extractfile, colnectexport (default: stampworld)\n\
    --site-name <name>      Site name for scraping (e.g. 'Great-Britain') (default: {country})\n\
    --album-directory <dir> Output directory for album files (default: Albums/{country})\n\
    --images-directory <dir> Output directory for images (default: Albums/{country}/images)\n\
    --album-file-name <file> Output file name for AlbumEasy (default: Albums/{country}/{country}.txt)\n\
    --extract-file-name <file> Output file name for extracted data (default: Albums/{country}/{country}_extract.txt)\n\
    --catalog-name <name>   Catalog name (e.g. 'SG') (default: SG)\n\
    --overwrite             Overwrite existing files\n\
    --start-year <year>     Filter: start year (default: 0)\n\
    --end-year <year>       Filter: end year (default: 9999)\n"
    )]
    struct Cli {
        /// Output-only mode: <db_path> <album_name>
        #[arg(long, default_value_t = false)]
        output_only: bool,

        /// Country name (required for album generation)
        #[arg(long, default_value = "Great-Britain")]
        country: String,

        /// Path to the database
        #[arg(long)]
        db_path: Option<String>,

        /// Album name (auto-generated as Album-{country} if not specified)
        #[arg(long)]
        album_name: Option<String>,

        /// Source type (stampworld, touchstamps, extractfile, colnectexport)
        #[arg(long, default_value = "stampworld")]
        source: String,

        /// Site name (for stampworld, defaults to country)
        #[arg(long)]
        site_name: Option<String>,

        /// Album directory
        #[arg(long)]
        album_directory: Option<String>,

        /// Images directory
        #[arg(long)]
        images_directory: Option<String>,

        /// Album file name
        #[arg(long)]
        album_file_name: Option<String>,

        /// Extract file name
        #[arg(long)]
        extract_file_name: Option<String>,

        /// Catalog name
        #[arg(long, default_value = "SG")]
        catalog_name: String,

        /// Overwrite existing files
        #[arg(long, default_value_t = false)]
        overwrite: bool,

        /// Start year for filtering stamps
        #[arg(long, default_value_t = 0)]
        start_year: i32,

        /// End year for filtering stamps
        #[arg(long, default_value_t = 9999)]
        end_year: i32,
    }

    let cli = Cli::parse();
    
    // Construct album name from country if not provided
    let album_name = cli.album_name.unwrap_or_else(|| country.replace(' ', "-"));
    
    // Construct site name from country if not provided
    let site_name = cli.site_name.unwrap_or_else(|| country.clone());
    
    // Construct DB path if not provided
    let db_path = cli.db_path.unwrap_or_else(|| format!("Albums/{}/{}.db", album_name, album_name));
    
    // Construct other paths based on album name
    let album_directory = cli.album_directory.unwrap_or_else(|| format!("Albums/{}", album_name));
    let images_directory = cli.images_directory.unwrap_or_else(|| format!("Albums/{}/images", album_name));
    let album_file_name = cli.album_file_name.unwrap_or_else(|| format!("Albums/{}/{}.txt", album_name, album_name));
    let extract_file_name = cli.extract_file_name.unwrap_or_else(|| format!("Albums/{}/{}_extract.txt", album_name, album_name));
    
    let start_year = cli.start_year;
    let end_year = cli.end_year;
    println!("[INFO] Stamp Album Generator started.");
    println!("[INFO] Country: {}", country);
    println!("[INFO] Album name: {}", album_name);
    println!("[INFO] Database path: {}", db_path);
    println!("[INFO] Year filter: {} to {}", start_year, end_year);

    let detected_os = get_operating_system_type();
    // If RUST_LOG=debug, log to file and terminal; else, just terminal
    let log_level = std::env::var("RUST_LOG").unwrap_or_default();
    if log_level == "debug" {
        Logger::try_with_str("debug")?
            .log_to_file(FileSpec::default())
            .duplicate_to_stderr(Duplicate::Debug)
            .start()?;
    } else {
        Logger::try_with_env_or_str("info")?
            .log_to_stderr()
            .start()?;
    }
    if detected_os != OSType::Windows
        && detected_os != OSType::MacOS
        && detected_os != OSType::Linux
    {
        println!("Unsupported operating system");
        std::process::exit(1);
    }

    // Output-only mode: cargo run -- --output-only --db-path <db_path> --album-name <album_name>
    if cli.output_only {
        println!("[INFO] Output-only mode selected. Generating AlbumEasy output from database.");
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner} Generating AlbumEasy output... {msg}")
                .unwrap(),
        );
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb.set_message(format!("Scraping data for album '{}', please wait...", album_name));
        let mut last_page = 0;
        println!("[INFO] Loading stamps from database: {}", db_path);
        let all_stamps = stamp_album_gen2::album_globals::load_stamps_from_db(&db_path);
        println!("[INFO] Filtering stamps by year: {} to {}", start_year, end_year);
        let filtered_stamps = all_stamps.into_iter().filter(|stamp| {
            let year = stamp.year;
            year >= start_year && year <= end_year
        }).collect::<Vec<_>>();
        println!("[INFO] Generating AlbumEasy output for album: {}", album_name);
        stamp_album_gen2::album_globals::generate_album_output_from_stamps(
            &filtered_stamps,
            &album_name,
            Some(Box::new(|page| {
                last_page = page;
                pb.set_message(format!(
                    "Generating AlbumEasy output for album '{}', page {}...",
                    album_name, page
                ));
            })),
        );
        pb.finish_with_message(format!(
            "AlbumEasy output generated for album '{}'. Total pages: {}",
            album_name, last_page
        ));
        println!("[INFO] AlbumEasy output complete. Output file: {}", album_file_name);
        return Ok(());
    }

    // Scrape-and-output workflow (default mode)
    // Scrape-and-output workflow (default mode)
    if !cli.output_only {
        println!("[INFO] Scrape-and-output mode selected. Starting data collection and AlbumEasy generation.");
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner} Scraping and generating AlbumEasy output... {msg}")
                .unwrap(),
        );
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb.set_message(format!("for album '{}', please wait...", album_name));
        let mut gen = CommandsFileGenerator::new();
        gen.album_name = album_name.clone();
        gen.album_directory = album_directory.clone();
        gen.images_directory = images_directory.clone();
        gen.album_file_name = album_file_name.clone();
        gen.extract_file_name = extract_file_name.clone();
        gen.catalog_name = cli.catalog_name.clone();
        gen.start_year = cli.start_year;
        gen.end_year = cli.end_year;
        println!("[INFO] Ensuring album and images directories exist: {}", gen.images_directory);
        std::fs::create_dir_all(&gen.images_directory).unwrap_or_else(|e| {
            println!("[ERROR] Failed to create images directory: {}", e);
        });
        // Scrape and save data
        println!("[INFO] Scraping data for site: {} (source: {})", site_name, cli.source);
        // Determine output file name based on year filters
        let mut output_file_name = album_name.clone();
        if cli.start_year > 0 && cli.end_year < 9999 {
            output_file_name = format!("{}-{}-{}.txt", album_name, cli.start_year, cli.end_year);
        } else if cli.start_year > 0 {
            output_file_name = format!("{}-{}-current.txt", album_name, cli.start_year);
        } else if cli.end_year < 9999 {
            output_file_name = format!("{}-XXXX-{}.txt", album_name, cli.end_year);
        } else {
            output_file_name = format!("{}.txt", album_name);
        }
        let album_file_path = format!("{}/{}", album_directory, output_file_name);
        let images_dir = format!("{}/images", album_directory);
        std::fs::create_dir_all(&images_dir).unwrap_or_else(|e| {
            println!("[ERROR] Failed to create images directory: {}", e);
        });
        match cli.source.as_str() {
            "stampworld" => {
                println!("[INFO] Collecting data from StampWorld...");
                let base_url = format!(
                    "https://www.stampworld.com/en/stamps/{}/Postage%20stamps/{}-{}?user=676374&page=",
                    site_name, cli.start_year, cli.end_year
                );
                println!("[DEBUG] Using base URL: {}", base_url);
                let data = gen.collect_stampworld_data(&base_url, cli.overwrite, Some(&|page| {
                    pb.set_message(format!("Scraping data for album '{}', page {}...", album_name, page));
                }));
                pb.set_message("Scraping complete. Saving extracted data...");
                println!("[INFO] Saving extracted data to file: {}", extract_file_name);
                gen.save_stamp_data(&extract_file_name, &data, "stampworld");
                pb.set_message("Data saved. Creating AlbumEasy file...");
                println!("[INFO] Creating AlbumEasy file: {}", album_file_path);
                gen.create_album(&album_file_path, &data);
                pb.finish_with_message("Scraping and data save complete.");

                // Start new spinner for AlbumEasy output generation
                let pb2 = ProgressBar::new_spinner();
                pb2.set_style(ProgressStyle::default_spinner()
                    .template("{spinner} Generating AlbumEasy output... {msg}")
                    .unwrap());
                pb2.enable_steady_tick(std::time::Duration::from_millis(100));
                pb2.set_message(format!("Generating AlbumEasy output for album '{}', please wait...", gen.album_name));

                println!("[INFO] Loading stamps from database: {}", db_path);
                let mut last_page = 0;
                let all_stamps = stamp_album_gen2::album_globals::load_stamps_from_db(&db_path);
                println!("[INFO] Filtering stamps by year: {} to {}", gen.start_year, gen.end_year);
                let filtered_stamps = all_stamps.into_iter().filter(|stamp| {
                    let year = stamp.year;
                    year >= gen.start_year && year <= gen.end_year
                }).collect::<Vec<_>>();
                println!("[INFO] Generating AlbumEasy output for album: {}", gen.album_name);
                stamp_album_gen2::album_globals::generate_album_output_from_stamps(
                    &filtered_stamps,
                    &gen.album_name,
                    Some(Box::new(|page| {
                        last_page = page;
                        pb2.set_message(format!(
                            "Generating AlbumEasy output for album '{}', page {}...",
                            gen.album_name, page
                        ));
                    })),
                );
                pb2.finish_with_message(format!(
                    "AlbumEasy output generated for album '{}'. Total pages: {}",
                    gen.album_name, last_page
                ));
                println!("[INFO] AlbumEasy output complete. Output file: {}", gen.album_file_name);
                return Ok(());
            },
            _ => {
                println!("[ERROR] Source '{}' is not yet implemented. Only 'stampworld' is supported.", cli.source);
                std::process::exit(1);
            }
        }
    }
    Ok(())
}
