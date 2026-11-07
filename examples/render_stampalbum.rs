use stamp_album_gen2::native_pdf::album::NativeAlbum;
use stamp_album_gen2::native_pdf::albumeasy;
use std::{env, fs, path::Path};

const USAGE: &str =
    "Usage: cargo run --example render_stampalbum -- <input.txt|input.stampalbum> <output.pdf>";

fn main() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let input = args.next().ok_or(USAGE)?;
    let output = args.next().ok_or(USAGE)?;
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
    let page_count = if first.is_some_and(|line| line.starts_with("STAMPALBUM")) {
        let album = NativeAlbum::load(input)?;
        album.write_pdf(Path::new(&output))?.len()
    } else {
        albumeasy::render_file(input, Path::new(&output))?
    };
    println!(
        "Created native PDF {} ({} pages)",
        Path::new(&output).display(),
        page_count
    );
    Ok(())
}
