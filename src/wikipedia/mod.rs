// Wikipedia content management module
// This module provides functionality for fetching, caching, and formatting
// Wikipedia and Grokipedia content for stamp album generation.

pub mod database;
pub mod fetchers;
pub mod formatters;
pub mod legacy;
pub mod models;
pub mod parsers;
pub mod utils;

// Re-export public API for convenience
pub use database::{fetch_and_cache_content, get_cached_content};
pub use fetchers::{fetch_grokipedia_content, fetch_wikipedia_content};
pub use formatters::{format_content_for_album, format_wikipedia_content_for_album};
pub use legacy::{
    fetch_and_cache_wikipedia_content, fetch_and_save_summary, fetch_and_save_summary_with_fetcher,
    get_cached_wikipedia_content,
};
pub use models::ContentSource;
pub use parsers::{parse_grokipedia_html, parse_wikipedia_html};
pub use utils::{estimate_header_height, estimate_text_height};
