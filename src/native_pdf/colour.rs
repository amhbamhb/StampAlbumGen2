use super::Result;

/// Opaque SVG named colour, shared by native colour commands.
#[derive(Clone, Copy, Debug, Default)]
pub struct NamedColour {
    rgb: [u8; 3],
}

impl NamedColour {
    pub fn parse(name: &str) -> Result<Self> {
        let name = name.trim();
        let invalid = || {
            "Expected an opaque SVG named colour; hex, RGB functions and transparency are not supported".to_string()
        };
        if name.is_empty() || !name.bytes().all(|byte| byte.is_ascii_alphabetic()) {
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

    pub fn rgb(self) -> [f64; 3] {
        self.rgb.map(|component| f64::from(component) / 255.0)
    }
}
