use super::Result;
use chrono::{Datelike, NaiveDate};

#[derive(Clone, Copy, Debug)]
pub enum DateFormat {
    YearMonthDay,
    DayMonthYear,
    MonthDayYear,
}

impl DateFormat {
    pub fn parse(token: &str) -> Result<Self> {
        Self::parse_for(token, "native_pdf.header_date")
    }

    pub fn parse_for(token: &str, prefix: &str) -> Result<Self> {
        match token {
            "YMD" => Ok(Self::YearMonthDay),
            "DMY" => Ok(Self::DayMonthYear),
            "MDY" => Ok(Self::MonthDayYear),
            _ => Err(format!("{prefix}.format must be YMD, DMY or MDY")),
        }
    }

    pub fn format(self, date: NaiveDate, separator: &str) -> Result<String> {
        self.format_for(date, separator, "native_pdf.header_date")
    }

    fn format_for(self, date: NaiveDate, separator: &str, prefix: &str) -> Result<String> {
        if !(1..=9999).contains(&date.year()) {
            return Err(format!(
                "{prefix} requires a date within years 0001 through 9999"
            ));
        }
        let year = format!("{:04}", date.year());
        let month = format!("{:02}", date.month());
        let day = format!("{:02}", date.day());
        let fields = match self {
            Self::YearMonthDay => [year, month, day],
            Self::DayMonthYear => [day, month, year],
            Self::MonthDayYear => [month, day, year],
        };
        Ok(fields.join(separator))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DateSource {
    Generation,
    Fixed(NaiveDate),
}

impl DateSource {
    pub fn parse(source: &str, fixed_date: &str) -> Result<Self> {
        Self::parse_for(source, fixed_date, "native_pdf.header_date")
    }

    pub fn parse_for(source: &str, fixed_date: &str, prefix: &str) -> Result<Self> {
        // Validate the explicit override even when currently inactive.
        let fixed = NaiveDate::parse_from_str(fixed_date, "%Y-%m-%d")
            .map_err(|_| format!("{prefix}.fixed_date must be a valid YYYY-MM-DD calendar date"))?;
        if !(1..=9999).contains(&fixed.year()) || fixed.format("%Y-%m-%d").to_string() != fixed_date
        {
            return Err(format!(
                "{prefix}.fixed_date must be a valid YYYY-MM-DD date within years 0001 through 9999"
            ));
        }
        match source {
            "generation" => Ok(Self::Generation),
            "fixed" => Ok(Self::Fixed(fixed)),
            _ => Err(format!("{prefix}.date_source must be generation or fixed")),
        }
    }

    fn resolve(self, generation_date: Option<NaiveDate>, prefix: &str) -> Result<NaiveDate> {
        match self {
            Self::Generation => generation_date.ok_or_else(|| {
                format!("{prefix}.date_source=generation requires a date captured once for the document; use resolve_with_context")
            }),
            Self::Fixed(date) => Ok(date),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct DateText {
    format: DateFormat,
    separator: String,
    before: String,
    after: String,
    source: DateSource,
    prefix: &'static str,
}

impl DateText {
    pub(crate) fn new(
        format: DateFormat,
        separator: &str,
        before: &str,
        after: &str,
        source: DateSource,
        prefix: &'static str,
        validate: impl Fn(&str) -> Result<()>,
    ) -> Result<Self> {
        for (key, value) in [
            ("separator", separator),
            ("before", before),
            ("after", after),
        ] {
            validate(&format!("X{value}")).map_err(|error| format!("{prefix}.{key}: {error}"))?;
        }
        Ok(Self {
            format,
            separator: separator.into(),
            before: before.into(),
            after: after.into(),
            source,
            prefix,
        })
    }

    pub(crate) fn source(&self) -> DateSource {
        self.source
    }

    pub(crate) fn text(&self, generation_date: Option<NaiveDate>) -> Result<String> {
        let date = self.source.resolve(generation_date, self.prefix)?;
        let date = self.format.format_for(date, &self.separator, self.prefix)?;
        Ok(format!("{}{date}{}", self.before, self.after))
    }
}
