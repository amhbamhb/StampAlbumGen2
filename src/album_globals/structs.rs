#[derive(Debug, Clone, PartialEq)]
pub struct GenericAttrs {
    attrs: std::collections::HashMap<String, String>,
}

// Inline unit tests moved to integration tests under `tests/`.
impl Default for GenericAttrs {
    fn default() -> Self {
        Self::new()
    }
}

impl GenericAttrs {
    pub fn new() -> Self {
        Self {
            attrs: std::collections::HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.attrs.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.attrs.get(key)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stamp {
    pub attrs: GenericAttrs,
    pub image_path: String,
    pub image_width_px: Option<i32>,
    pub image_height_px: Option<i32>,
}
