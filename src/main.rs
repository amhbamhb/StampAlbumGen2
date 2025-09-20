use indicatif::{ProgressBar, ProgressStyle};
// Main entry point for StampAlbumGen2

use flexi_logger::{Duplicate, FileSpec, Logger};
use stamp_album_gen2::types::OSType;
use stamp_album_gen2::commands_file_generator::CommandsFileGenerator;
use clap::Parser;

mod db_utils;
mod placeholder_image;
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

    // Validate required parameters
    if !cli.output_only && cli.site_name.is_none() {
        eprintln!("Error: --site-name is required when not using --output-only mode");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  # Generate album by scraping:");
        eprintln!("  stamp_album_gen2 --site-name Canada --album-name Canada");
        eprintln!();
        eprintln!("  # Generate from existing database:");
        eprintln!("  stamp_album_gen2 --output-only --db-path Albums/Canada/Canada.db --album-name Canada");
        eprintln!();
        eprintln!("Use --help for more information.");
        std::process::exit(1);
    }

    if cli.output_only && (cli.db_path.is_none() || cli.album_name.is_none()) {
        eprintln!("Error: --output-only mode requires both --db-path and --album-name");
        eprintln!();
        eprintln!("Example:");
        eprintln!("  stamp_album_gen2 --output-only --db-path Albums/Canada/Canada.db --album-name Canada");
        eprintln!();
        std::process::exit(1);
    }

    // Determine country name from various sources
    let country = if let Some(ref site) = cli.site_name {
        site.clone()
    } else if let Some(ref album) = cli.album_name {
        album.clone()
    } else if let Some(ref country_name) = cli.country {
        country_name.clone()
    } else {
        eprintln!("Error: Could not determine country name. Please provide --site-name, --album-name, or --country");
        std::process::exit(1);
    };

    // Construct album name from country if not provided
    let album_name = cli.album_name.unwrap_or_else(|| country.replace(' ', "-"));

    // Construct site name from country if not provided
    let _site_name = cli.site_name.unwrap_or_else(|| country.clone());

    // Construct DB path if not provided - use album_name for database folder/file
    let db_path = cli.db_path.unwrap_or_else(|| format!("Albums/{}/{}.db", album_name, album_name));
    
    // Set the DB path for all modules to use
    stamp_album_gen2::file_utils::set_db_path_from_cli(&db_path);
    /// Stamp Album Generator CLI
    #[derive(Parser, Debug)]
    #[command(
        name = "stamp_album_gen2",
        author,
        version,
        about = "Stamp Album Generator: Scrape, filter, and output AlbumEasy files from stamp DBs.",
        long_about = "A powerful tool for generating stamp albums from various data sources.

Examples:
  1. Output AlbumEasy from an existing DB:
     stamp_album_gen2 --output-only --db-path Albums/Canada/Canada.db --album-name Canada

  2. Scrape and generate AlbumEasy for Great Britain, 1902-1910:
     stamp_album_gen2 --source stampworld --site-name Great-Britain --album-name Great-Britain --start-year 1902 --end-year 1910

  3. Generate album for Canada stamps (1875-1876):
     stamp_album_gen2 --site-name Canada --album-name Canada --start-year 1875 --end-year 1876

Required Parameters:
  You must provide either --site-name (for scraping) or --album-name + --db-path (for existing DB)

