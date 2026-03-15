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
    /// Source type: stampworld, touchstamps, extractfile, colnectexport, us1847
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

    // Default end-year resolved at runtime from system clock.
    #[arg(long, default_value_t = crate::runtime_defaults::default_end_year())]
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

    /// Path to a Colnect inbox folder. When set, matching CSV files are
    /// discovered recursively and processed country-by-country.
    #[arg(long)]
    pub colnect_inbox: Option<String>,

    /// If true, discovered Colnect CSV files are moved into
    /// <inbox>/<country>/ before processing.
    #[arg(long, default_value_t = false)]
    pub colnect_auto_route: bool,

    /// Optional country filter for inbox processing (case-insensitive).
    #[arg(long)]
    pub colnect_country: Option<String>,

    /// Allow creating missing Albums/<Country>/<Country>.db when ingesting
    /// inbox files. If false, missing country DBs are skipped.
    #[arg(long, default_value_t = false)]
    pub colnect_create_country_db: bool,

    /// Skip files already imported (by SHA256 checksum) into the target
    /// country DB. Default true.
    #[arg(long, default_value_t = true)]
    pub colnect_skip_duplicates: bool,

    /// Reimport even if checksum already exists in import history.
    #[arg(long, default_value_t = false)]
    pub colnect_force_reimport: bool,
}
