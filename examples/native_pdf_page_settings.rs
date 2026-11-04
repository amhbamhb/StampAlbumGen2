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
            "Config [page]/[header]/[colors] overrides root settings: {}",
            config.overridden_keys.join(", ")
        );
    }
    if !config.unapplied_keys.is_empty() {
        eprintln!("Native proof applies page geometry, border colours, optional measured title/header/footer and footer page numbers; {} other config settings are not applied (including general text, custom fonts, header numbers/dates, images and runtime options).", config.unapplied_keys.len());
    }
    write_page_geometry_proof(&config.page_settings, &config.proof, &output)?;
    println!("Created {} using {}: {} diagnostic pages; configured borders; guide enabled: {}, width: {} mm, RGB: {:?}; no stamp rows. Vertical inter-element gap state: {} mm.", output.display(), config_path.display(), config.proof.page_count, config.proof.guide_enabled, config.proof.guide_width_mm, config.proof.guide_colour.rgb(), config.page_settings.spacing().vertical_mm());
    let first = config
        .page_settings
        .resolve_with_page_count(1, config.proof.page_count)?;
    if let Some(number) = first.footer_number {
        println!("Page 1 footer number: {:?}; measured height {:.4} mm, reserved height {:.4} mm; remaining content bottom {:.4} mm.", number.lines.iter().map(|line| line.text.as_str()).collect::<Vec<_>>(), number.measured_height_mm, number.reserved_height_mm, first.content_bounds.bottom);
    }
    if let Some(footer) = first.footer {
        println!("Page 1 footer: measured height {:.4} mm, reserved height {:.4} mm; reservation top {:.4} mm; remaining content bottom {:.4} mm.", footer.measured_height_mm, footer.reserved_height_mm, footer.reservation_top_mm, first.content_bounds.bottom);
    }
    if let Some(header) = first.header {
        println!("Page 1 header: measured height {:.4} mm, reserved height {:.4} mm; reservation bottom {:.4} mm; remaining content top {:.4} mm.", header.measured_height_mm, header.reserved_height_mm, header.reservation_bottom_mm, first.content_bounds.top);
    }
    if let Some(title) = first.title {
        println!("Page 1 title: measured height {:.4} mm, reserved height {:.4} mm; remaining content top {:.4} mm.", title.measured_height_mm, title.reserved_height_mm, first.content_bounds.top);
    }
    Ok(())
}
