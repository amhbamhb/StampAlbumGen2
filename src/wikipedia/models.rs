/// Source of the content
#[derive(Debug, Clone)]
pub enum ContentSource {
    Grokipedia,
    Wikipedia,
}

impl ContentSource {
    pub fn as_str(&self) -> &str {
        match self {
            ContentSource::Grokipedia => "grokipedia",
            ContentSource::Wikipedia => "wikipedia",
        }
    }
}