Options:"
    )]
    struct Cli {
        /// Output-only mode: Generate AlbumEasy from existing DB (requires --db-path and --album-name)
        #[arg(long, default_value_t = false)]
        output_only: bool,

        /// Country name (used as default for other parameters if not specified)
        #[arg(long)]
        country: Option<String>,

        /// Path to the SQLite database (required for --output-only mode)
        #[arg(long)]
        db_path: Option<String>,

        /// Album name (required for album generation)
        #[arg(long)]
        album_name: Option<String>,

        /// Source type: stampworld, touchstamps, extractfile, colnectexport
        #[arg(long, default_value = "stampworld")]
        source: String,

        /// Site name for scraping (required for scraping mode, e.g. 'Great-Britain', 'Canada')
        #[arg(long)]
        site_name: Option<String>,

        /// Album directory (default: Albums/{album_name})
        #[arg(long)]
        album_directory: Option<String>,

        /// Images directory (default: Albums/{album_name}/images)
        #[arg(long)]
        images_directory: Option<String>,

        /// Album file name (default: Albums/{album_name}/{album_name}.txt)
        #[arg(long)]
        album_file_name: Option<String>,

        /// Extract file name (default: Albums/{album_name}/{album_name}_extract.txt)
        #[arg(long)]
        extract_file_name: Option<String>,

        /// Catalog name (e.g. 'SG', 'Michel')
        #[arg(long, default_value = "SG")]
        catalog_name: String,

        /// Overwrite existing album text files
        #[arg(long, default_value_t = false)]
        overwrite_album: bool,

        /// Overwrite existing image files
        #[arg(long, default_value_t = false)]
        overwrite_images: bool,

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
    
    // Set the database path in the global variable
    stamp_album_gen2::file_utils::set_db_path_from_cli(&db_path);
    
    // Construct other paths based on album name
    let album_directory = cli.album_directory.unwrap_or_else(|| format!("Albums/{}", album_name));
    let images_directory = cli.images_directory.unwrap_or_else(|| format!("Albums/{}/images", album_name));

    let start_year = cli.start_year;
    let end_year = cli.end_year;

    // Construct album file name with year range if year filters are specified
    let album_file_name = cli.album_file_name.unwrap_or_else(|| {
        // Include year range in filename if year filters are used (not default values)
        if start_year != 0 || end_year != 9999 {
            format!("Albums/{}/{}-{}-{}.txt", album_name, album_name, start_year, end_year)
        } else {
            format!("Albums/{}/{}.txt", album_name, album_name)
        }
    });

    let extract_file_name = cli.extract_file_name.unwrap_or_else(|| format!("Albums/{}/{}_extract.txt", album_name, album_name));
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
        println!("[INFO] Loaded {} stamps from database", all_stamps.len());

        // For development purposes, include all stamps initially
        // Year filtering will be applied during album generation if needed
        let filtered_stamps: Vec<_>;

        // Optional: Apply year filtering only if explicitly requested and stamps have year data
        if start_year > 0 || end_year < 9999 {
            println!("[INFO] Applying year filter: {} to {}", start_year, end_year);
            filtered_stamps = all_stamps.into_iter().filter(|stamp| {
                let mut year = stamp.year;

                // If year is 0, try to extract from description
                if year == 0 {
                    let year_regex = regex::Regex::new(r"(18|19|20)\d{2}").unwrap();
                    if let Some(cap) = year_regex.find(&stamp.desc) {
                        if let Ok(extracted_year) = cap.as_str().parse::<i32>() {
                            if extracted_year >= 1800 && extracted_year <= 2100 {
                                year = extracted_year;
                                println!("[DEBUG] Extracted year {} from description '{}' for stamp {}", year, stamp.desc, stamp.sg_number);
                            }
                        }
                    }
                }

                // Include stamp if it has a valid year in range, or if year extraction failed but series suggests it belongs
                let year_range_span = end_year - start_year;
                let matches = if year >= start_year && year <= end_year {
                    true
                } else if year == 0 && year_range_span > 20 {
                    // For broad year ranges, include stamps where year extraction failed
                    // This helps capture stamps that belong to the era but have unclear dating
                    println!("[DEBUG] Including stamp {} with unknown year (year=0) due to broad range", stamp.sg_number);
                    true
                } else {
                    false
                };
                matches
            }).collect::<Vec<_>>();
            println!("[INFO] Filtered to {} stamps within year range", filtered_stamps.len());
        } else {
            filtered_stamps = all_stamps;
        }        // Update filename if only end-year was specified
        let mut final_album_file_name = album_file_name.clone();
        if start_year == 0 && end_year < 9999 && !filtered_stamps.is_empty() {
            // Find the minimum year from filtered stamps
            let min_year = filtered_stamps.iter().map(|stamp| stamp.year).min().unwrap_or(end_year);
            final_album_file_name = format!("Albums/{}/{}-{}-{}.txt", album_name, album_name, min_year, end_year);
            println!("[INFO] Corrected output filename to: {}", final_album_file_name);
        }
        
        // Set the global album file name
        *stamp_album_gen2::types::ALBUM_FILE_NAME.lock().unwrap() = final_album_file_name.clone();
        *stamp_album_gen2::stampworld::state::ALBUM_FILE_NAME.lock().unwrap() = final_album_file_name.clone();

        // Create placeholder image for output-only mode
        println!("[INFO] Creating placeholder image for unknown/blank stamps...");
        match placeholder_image::ensure_placeholder_image_for_album(&album_name) {
            Ok(path) => println!("[INFO] Placeholder image ready: {}", path),
            Err(e) => println!("[WARN] Failed to create placeholder image: {}", e),
        }

        // Download missing images if remote URLs are available
        println!("[INFO] Downloading missing stamp images...");
        match stamp_album_gen2::image_downloader::download_missing_stamp_images(&filtered_stamps, &album_name) {
            Ok(_) => println!("[INFO] Image download process completed"),
            Err(e) => println!("[WARN] Some images failed to download: {}", e),
        }

        println!("[INFO] Generating AlbumEasy output for album: {}", album_name);
        stamp_album_gen2::stampworld::generate_album_output_from_stamps(
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
        println!("[INFO] AlbumEasy output complete. Output file: {}", final_album_file_name);
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

        // Create placeholder image early in the process
        println!("[INFO] Creating placeholder image for unknown/blank stamps...");
        match placeholder_image::ensure_placeholder_image_for_album(&album_name) {
            Ok(path) => println!("[INFO] Placeholder image ready: {}", path),
            Err(e) => println!("[WARN] Failed to create placeholder image: {}", e),
        }
        // Scrape and save data
        println!("[INFO] Scraping data for site: {} (source: {})", site_name, cli.source);
        // Determine output file name based on year filters
        let output_file_name = if cli.start_year > 0 && cli.end_year < 9999 {
            format!("{}-{}-{}.txt", album_name, cli.start_year, cli.end_year)
        } else if cli.start_year > 0 {
            format!("{}-{}-current.txt", album_name, cli.start_year)
        } else if cli.end_year < 9999 {
            // We'll determine the actual first year after loading stamps
            format!("{}-XXXX-{}.txt", album_name, cli.end_year)
        } else {
            format!("{}.txt", album_name)
        };
        let album_file_path = format!("{}/{}", album_directory, output_file_name);
        
        // Update the global album file name for consistency
        *stamp_album_gen2::types::ALBUM_FILE_NAME.lock().unwrap() = album_file_path.clone();
        *stamp_album_gen2::stampworld::state::ALBUM_FILE_NAME.lock().unwrap() = album_file_path.clone();
        
        let mut gen = CommandsFileGenerator::new();
        gen.album_name = album_name.clone();
        gen.album_directory = album_directory.clone();
        gen.images_directory = images_directory.clone();
        gen.album_file_name = album_file_path.clone();  // Use the corrected path with year range
        gen.extract_file_name = extract_file_name.clone();
        gen.site_name = site_name.clone();
        gen.catalog_name = cli.catalog_name.clone();
        gen.start_year = cli.start_year;
        gen.end_year = cli.end_year;
        
        println!("[INFO] Ensuring album and images directories exist: {}", gen.images_directory);
        std::fs::create_dir_all(&gen.images_directory).unwrap_or_else(|e| {
            println!("[ERROR] Failed to create images directory: {}", e);
        });
        
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
                let data = gen.collect_stampworld_data(&base_url, cli.overwrite_images, Some(&|page| {
                    pb.set_message(format!("Scraping data for album '{}', page {}...", album_name, page));
                }));
                pb.set_message("Scraping complete. Processing and saving data...");
                println!("[INFO] Scraping complete. Found {} stamp records", data.len());
                println!("[INFO] Saving extracted data to file: {}", extract_file_name);
                gen.save_stamp_data(&extract_file_name, &data, "stampworld", cli.overwrite_album);
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
                println!("[INFO] Loaded {} stamps from database", all_stamps.len());
                
                // Debug: show some sample stamps
                for (i, stamp) in all_stamps.iter().take(3).enumerate() {
                    println!("[DEBUG] Sample stamp {}: sg={}, year={}, desc='{}'", i, stamp.sg_number, stamp.year, stamp.desc);
                }
                
                println!("[INFO] Filtering stamps by year: {} to {}", gen.start_year, gen.end_year);
                let year_range_span = gen.end_year - gen.start_year;
                let filtered_stamps = all_stamps.into_iter().filter(|stamp| {
                    let mut year = stamp.year;
                    
                    // If year is 0, try to extract from description
                    if year == 0 {
                        let year_regex = regex::Regex::new(r"(18|19|20)\d{2}").unwrap();
                        if let Some(cap) = year_regex.find(&stamp.desc) {
                            if let Ok(extracted_year) = cap.as_str().parse::<i32>() {
                                if extracted_year >= 1800 && extracted_year <= 2100 {
                                    year = extracted_year;
                                    println!("[DEBUG] Extracted year {} from description '{}' for stamp {}", year, stamp.desc, stamp.sg_number);
                                }
                            }
                        }
                    }
                    
                    // For broad year ranges (span > 20 years), include stamps with year=0
                    // or stamps that fall within a reasonable range
                    // Also include stamps where year extraction failed but series suggests they belong
                    let matches = if year_range_span > 20 {
                        // For broad ranges, be very inclusive
                        year == 0 || (year >= gen.start_year - 5 && year <= gen.end_year + 5)
                    } else {
                        // For narrow ranges, be more strict
                        year >= gen.start_year && year <= gen.end_year
                    };
                    matches
                }).collect::<Vec<_>>();
                println!("[INFO] Filtered to {} stamps within year range (year range span: {} years)", filtered_stamps.len(), year_range_span);
                
                // Update filename if only end-year was specified
                let _final_album_file_path = album_file_path.clone();
                if cli.start_year == 0 && cli.end_year < 9999 && !filtered_stamps.is_empty() {
                    // Find the minimum year from filtered stamps
                    let min_year = filtered_stamps.iter().map(|stamp| stamp.year).min().unwrap_or(cli.end_year);
                    let corrected_filename = format!("{}-{}-{}.txt", album_name, min_year, cli.end_year);
                    let _corrected_path = format!("{}/{}", album_directory, corrected_filename);
                    println!("[INFO] Corrected output filename to: {}", corrected_filename);
                }
                
                // Download missing images if remote URLs are available
                println!("[INFO] Downloading missing stamp images...");
                match stamp_album_gen2::image_downloader::download_missing_stamp_images(&filtered_stamps, &gen.album_name) {
                    Ok(_) => println!("[INFO] Image download process completed"),
                    Err(e) => println!("[WARN] Some images failed to download: {}", e),
                }
                
                println!("[INFO] Generating AlbumEasy output for album: {}", gen.album_name);
                stamp_album_gen2::stampworld::generate_album_output_from_stamps(
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
