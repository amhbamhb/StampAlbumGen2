use super::Result;

/// Shared checked labels; caller supplies component-specific diagnostic keys.
#[derive(Clone, Debug)]
pub(crate) struct PageNumbering {
    start: i32,
    before: String,
    after: String,
}

impl PageNumbering {
    pub(crate) fn new(start: i32, before: &str, after: &str) -> Self {
        Self {
            start,
            before: before.into(),
            after: after.into(),
        }
    }

    pub(crate) fn text(
        &self,
        page: usize,
        total: usize,
        command: &str,
        start_key: &str,
    ) -> Result<String> {
        if total == 0 || u32::try_from(total).is_err() || page == 0 || page > total {
            return Err(format!(
                "{command} requires a positive final PDF page count and a physical page within it"
            ));
        }
        let offset = i64::try_from(page - 1)
            .map_err(|_| format!("{start_key} overflows printed numbering"))?;
        let number = i64::from(self.start)
            .checked_add(offset)
            .and_then(|n| i32::try_from(n).ok())
            .ok_or_else(|| format!("{start_key} overflows signed 32-bit printed numbering"))?;
        Ok(format!("{}{number}{}", self.before, self.after).replace("$PAGES$", &total.to_string()))
    }
}
