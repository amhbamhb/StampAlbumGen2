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
        match token {
            "YMD" => Ok(Self::YearMonthDay),
            "DMY" => Ok(Self::DayMonthYear),
            "MDY" => Ok(Self::MonthDayYear),
            _ => Err("native_pdf.header_date.format must be YMD, DMY or MDY".into()),
        }
    }

    pub fn format(self, date: NaiveDate, separator: &str) -> Result<String> {
        if !(1..=9999).contains(&date.year()) {
            return Err(
                "native_pdf.header_date requires a date within years 0001 through 9999".into(),
            );
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
        // Validate the explicit override even when currently inactive.
        let fixed = NaiveDate::parse_from_str(fixed_date, "%Y-%m-%d").map_err(|_| {
            "native_pdf.header_date.fixed_date must be a valid YYYY-MM-DD calendar date"
        })?;
        if !(1..=9999).contains(&fixed.year()) || fixed.format("%Y-%m-%d").to_string() != fixed_date
        {
            return Err("native_pdf.header_date.fixed_date must be a valid YYYY-MM-DD date within years 0001 through 9999".into());
        }
        match source {
            "generation" => Ok(Self::Generation),
            "fixed" => Ok(Self::Fixed(fixed)),
            _ => Err("native_pdf.header_date.date_source must be generation or fixed".into()),
        }
    }

    pub(crate) fn resolve(self, generation_date: Option<NaiveDate>) -> Result<NaiveDate> {
        match self {
            Self::Generation => generation_date.ok_or_else(|| {
                "native_pdf.header_date.date_source=generation requires a date captured once for the document; use resolve_with_context".into()
            }),
            Self::Fixed(date) => Ok(date),
        }
    }
}
