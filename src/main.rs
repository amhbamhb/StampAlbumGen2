// Main entry point for StampAlbumGen2
use std::env;
use stamp_album_gen2::album_globals::OSType;
use stamp_album_gen2::commands_file_generator::CommandsFileGenerator;

mod db_utils;
mod album_text_generator;

fn get_operating_system_type() -> OSType {
    let os = env::consts::OS;
    match os {
        "windows" => OSType::Windows,
        "macos" => OSType::MacOS,
        "linux" => OSType::Linux,
        _ => OSType::Unknown,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    let detected_os = get_operating_system_type();
    if detected_os != OSType::Windows && detected_os != OSType::MacOS && detected_os != OSType::Linux {
        println!("Unsupported operating system");
        std::process::exit(1);
    }

    // Output-only mode: cargo run -- output-only <db_path> <album_name>
    if args.get(0).map(|s| s == "output-only").unwrap_or(false) {
        if args.len() < 3 {
            println!("USAGE: cargo run -- output-only <db_path> <album_name>");
            std::process::exit(1);
        }
        let db_path = &args[1];
        let album_name = &args[2];
        stamp_album_gen2::album_globals::generate_album_output(db_path, album_name);
        println!("[DEBUG] AlbumEasy output generated from DB only.");
        return Ok(());
    }

    if args.len() < 1 {
        println!("USAGE1 : cargo run -- <stampworld|touchstamps> <album name> <source> <directory> [options]");
        println!("EXAMPLE: cargo run -- stampworld \"United Nations New York\" UN-New-York albums");
        println!("USAGE2 : cargo run -- colnectexport <album name> <export file path> [options]");
        println!("EXAMPLE: cargo run -- colnectexport \"United Nations New York\" UN_New_York_export.csv");
        println!("USAGE3 : cargo run -- extractfile <album name> <extract file path> [options]");
        println!("EXAMPLE: cargo run -- extractfile \"United Nations New York\" UN_New_York_export.txt");
        println!("USAGE4 : cargo run -- output-only <db_path> <album_name>");
        println!("EXAMPLE: cargo run -- output-only Albums/Great-Britain/Great-Britain.db \"Great Britain - 4 Kings\"");
        std::process::exit(1);
    } else if args.len() < 2 {
        println!("Missing album name");
        std::process::exit(1);
    }

    let valid_sources = ["stampworld", "touchstamps", "extractfile", "colnectexport"];
    if !valid_sources.iter().any(|s| s.eq_ignore_ascii_case(&args[0])) {
        println!("Valid sources are stampworld, touchstamps, extractfile, and colnectexport");
        std::process::exit(1);
    }

    println!("[DEBUG] Args: {:?}", args);
    println!("[DEBUG] Detected OS: {:?}", detected_os);
    // Test image fetch
    let test_url = "https://upload.wikimedia.org/wikipedia/commons/4/47/PNG_transparency_demonstration_1.png";
    let test_name = "test_image";
    let test_path = "test_image.png";
    let _ = stamp_album_gen2::commands_file_generator::get_image(test_url, test_name, test_path);
    println!("[DEBUG] Test image fetch attempted.");

    // Implement Stampworld connection string and flow
    if args[0].eq_ignore_ascii_case("stampworld") {
        if args.len() < 3 {
            std::process::exit(1);
        } else if args.len() < 4 { /* ...existing code... */ }
        let site_name = &args[2];
        let album_directory = format!("{}/{}", &args[3], site_name.replace("/", "-"));
        let images_directory = format!("{}/images", album_directory);
        let album_file_name = format!("{}/{}.txt", album_directory, args[1].replace(" ", "_"));
        let extract_file_name = format!("{}/{}_extract.txt", album_directory, site_name.replace("/", "-"));
        let catalog_name = "Sw";
        let base_url = format!("https://www.stampworld.com/en/stamps/{}/?page=", site_name);
        println!("[DEBUG] Stampworld base URL: {}", base_url);
        println!("[DEBUG] Album directory: {}", album_directory);
        println!("[DEBUG] Images directory: {}", images_directory);
        println!("[DEBUG] Album file name: {}", album_file_name);
        println!("[DEBUG] Extract file name: {}", extract_file_name);
        // Parse startYear, endYear, overwrite from args
        let mut start_year = 0;
        let mut end_year = 9999;
        let mut overwrite = false;
        for arg in &args[4..] { /* ...existing code... */ }
        // Ensure directories exist
        std::fs::create_dir_all(&images_directory).unwrap_or_else(|e| {
            println!("[DEBUG] Failed to create images directory: {}", e);
        });
        // Collect data and save
        let mut gen = CommandsFileGenerator::new();
        gen.site_name = site_name.clone();
        gen.album_directory = album_directory.clone();
        gen.images_directory = images_directory.clone();
        gen.album_name = args[1].clone();
        gen.album_file_name = album_file_name.clone();
        gen.extract_file_name = extract_file_name.clone();
        gen.catalog_name = catalog_name.to_string();
        gen.start_year = start_year;
        gen.end_year = end_year;
        let stamp_data = gen.collect_stampworld_data(&base_url, overwrite);
        println!("[DEBUG] Stampworld data collected: {} entries", stamp_data.len());
        gen.save_stamp_data(&extract_file_name, &stamp_data, "stampworld");
        gen.create_album(&album_file_name, &stamp_data);
        println!("[DEBUG] Album creation complete.");
        // After scraping is complete, generate AlbumEasy output from DB
        let db_path = format!("{}/{}.db", album_directory, site_name.replace("/", "-"));
        let album_name = args[1].clone();
        stamp_album_gen2::album_globals::generate_album_output(&db_path, &album_name);
        println!("[DEBUG] AlbumEasy output generated from DB after scraping.");
    }
    // Placeholder for the rest of the logic
    // You will need to implement:
    // - get_file_source()
    // - check_options()
    // - collectColnectData(), collectStampworldData(), collectTouchstampsData()
    // - getDataFromFile(), getDataFromExport()
    // - saveStampData(), createAlbum()

    Ok(())
}
