use super::Result;

/// Opaque colour definition supporting named colours, hex forms (#RGB, #RRGGBB),
/// bare hex (RGB, RRGGBB), and rgb() functional syntax.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Color {
    rgb: [u8; 3],
}

pub type NamedColour = Color;

impl Color {
    pub const BLACK: Self = Self { rgb: [0, 0, 0] };
    pub const WHITE: Self = Self {
        rgb: [255, 255, 255],
    };

    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            rgb: [red, green, blue],
        }
    }

    /// Strict parser accepting only ASCII alphabetic SVG named colours.
    /// Preserves legacy AlbumEasy 4.5 validation behavior where hex and rgb are rejected.
    pub fn parse_strict_named(name: &str) -> Result<Self> {
        let name = name.trim();
        let invalid = || {
            "Expected an opaque SVG named colour; hex, RGB functions and transparency are not supported".to_string()
        };
        if name.is_empty() || !name.bytes().all(|byte| byte.is_ascii_alphabetic()) {
            return Err(invalid());
        }
        let lower = name.to_ascii_lowercase();
        if matches!(
            lower.as_str(),
            "none" | "transparent" | "currentcolor" | "inherit"
        ) {
            return Err(invalid());
        }
        let colour: svgtypes::Color = name.parse().map_err(|_| invalid())?;
        if colour.alpha != 255 {
            return Err(invalid());
        }
        Ok(Self {
            rgb: [colour.red, colour.green, colour.blue],
        })
    }

    /// Parse any supported colour representation:
    /// - SVG / CSS named colours: "black", "navy", "forestgreen", "darkred", etc. (case-insensitive)
    /// - 6-digit hex: "#1E4A7A", "#ff0000", or bare "1E4A7A"
    /// - 3-digit hex: "#F80", "#369", or bare "F80"
    /// - 8-digit or 4-digit hex: opaque alpha (255 / 15) allowed; non-opaque returns explicit error
    /// - rgb(r, g, b) functional syntax with 0-255 integers or 0%-100% percentages
    pub fn parse(input: &str) -> Result<Self> {
        let trimmed = input.trim().trim_matches('"').trim_matches('\'').trim();
        if trimmed.is_empty() {
            return Err("Colour string must not be empty".to_string());
        }

        let lower = trimmed.to_ascii_lowercase();

        // Reject non-colour CSS keywords with clear messages without echoing private values
        if lower == "none" {
            return Err("Colour 'none' is not a valid colour value".to_string());
        }
        if lower == "transparent" {
            return Err(
                "Transparency and alpha channels are not supported; colours must be fully opaque"
                    .to_string(),
            );
        }
        if matches!(lower.as_str(), "currentcolor" | "inherit") {
            return Err("Unsupported colour keyword".to_string());
        }

        // Handle bare hex (3 or 6 hex digits without '#')
        if (trimmed.len() == 3 || trimmed.len() == 6)
            && trimmed.chars().all(|c| c.is_ascii_hexdigit())
        {
            let prefixed = format!("#{trimmed}");
            return Self::parse_hex(&prefixed);
        }

        // Handle prefixed hex (#...)
        if trimmed.starts_with('#') {
            return Self::parse_hex(trimmed);
        }

        // Try svgtypes (handles named colours, rgb(), rgba())
        match trimmed.parse::<svgtypes::Color>() {
            Ok(colour) => {
                if colour.alpha != 255 {
                    return Err(
                        "Transparency and alpha channels are not supported; colours must be fully opaque"
                            .to_string(),
                    );
                }
                Ok(Self {
                    rgb: [colour.red, colour.green, colour.blue],
                })
            }
            Err(_) => {
                if lower.starts_with("rgb") {
                    Err("Invalid rgb() colour format; expected rgb(r, g, b)".to_string())
                } else if trimmed.starts_with('#') {
                    Err("Invalid hexadecimal colour; expected #RGB or #RRGGBB".to_string())
                } else {
                    Err(
                        "Expected a valid colour: SVG named colour, hex (#RGB or #RRGGBB), or rgb(r, g, b)"
                            .to_string(),
                    )
                }
            }
        }
    }

    fn parse_hex(hex_str: &str) -> Result<Self> {
        let hex = hex_str.strip_prefix('#').unwrap_or(hex_str);
        if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("Invalid hexadecimal colour: contains non-hex characters".to_string());
        }
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16).map_err(|_| "Invalid hex")? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16).map_err(|_| "Invalid hex")? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16).map_err(|_| "Invalid hex")? * 17;
                Ok(Self::from_rgb(r, g, b))
            }
            4 => {
                let a = u8::from_str_radix(&hex[3..4], 16).map_err(|_| "Invalid hex")? * 17;
                if a != 255 {
                    return Err(
                        "Transparency and alpha channels are not supported; colours must be fully opaque"
                            .to_string(),
                    );
                }
                let r = u8::from_str_radix(&hex[0..1], 16).map_err(|_| "Invalid hex")? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16).map_err(|_| "Invalid hex")? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16).map_err(|_| "Invalid hex")? * 17;
                Ok(Self::from_rgb(r, g, b))
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex")?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex")?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex")?;
                Ok(Self::from_rgb(r, g, b))
            }
            8 => {
                let a = u8::from_str_radix(&hex[6..8], 16).map_err(|_| "Invalid hex")?;
                if a != 255 {
                    return Err(
                        "Transparency and alpha channels are not supported; colours must be fully opaque"
                            .to_string(),
                    );
                }
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex")?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex")?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex")?;
                Ok(Self::from_rgb(r, g, b))
            }
            _ => Err(format!(
                "Invalid hexadecimal colour length ({}); expected 3 or 6 hex digits",
                hex.len()
            )),
        }
    }

    pub fn rgb(self) -> [f64; 3] {
        self.rgb.map(|component| f64::from(component) / 255.0)
    }

    pub fn rgb_u8(self) -> [u8; 3] {
        self.rgb
    }

    pub fn to_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.rgb[0], self.rgb[1], self.rgb[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_named_colours() {
        let black = Color::parse("black").unwrap();
        assert_eq!(black.rgb_u8(), [0, 0, 0]);
        assert_eq!(black.to_hex(), "#000000");

        let navy = Color::parse("NAVY").unwrap();
        assert_eq!(navy.rgb_u8(), [0, 0, 128]);

        let darkred = Color::parse("DarkRed").unwrap();
        assert_eq!(darkred.rgb_u8(), [139, 0, 0]);
    }

    #[test]
    fn parse_hex_colours() {
        let c1 = Color::parse("#1E4A7A").unwrap();
        assert_eq!(c1.rgb_u8(), [0x1E, 0x4A, 0x7A]);
        assert_eq!(c1.to_hex(), "#1E4A7A");

        let c2 = Color::parse("#1e4a7a").unwrap();
        assert_eq!(c2, c1);

        let c3 = Color::parse("#F80").unwrap();
        assert_eq!(c3.rgb_u8(), [0xFF, 0x88, 0x00]);
        assert_eq!(c3.to_hex(), "#FF8800");

        // Bare hex without '#'
        let c4 = Color::parse("1E4A7A").unwrap();
        assert_eq!(c4, c1);

        let c5 = Color::parse("F80").unwrap();
        assert_eq!(c5, c3);

        // Quoted hex
        let c6 = Color::parse("\"#1E4A7A\"").unwrap();
        assert_eq!(c6, c1);
    }

    #[test]
    fn parse_rgb_function() {
        let c1 = Color::parse("rgb(30, 74, 122)").unwrap();
        assert_eq!(c1.rgb_u8(), [30, 74, 122]);

        let c2 = Color::parse("rgb(100%, 0%, 0%)").unwrap();
        assert_eq!(c2.rgb_u8(), [255, 0, 0]);

        let c3 = Color::parse("\"rgb(0, 128, 255)\"").unwrap();
        assert_eq!(c3.rgb_u8(), [0, 128, 255]);
    }

    #[test]
    fn parse_rejections() {
        assert!(Color::parse("").is_err());
        assert!(Color::parse("none").is_err());
        assert!(Color::parse("transparent").is_err());
        assert!(Color::parse("#GG00FF").is_err());
        assert!(Color::parse("#12").is_err());
        assert!(Color::parse("#12345").is_err());
        assert!(Color::parse("rgba(0, 0, 0, 0.5)").is_err());
        assert!(Color::parse("#1E4A7A80").is_err());
    }

    #[test]
    fn strict_named_rejects_hex_and_rgb() {
        assert!(Color::parse_strict_named("black").is_ok());
        assert!(Color::parse_strict_named("navy").is_ok());
        assert!(Color::parse_strict_named("#1E4A7A").is_err());
        assert!(Color::parse_strict_named("#F80").is_err());
        assert!(Color::parse_strict_named("rgb(0,0,0)").is_err());
    }
}
