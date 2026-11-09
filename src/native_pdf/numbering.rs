use super::Result;

#[derive(Clone, Copy, Debug)]
pub(crate) struct NumberingContext {
    pub section_ordinal: usize,
    pub section_pages: usize,
    pub document_pages: usize,
    pub start: Option<i32>,
}

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
        let context = NumberingContext {
            section_ordinal: page,
            section_pages: total,
            document_pages: total,
            start: None,
        };
        let number = self.display_number(context, command, start_key)?;
        Ok(format!("{}{number}{}", self.before, self.after).replace("$PAGES$", &total.to_string()))
    }

    pub(crate) fn text_with_context(
        &self,
        context: NumberingContext,
        command: &str,
        start_key: &str,
    ) -> Result<String> {
        let number = self.display_number(context, command, start_key)?;
        Ok(format!("{}{number}{}", self.before, self.after)
            .replace("$PAGES$", &context.document_pages.to_string())
            .replace("$DOCUMENT_PAGES$", &context.document_pages.to_string())
            .replace("$SECTION_PAGE$", &context.section_ordinal.to_string())
            .replace("$SECTION_PAGES$", &context.section_pages.to_string()))
    }

    fn display_number(
        &self,
        context: NumberingContext,
        command: &str,
        start_key: &str,
    ) -> Result<i32> {
        if context.document_pages == 0
            || u32::try_from(context.document_pages).is_err()
            || context.section_pages == 0
            || u32::try_from(context.section_pages).is_err()
            || context.section_ordinal == 0
            || context.section_ordinal > context.section_pages
        {
            return Err(format!(
                "{command} requires positive document and section page counts and a section ordinal within its section"
            ));
        }
        let offset = i64::try_from(context.section_ordinal - 1)
            .map_err(|_| format!("{start_key} overflows printed numbering"))?;
        let number = i64::from(context.start.unwrap_or(self.start))
            .checked_add(offset)
            .and_then(|n| i32::try_from(n).ok())
            .ok_or_else(|| format!("{start_key} overflows signed 32-bit printed numbering"))?;
        Ok(number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_section_pages_starting_at_ten_end_at_nineteen() -> Result<()> {
        let numbering = PageNumbering::new(
            1,
            "Page ",
            " of $PAGES$; section $SECTION_PAGE$/$SECTION_PAGES$; document $DOCUMENT_PAGES$",
        );
        for ordinal in 1..=10 {
            let label = numbering.text_with_context(
                NumberingContext {
                    section_ordinal: ordinal,
                    section_pages: 10,
                    document_pages: 27,
                    start: Some(10),
                },
                "ALBUM_PAGES_FOOTER_NUMBER",
                "album_pages_footer_number_start",
            )?;
            assert_eq!(
                label,
                format!(
                    "Page {} of 27; section {ordinal}/10; document 27",
                    9 + ordinal
                )
            );
        }
        Ok(())
    }
}
