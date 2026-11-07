use stamp_album_gen2::native_pdf::album::NativeAlbum;
use std::{env, path::Path};

fn main() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let input = args
        .next()
        .ok_or("Usage: cargo run --example render_stampalbum -- <input.stampalbum> <output.pdf>")?;
    let output = args
        .next()
        .ok_or("Usage: cargo run --example render_stampalbum -- <input.stampalbum> <output.pdf>")?;
    if args.next().is_some() {
        return Err(
            "Usage: cargo run --example render_stampalbum -- <input.stampalbum> <output.pdf>"
                .into(),
        );
    }

    let album = NativeAlbum::load(Path::new(&input))?;
    let pages = album.write_pdf(Path::new(&output))?;
    println!(
        "Created native PDF {} ({} pages)",
        Path::new(&output).display(),
        pages.len()
    );
    Ok(())
}
