use crate::native_pdf::{
    Result,
    colour::{Color, NamedColour},
};

/// COLOUR_ALBUM_BORDER and its COLOR_ALBUM_BORDER spelling alias.
/// AlbumEasy applies the final setting to every page, including earlier pages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlbumBorderColour(Color);

impl AlbumBorderColour {
    /// Parse colour argument supporting named colours, hex forms (#RGB, #RRGGBB, bare hex), and rgb().
    pub fn parse(arguments: &[&str]) -> Result<Self> {
        let [name] = arguments else {
            return Err("COLOUR_ALBUM_BORDER requires exactly one colour argument".into());
        };
        Color::parse(name)
            .map(Self)
            .map_err(|error| format!("COLOUR_ALBUM_BORDER: {error}"))
    }

    /// Strict parser accepting only ASCII alphabetic SVG named colours for AlbumEasy 4.5 baseline tests.
    pub fn parse_strict(arguments: &[&str]) -> Result<Self> {
        let [name] = arguments else {
            return Err("COLOUR_ALBUM_BORDER requires exactly one named colour".into());
        };
        Color::parse_strict_named(name)
            .map(Self)
            .map_err(|error| format!("COLOUR_ALBUM_BORDER: {error}"))
    }

    pub fn rgb(self) -> [f64; 3] {
        self.0.rgb()
    }

    pub fn color(self) -> Color {
        self.0
    }

    pub fn named(self) -> NamedColour {
        self.0
    }
}
