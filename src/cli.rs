use clap::Parser;

/// CLI argument structure for StampAlbumGen2
#[derive(Parser, Debug)]
#[command(
    name = "stamp_album_gen2",
    author,
    version,
    about = "Stamp Album Generator"
)]
pub struct Cli {
    /// Source type: stampworld, touchstamps, extractfile, colnectexport
    #[arg(long, default_value = "stampworld")]
    pub source: String,

    /// Site name for scraping (e.g. 'Great-Britain', 'Canada')
    #[arg(long)]
    pub site_name: Option<String>,

    /// Album name to generate
    #[arg(long)]
    pub album_name: Option<String>,

    /// Database path (for output-only mode)
    #[arg(long)]
    pub db_path: Option<String>,

    /// Optional base URL to use when scraping StampWorld (overrides default construction)
    #[arg(long)]
    pub base_url: Option<String>,

    #[arg(long, default_value_t = 1840)]
    pub start_year: i32,

    // Default end-year set at compile time to the current year via build.rs -> BUILD_YEAR
    #[arg(long, default_value = env!("BUILD_YEAR"))]
    pub end_year: i32,

    #[arg(long, default_value_t = false)]
    pub overwrite_album: bool,

    #[arg(long, default_value_t = false)]
    pub overwrite_images: bool,

    /// Maximum number of pages to fetch when scraping StampWorld (0 = no explicit cap)
    #[arg(long, default_value_t = 0)]
    pub max_pages: i32,

    /// Stop scraping after this many consecutive pages that yielded zero matching stamps (default 3)
    #[arg(long, default_value_t = 3)]
    pub stop_after_no_hits: i32,

    /// When source appears misspelled, accept the top suggestion automatically
    /// (useful in scripts). This is the same as answering 'yes' to a prompt.
    #[arg(long = "yes", short = 'y', default_value_t = false)]
    pub accept_top_suggestion: bool,

    /// Create a `config.toml` from the bundled template (or defaults) and exit.
    /// Use --init-config --force-init-config to overwrite an existing file (with backup).
    #[arg(long, default_value_t = false)]
    pub init_config: bool,

    /// When used with --init-config, force overwriting an existing config.toml
    /// after creating a timestamped backup. Default: false
    #[arg(long = "force-init-config", default_value_t = false)]
    pub force_init_config: bool,

    /// Path to Colnect CSV file for ingestion
    #[arg(long)]
    pub colnect_csv: Option<String>,
}
