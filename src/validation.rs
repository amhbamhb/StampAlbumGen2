use strsim::jaro_winkler;

/// Validation errors that can occur during input validation
#[derive(Debug)]
pub enum ValidationError {
    NoInputProvided,
    InvalidYearRange {
        start_year: i32,
        end_year: i32,
    },
    UnrecognizedSource {
        source: String,
        suggestions: Vec<(String, f64)>,
    },
    DatabasePathError(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::NoInputProvided => write!(f, "No meaningful input provided"),
            ValidationError::InvalidYearRange {
                start_year,
                end_year,
            } => {
                write!(
                    f,
                    "Invalid year range: start_year ({}) > end_year ({})",
                    start_year, end_year
                )
            }
            ValidationError::UnrecognizedSource { source, .. } => {
                write!(f, "Unrecognized source '{}'", source)
            }
            ValidationError::DatabasePathError(msg) => write!(f, "Database path error: {}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validate that the user provided meaningful input
pub fn validate_user_input(cli: &crate::cli::Cli) -> Result<(), ValidationError> {
    let source_non_default = cli.source.to_lowercase() != "stampworld";
    let user_provided_any = cli.db_path.is_some()
        || cli.album_name.is_some()
        || cli.site_name.is_some()
        || cli.source_file.is_some()
        || cli.base_url.is_some()
        || cli.colnect_csv.is_some()
        || source_non_default
        || cli.start_year != 1840
        || cli.end_year != crate::runtime_defaults::default_end_year()
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

        // This will be handled by the caller to print help and exit
        return Err(ValidationError::NoInputProvided);
    }

    Ok(())
}

/// Validate year range
pub fn validate_year_range(start_year: i32, end_year: i32) -> Result<(), ValidationError> {
    if start_year > end_year {
        return Err(ValidationError::InvalidYearRange {
            start_year,
            end_year,
        });
    }
    Ok(())
}

/// Validate source and return effective source (potentially auto-corrected)
pub fn validate_source(cli: &crate::cli::Cli) -> Result<String, ValidationError> {
    let known_sources = [
        "stampworld",
        "touchstamps",
        "extractfile",
        "colnectexport",
        "us1847",
    ];
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
                    use std::io::IsTerminal;
                    if std::io::stdin().is_terminal() && std::io::stdout().is_terminal() {
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
        // return error with suggestions
        if !known_sources
            .iter()
            .any(|s| s == &effective_source.as_str())
        {
            // Prepare a human-friendly top-3 list with numeric Jaro-Winkler scores.
            let mut top3: Vec<(String, f64)> = scored
                .iter()
                .take(3)
                .map(|(s, score)| (s.to_string(), *score))
                .collect();
            // Filter low scores and keep only those > 0.0 (we'll control messaging)
            top3.retain(|(_, score)| *score > 0.0);

            return Err(ValidationError::UnrecognizedSource {
                source: cli.source.clone(),
                suggestions: top3,
            });
        }
    }

    Ok(effective_source)
}

/// Print validation error and help information
pub fn print_validation_error(error: &ValidationError) {
    match error {
        ValidationError::NoInputProvided => {
            // Help was already printed in validate_user_input
        }
        ValidationError::InvalidYearRange {
            start_year,
            end_year,
        } => {
            eprintln!(
                "Invalid year range: --start-year ({}) is greater than --end-year ({}).",
                start_year, end_year
            );
        }
        ValidationError::UnrecognizedSource {
            source,
            suggestions,
        } => {
            eprintln!("Unrecognized source '{}'.", source);
            if !suggestions.is_empty() {
                eprintln!("Top suggestions:");
                for (name, score) in suggestions.iter() {
                    eprintln!("  - {} (score={:.3})", name, score);
                }
                eprintln!(
                    "(Suggestions computed using Jaro–Winkler similarity; higher is better.)"
                );
            } else {
                eprintln!("Valid sources are: stampworld, touchstamps, extractfile, colnectexport, us1847");
            }
            eprintln!("\nHere are quick examples you can copy-paste:");
            eprintln!("  # Generate from an existing DB (db-only):");
            eprintln!("  stamp_album_gen2 --db-path Albums/Canada/Canada.db --album-name Canada");
            eprintln!("\n  # Scrape StampWorld and generate album (scrape-only):");
            eprintln!(
                "  stamp_album_gen2 --source stampworld --site-name 'Great-Britain' --album-name 'Great Britain' --start-year 1900 --end-year 1950\n"
            );
            eprintln!("Full help is shown below:\n");
        }
        ValidationError::DatabasePathError(msg) => {
            eprintln!("Database path error: {}", msg);
        }
    }
}
