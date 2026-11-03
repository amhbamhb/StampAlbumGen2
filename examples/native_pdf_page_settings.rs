use stamp_album_gen2::native_pdf::{config::NativePdfConfig, renderer::write_page_geometry_proof};
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = std::env::args_os().skip(1);
    let output = PathBuf::from(arguments.next().ok_or(
        "Usage: cargo run --example native_pdf_page_settings -- <new-output.pdf> [config-path]",
    )?);
    let config_path = arguments
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("config-pdf.toml"));
    if arguments.next().is_some() {
        return Err("Expected an output PDF path and an optional config path".into());
    }
    if output.exists() {
        return Err(format!("Refusing to overwrite {}", output.display()).into());
    }
    let config = NativePdfConfig::load(&config_path)?;
    if !config.overridden_keys.is_empty() {
        eprintln!(
            "Config [page]/[colors] overrides root settings: {}",
            config.overridden_keys.join(", ")
        );
    }
    if !config.unapplied_keys.is_empty() {
        eprintln!("Native proof applies only page size, margins/even margins, border, border colour and spacing; {} other config settings are not applied (including any text, fonts, headers, footers, images and runtime options).", config.unapplied_keys.len());
    }
    write_page_geometry_proof(&config.page_settings, 3, &output)?;
    println!("Created {} using {}: three geometry-only pages; configured borders plus a 0.1 mm content guide; no text/rows. Vertical inter-element gap state: {} mm.", output.display(), config_path.display(), config.page_settings.spacing().vertical_mm());
    Ok(())
}
