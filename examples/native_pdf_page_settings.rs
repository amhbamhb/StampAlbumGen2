use stamp_album_gen2::native_pdf::{
    commands::page::{
        album_pages_margins::Margins, album_pages_marginse::EvenMargins,
        album_pages_size::PageSize, album_pages_spacing::Spacing,
    },
    document::PageSettings,
    renderer::write_page_geometry_proof,
};
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = std::env::args_os().skip(1);
    let output = PathBuf::from(
        arguments
            .next()
            .ok_or("Usage: cargo run --example native_pdf_page_settings -- <new-output.pdf>")?,
    );
    if arguments.next().is_some() {
        return Err("Expected one output PDF path".into());
    }
    if output.exists() {
        return Err(format!("Refusing to overwrite {}", output.display()).into());
    }
    let mut settings = PageSettings::new(
        PageSize::parse(&[210.0, 297.0])?,
        Margins::parse(&[25.0, 12.0, 18.0, 21.0])?,
        Spacing::parse(&[6.25, 8.75])?,
    );
    settings.set_even_margins(EvenMargins::parse(&[12.0, 25.0, 22.0, 16.0])?);
    write_page_geometry_proof(&settings, 3, &output)?;
    println!("Created {}: three geometry-only pages; no text/rows. Outer rectangle: margin border; inner: content guide. Vertical inter-element gap state: {} mm.", output.display(), settings.spacing().vertical_mm());
    Ok(())
}
