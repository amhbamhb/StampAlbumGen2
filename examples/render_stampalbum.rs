use stamp_album_gen2::native_pdf::albumeasy;
use stamp_album_gen2::native_pdf::{album::NativeAlbum, config::NativePdfConfig};
use std::{env, fs, path::Path};

const USAGE: &str = "Usage: cargo run --example render_stampalbum -- <input.txt|input.stampalbum> <output.pdf> [config.toml]";

fn main() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let input = args.next().ok_or(USAGE)?;
    let output = args.next().ok_or(USAGE)?;
    let config_path = args.next();
    if args.next().is_some() {
        return Err(USAGE.into());
    }

    let input = Path::new(&input);
    let source = fs::read_to_string(input)
        .map_err(|error| format!("Cannot read {}: {error}", input.display()))?;
    let first = source
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('#'));
    let page_count = if first.is_some_and(|line| {
        line.split_whitespace()
            .next()
            .is_some_and(|name| name.eq_ignore_ascii_case("STAMPALBUM"))
    }) {
        let album = NativeAlbum::load(input)?;
        if let Some(config_path) = config_path {
            let config = NativePdfConfig::load(Path::new(&config_path))?;
            album
                .write_pdf_with_config(&config, Path::new(&output))?
                .len()
        } else {
            album.write_pdf(Path::new(&output))?.len()
        }
    } else {
        if config_path.is_some() {
            return Err("A TOML config may only be supplied for native STAMPALBUM input".into());
        }
        albumeasy::render_file(input, Path::new(&output))?
    };
    println!(
        "Created native PDF {} ({} pages)",
        Path::new(&output).display(),
        page_count
    );
    Ok(())
}
