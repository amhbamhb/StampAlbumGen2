// Main entry point for StampAlbumGen2

use clap::Parser;
use stamp_album_gen2::types::OSType;
use flexi_logger::Logger;
use stamp_album_gen2::commands_file_generator::CommandsFileGenerator;

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
#[command(name = "stamp_album_gen2", author, version, about = "Stamp Album Generator")]
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

    #[arg(long, default_value_t = 0)]
    start_year: i32,

    #[arg(long, default_value_t = 9999)]
    end_year: i32,

    #[arg(long, default_value_t = false)]
    overwrite_album: bool,

    #[arg(long, default_value_t = false)]
    overwrite_images: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Debug: show parsed CLI early so we can diagnose routing (scrape vs output-only)
    println!("DEBUG: CLI args = {:?}", cli);

    // Basic logging setup
    Logger::try_with_env_or_str("info")?.log_to_stderr().start()?;

    let detected_os = get_operating_system_type();
    if detected_os == OSType::Unknown {
        eprintln!("Unsupported operating system");
        std::process::exit(1);
    }

    // If a database path is explicitly provided, treat this as output-only mode
    // (do not perform a scrape). Otherwise, when source == "stampworld" run
    // the scraper; for other source values require --db-path.
    if cli.db_path.is_some() {
        // Output-only mode: use the provided db_path and album_name
        let db_path = cli.db_path.unwrap();
        let album_name = cli.album_name.unwrap_or_else(|| "Album".to_string());
        stamp_album_gen2::album_globals::generate_album_output(&db_path, &album_name, None::<fn(usize)>, Some(cli.start_year), Some(cli.end_year));
    } else if cli.source.to_lowercase() == "stampworld" {
        // Scrape mode
        // Require site_name for scraping. If album_name is provided but site_name
        // is not, derive site_name from album_name. This avoids accidentally
        // defaulting to 'Israel'.
        let site_name = if let Some(s) = cli.site_name.clone() {
            s
        } else if let Some(a) = cli.album_name.clone() {
            a.clone()
        } else {
            eprintln!("--site-name is required for scraping mode (or provide --album-name to derive it)");
            std::process::exit(1);
        };
        let album_name = cli.album_name.unwrap_or_else(|| site_name.clone());

        println!("Starting StampWorld scrape for site='{}' album='{}' years={}..{} overwrite_images={} overwrite_album={}", site_name, album_name, cli.start_year, cli.end_year, cli.overwrite_images, cli.overwrite_album);

        // Build a CommandsFileGenerator and invoke the collector
        let mut cfg = CommandsFileGenerator::new();
        cfg.site_name = site_name.clone();
        cfg.images_directory = format!("Albums/{}/images", album_name.replace(' ', "-"));
        cfg.album_directory = format!("Albums/{}", album_name.replace(' ', "-"));
        cfg.album_name = album_name.clone();
        cfg.start_year = cli.start_year;
        cfg.end_year = cli.end_year;

    // Build base URL for stampworld pages
        // Use the English-prefixed canonical path: '/en/stamps/{country}/' and let the collector append '?page=N'
        let base_url = format!("https://www.stampworld.com/en/stamps/{}/", site_name.replace(' ', "%20"));
    // Compose the DB path early and set it for the collector so both scraper
    // and generator operate on the same DB file under Albums/{Album}/{Album}.db.
    let db_path = format!("{}/{}.db", cfg.album_directory, album_name.replace('/', "-"));
    stamp_album_gen2::file_utils::set_db_path_from_cli(&db_path);
    println!("DEBUG: collector DB path set to {}", db_path);

    let pages = cfg.collect_stampworld_data(&base_url, cli.overwrite_images, None::<&dyn Fn(usize)>);
    println!("Scrape produced {} stamp lines", pages.len());

    // After scraping, generate album output from the same DB
    stamp_album_gen2::album_globals::generate_album_output(&db_path, &album_name, None::<fn(usize)>, Some(cli.start_year), Some(cli.end_year));
    } else {
        // No db_path and not stampworld: require db_path
        eprintln!("--db-path is required when not using stampworld scraping mode");
        std::process::exit(1);
    }

    Ok(())
}
