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
    review_ui::{
        log_runtime_event, run_review_ui, run_review_ui_bootstrap, ReviewUiBootstrapOutcome,
        ReviewUiRunOutcome,
    },
    setup::{
        align_process_working_dir_to_runtime_layout, ensure_linux_desktop_integration,
        ensure_windows_desktop_integration, install_linux_desktop_integration, setup_logging,
        startup_probe_path, uninstall_linux_desktop_integration, validate_os, write_startup_probe,
        write_startup_probe_snapshot,
    },
    validation::{
        print_validation_error, validate_source, validate_user_input, validate_year_range,
    },
};
use std::io::IsTerminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::panic::set_hook(Box::new(|panic_info| {
        let location = panic_info
            .location()
            .map(|loc| format!("{}:{}", loc.file(), loc.line()))
            .unwrap_or_else(|| "<unknown>".to_string());
        let payload = if let Some(msg) = panic_info.payload().downcast_ref::<&str>() {
            (*msg).to_string()
        } else if let Some(msg) = panic_info.payload().downcast_ref::<String>() {
            msg.clone()
        } else {
            "non-string panic payload".to_string()
        };
        let detail = format!("panic at {}: {}", location, payload);
        eprintln!("{}", detail);
        write_startup_probe("panic", &detail);
        log_runtime_event(&format!("FATAL: {}", detail));
    }));

    let raw_args: Vec<String> = std::env::args_os()
        .map(|a| a.to_string_lossy().to_string())
        .collect();
    let launched_without_args = raw_args.len() <= 1;
    let interactive_terminal = std::io::stdin().is_terminal() && std::io::stderr().is_terminal();
    // Detect a GUI desktop session launched without a terminal (e.g. file manager click).
    // Explicitly excluded when the binary lives inside a `target/` directory, which means
    // we are running under `cargo test` or a dev build — in that context we never want to
    // auto-launch the Review UI just because DISPLAY is set.
    let running_from_target = std::env::current_exe()
        .map(|p| {
            p.components().any(|c| {
                c.as_os_str()
                    .to_string_lossy()
                    .eq_ignore_ascii_case("target")
            })
        })
        .unwrap_or(false);
    let gui_session = !interactive_terminal
        && !running_from_target
        && (std::env::var("DISPLAY")
            .map(|v| !v.trim().is_empty())
            .unwrap_or(false)
            || std::env::var("WAYLAND_DISPLAY")
                .map(|v| !v.trim().is_empty())
                .unwrap_or(false));
    write_startup_probe(
        "startup.begin",
        &format!("probe_path={}", startup_probe_path().to_string_lossy()),
    );
    write_startup_probe_snapshot(&raw_args, launched_without_args, interactive_terminal);

    // Parse CLI arguments
    let mut cli = Cli::parse();
    write_startup_probe("cli.parse", "CLI arguments parsed successfully");

    if cli.uninstall_desktop_launcher {
        match uninstall_linux_desktop_integration() {
            Ok(report) => {
                println!(
                    "{} (desktop entry: {}, icon: {})",
                    report.summary, report.desktop_entry_path, report.icon_path
                );
                return Ok(());
            }
            Err(e) => {
                eprintln!("Failed to uninstall desktop launcher artifacts: {}", e);
                std::process::exit(1);
            }
        }
    }

    if cli.install_desktop_launcher {
        match install_linux_desktop_integration() {
            Ok(report) => {
                println!(
                    "{} (desktop entry: {}, icon: {})",
                    report.summary, report.desktop_entry_path, report.icon_path
                );
                return Ok(());
            }
            Err(e) => {
                eprintln!("Failed to install desktop launcher artifacts: {}", e);
                std::process::exit(1);
            }
        }
    }

    let aligned_runtime_dir = match align_process_working_dir_to_runtime_layout() {
        Ok(dir) => {
            let detail = dir
                .as_ref()
                .map(|p| format!("aligned_to={}", p.to_string_lossy()))
                .unwrap_or_else(|| "alignment_not_needed".to_string());
            write_startup_probe("runtime.align", &detail);
            dir
        }
        Err(e) => {
            write_startup_probe("runtime.align.error", &e.to_string());
            return Err(Box::new(e));
        }
    };

    if launched_without_args && (interactive_terminal || gui_session) {
        cli.review_ui = true;
        write_startup_probe(
            "review_ui.auto_enable",
            &format!(
                "enabled_by={}",
                if interactive_terminal {
                    "interactive_terminal"
                } else {
                    "gui_session"
                }
            ),
        );
    }

    let startup_config = config::load_config("config.toml");
    // Always ensure the template file exists, even when config.toml was found and loaded
    // successfully. This covers the case where a user deletes config_template.toml after
    // the initial setup — it will be silently recreated from the embedded defaults on the
    // next run without touching their existing config.toml.
    if let Err(e) = config::ensure_config_and_template_files("config.toml", "config_template.toml")
    {
        log::warn!("Failed to ensure config template file: {}", e);
    }
    write_startup_probe("config.load", "loaded config.toml");
    let enable_pdf_dx_logging = startup_config.pdf_dx_logging_enabled() || cli.pdf_dx_logging;
    stamp_album_gen2::album_globals::set_pdf_dx_logging_enabled(enable_pdf_dx_logging);
    if let Some(path) = aligned_runtime_dir.as_ref() {
        eprintln!(
            "Runtime working directory aligned to {}",
            path.to_string_lossy()
        );
    }
    if enable_pdf_dx_logging {
        eprintln!(
            "PDF DX logging is enabled (via {}); detailed PDF generation diagnostics will be written to Logs/",
            if cli.pdf_dx_logging {
                "--pdf-dx-logging"
            } else {
                "runtime.pdf_dx_logging=true"
            }
        );
    }

    // Review UI mode is intentionally independent from normal source/mode
    // validation to keep the workflow lightweight.
    if cli.review_ui {
        write_startup_probe("review_ui.enter", "starting review-ui mode");
        validate_os()?;

        if launched_without_args {
            log_runtime_event(
                "No CLI arguments provided; launching Review UI startup screen by default.",
            );
        }

        match ensure_linux_desktop_integration() {
            Ok(report) => {
                if report.changed {
                    log_runtime_event(&format!(
                        "Desktop integration installed: {} (desktop entry: {}, icon: {})",
                        report.summary, report.desktop_entry_path, report.icon_path
                    ));
                } else {
                    log_runtime_event(&format!("Desktop integration check: {}", report.summary));
                }
            }
            Err(e) => {
                log_runtime_event(&format!(
                    "Desktop integration check failed (non-fatal): {}",
                    e
                ));
            }
        }

        match ensure_windows_desktop_integration() {
            Ok(report) => {
                if report.changed {
                    log_runtime_event(&format!(
                        "Desktop shortcut installed: {} (desktop: {}, start_menu: {}, icon: {})",
                        report.summary,
                        report.desktop_shortcut_path,
                        report.start_menu_shortcut_path,
                        report.icon_path
                    ));
                } else {
                    log_runtime_event(&format!("Desktop shortcut check: {}", report.summary));
                }
            }
            Err(e) => {
                log_runtime_event(&format!("Desktop shortcut check failed (non-fatal): {}", e));
            }
        }

        log_runtime_event("Review UI: initializing...");
        write_startup_probe("review_ui.bootstrap", "initializing startup selector flow");

        enum ReviewUiLaunchState {
            Bootstrap {
                open_browser: bool,
            },
            Selected {
                db_path: String,
                album_name: Option<String>,
                open_browser: bool,
                prebound: Option<(tiny_http::Server, String)>,
            },
        }

        let mut launch_state = if cli.db_path.is_none() && cli.album_name.is_none() {
            ReviewUiLaunchState::Bootstrap { open_browser: true }
        } else {
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
            log_runtime_event(&format!("Review UI: using requested database {}", db_path));
            ReviewUiLaunchState::Selected {
                db_path,
                album_name: cli.album_name.clone(),
                open_browser: true,
                prebound: None,
            }
        };

        loop {
            let review_cfg = config::load_config("config.toml");
            let current_state = std::mem::replace(
                &mut launch_state,
                ReviewUiLaunchState::Bootstrap { open_browser: true },
            );

            match current_state {
                ReviewUiLaunchState::Bootstrap { open_browser } => {
                    log_runtime_event(
                        "Review UI: no country/database selected on CLI; opening startup selector..."
                    );
                    let bootstrap_outcome = match run_review_ui_bootstrap(
                        cli.review_ui_bind.as_deref(),
                        review_cfg.effective_colnect_inbox_path(),
                        open_browser,
                    ) {
                        Ok(outcome) => outcome,
                        Err(e) => {
                            let detail = format!("Review UI bootstrap failed: {}", e);
                            log_runtime_event(&detail);
                            write_startup_probe("review_ui.bootstrap.error", &detail);
                            return Err(e);
                        }
                    };
                    match bootstrap_outcome {
                        ReviewUiBootstrapOutcome::Exit => {
                            log_runtime_event("No country selected; exiting Review UI startup.");
                            return Ok(());
                        }
                        ReviewUiBootstrapOutcome::ReloadRequested => {
                            log_runtime_event(
                                "Review UI: config changed in startup selector; reloading...",
                            );
                            launch_state = ReviewUiLaunchState::Bootstrap {
                                open_browser: false,
                            };
                            continue;
                        }
                        ReviewUiBootstrapOutcome::Selected {
                            db_path,
                            country,
                            server,
                            bind,
                        } => {
                            log_runtime_event(&format!(
                                "Review UI: selected '{}' with database {}",
                                country, db_path
                            ));
                            launch_state = ReviewUiLaunchState::Selected {
                                db_path,
                                album_name: Some(country),
                                open_browser: false,
                                prebound: Some((server, bind)),
                            };
                        }
                    }
                }
                ReviewUiLaunchState::Selected {
                    db_path,
                    album_name,
                    open_browser,
                    prebound,
                } => {
                    // Re-load config, merging any album-local config.toml override.
                    let album_cfg = {
                        let fallback_slug =
                            album_name.as_deref().unwrap_or("").trim().replace(' ', "-");
                        let album_dir = std::path::Path::new(&db_path)
                            .parent()
                            .map(|p| p.to_path_buf())
                            .unwrap_or_else(|| std::path::Path::new("Albums").join(&fallback_slug));
                        config::load_config_with_override("config.toml", &album_dir)
                    };
                    log_runtime_event(
                        "Review UI: loading compare data. This may take a few seconds...",
                    );
                    let run_outcome = match run_review_ui(
                        &db_path,
                        album_name.as_deref(),
                        prebound,
                        cli.review_ui_bind.as_deref(),
                        open_browser,
                        &album_cfg.effective_default_catalog_code(),
                        &album_cfg.review_ui_default_sort_mode(),
                        album_cfg.review_ui_odd_row_color(),
                        album_cfg.review_ui_even_row_color(),
                        album_cfg.review_ui_series_default_checked(),
                        album_cfg.review_ui_description_default_checked(),
                        album_cfg.review_ui_watermark_default_checked(),
                        album_cfg.review_ui_design_default_checked(),
                        album_cfg.review_ui_engraving_default_checked(),
                        album_cfg.review_ui_perforation_default_checked(),
                        album_cfg.review_ui_context_default_checked(),
                        album_cfg.review_ui_colnect_links_default_checked(),
                        album_cfg.review_ui_colnect_image_url_default_checked(),
                        album_cfg.review_ui_colnect_metadata_default_checked(),
                        album_cfg.review_ui_replace_sg_number_default_checked(),
                        album_cfg.review_ui_right_unmatched_only_default_checked(),
                        album_cfg.albumeasy_auto_pdf_enabled(),
                        album_cfg.effective_albumeasy_cli_path().is_some(),
                        &album_cfg.review_ui_catalog_family_options(),
                    ) {
                        Ok(outcome) => outcome,
                        Err(e) => {
                            let detail =
                                format!("Review UI compare startup failed for {}: {}", db_path, e);
                            log_runtime_event(&detail);
                            write_startup_probe("review_ui.compare.error", &detail);
                            return Err(e);
                        }
                    };
                    match run_outcome {
                        ReviewUiRunOutcome::Exit => return Ok(()),
                        ReviewUiRunOutcome::ReloadRequested => {
                            log_runtime_event(
                                "Review UI: config changed in compare view; reloading...",
                            );
                            launch_state = ReviewUiLaunchState::Selected {
                                db_path,
                                album_name,
                                open_browser: false,
                                prebound: None,
                            };
                            continue;
                        }
                        ReviewUiRunOutcome::ReturnToStartRequested => {
                            log_runtime_event(
                                "Review UI: return-to-start requested from compare view; opening startup selector..."
                            );
                            launch_state = ReviewUiLaunchState::Bootstrap {
                                open_browser: false,
                            };
                            continue;
                        }
                    }
                }
            }
        }
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
    write_startup_probe("config.load", "loaded config.toml for non-review flow");
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
    match setup_logging() {
        Ok(_) => {
            write_startup_probe("logging.setup", "flexi_logger initialized");
        }
        Err(e) => {
            write_startup_probe("logging.setup.error", &e.to_string());
            return Err(Box::new(e));
        }
    }

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
