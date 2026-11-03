use crate::native_pdf::{colour::NamedColour, Result};

/// COLOUR_ALBUM_BORDER and its COLOR_ALBUM_BORDER spelling alias.
/// AlbumEasy applies the final setting to every page, including earlier pages.
#[derive(Clone, Copy, Debug, Default)]
pub struct AlbumBorderColour(NamedColour);

impl AlbumBorderColour {
    pub fn parse(arguments: &[&str]) -> Result<Self> {
        let [name] = arguments else {
            return Err("COLOUR_ALBUM_BORDER requires exactly one named colour".into());
        };
        NamedColour::parse(name)
            .map(Self)
            .map_err(|error| format!("COLOUR_ALBUM_BORDER: {error}"))
    }

    pub fn rgb(self) -> [f64; 3] {
        self.0.rgb()
    }
}
