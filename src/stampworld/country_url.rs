//! Persist explicit country URLs without changing display names or album identity.

use std::{collections::HashMap, io::Write, path::Path, sync::Mutex};

const MAPPINGS_FILE: &str = "slug_overrides.toml";
static SAVE_LOCK: Mutex<()> = Mutex::new(());

/// Build a country page URL, preserving punctuation and replacing whitespace with hyphens.
pub fn country_page_url(slug: &str) -> String {
    let slug = slug.split_whitespace().collect::<Vec<_>>().join("-");
    format!(
        "https://www.stampworld.com/en/stamps/{}/",
        urlencoding::encode(&slug)
    )
}

/// Build the listing URL for automatic or explicitly saved country slugs.
pub fn listing_url(
    country: &str,
    start_year: i32,
    end_year: i32,
    overrides: &HashMap<String, String>,
) -> String {
    let slug = overrides
        .get(country)
        .map(String::as_str)
        .unwrap_or(country);
    let page = country_page_url(slug);
    if start_year == 0 && end_year == 9999 {
        format!("{page}?page=")
    } else {
        format!("{page}Postage%20stamps/{start_year}-{end_year}?page=")
    }
}

/// Validate a full country URL and build its listing URL for a single pull.
pub fn listing_url_from_country_url(
    value: &str,
    start_year: i32,
    end_year: i32,
) -> Result<String, String> {
    let slug = slug_from_url(value)?;
    Ok(listing_url(&slug, start_year, end_year, &HashMap::new()))
}

fn slug_from_url(value: &str) -> Result<String, String> {
    let url = reqwest::Url::parse(value.trim())
        .map_err(|_| "Enter a full StampWorld country URL, for example https://www.stampworld.com/en/stamps/South-Korea/.".to_string())?;
    if url.scheme() != "https"
        || url.host_str() != Some("www.stampworld.com")
        || url.port().is_some()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err("Use an https://www.stampworld.com/en/stamps/<country>/ URL without credentials, query parameters or a fragment.".into());
    }
    let segments: Vec<_> = url.path().trim_end_matches('/').split('/').collect();
    if segments.len() != 4
        || segments[1] != "en"
        || segments[2] != "stamps"
        || segments[3].is_empty()
    {
        return Err("Paste the country page URL, not a stamp, year-range or other page.".into());
    }
    let slug = urlencoding::decode(segments[3])
        .map_err(|error| format!("Invalid URL encoding: {error}"))?
        .into_owned();
    if slug.len() > 512
        || slug
            .chars()
            .any(|c| c.is_control() || matches!(c, '/' | '\\'))
        || slug == "."
        || slug == ".."
    {
        return Err("Invalid StampWorld country path.".into());
    }
    Ok(slug)
}

fn validate_country(country: &str) -> Result<&str, String> {
    let country = country.trim();
    if country.is_empty() || country.len() > 512 || country.chars().any(char::is_control) {
        return Err("A valid country name is required.".into());
    }
    Ok(country)
}

/// Read the effective URL and whether a custom country mapping is saved.
pub fn load_country_url(country: &str) -> Result<(String, bool), String> {
    let country = validate_country(country)?;
    let mappings = crate::config::try_load_slug_overrides(MAPPINGS_FILE)?;
    let custom = mappings.get(country);
    Ok((
        country_page_url(custom.map(String::as_str).unwrap_or(country)),
        custom.is_some(),
    ))
}

/// Save a country mapping, or remove it when the input is blank.
pub fn save_country_url(country: &str, value: &str) -> Result<(String, bool), String> {
    let country = validate_country(country)?;
    let slug = if value.trim().is_empty() {
        None
    } else {
        Some(slug_from_url(value)?)
    };
    let _guard = SAVE_LOCK
        .lock()
        .map_err(|_| "Country mapping save lock failed".to_string())?;
    let path = crate::config::slug_override_path(MAPPINGS_FILE)?;
    save_mapping_at(&path, country, slug.as_deref())?;
    Ok((
        country_page_url(slug.as_deref().unwrap_or(country)),
        slug.is_some(),
    ))
}

fn save_mapping_at(path: &Path, country: &str, slug: Option<&str>) -> Result<(), String> {
    let mut document: toml::Value = match std::fs::read_to_string(path) {
        Ok(content) => toml::from_str(&content).map_err(|error| {
            format!(
                "Invalid {}: {error}; mappings were not changed.",
                path.display()
            )
        })?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            toml::Value::Table(Default::default())
        }
        Err(error) => return Err(format!("Cannot read {}: {error}", path.display())),
    };
    let table = document
        .as_table_mut()
        .ok_or_else(|| "Country mappings must be a TOML document".to_string())?;
    let mappings = table
        .entry("slug_overrides".to_string())
        .or_insert_with(|| toml::Value::Table(Default::default()));
    let mappings = mappings
        .as_table_mut()
        .ok_or_else(|| "slug_overrides must be a TOML table".to_string())?;
    if let Some(slug) = slug {
        mappings.insert(country.to_string(), toml::Value::String(slug.to_string()));
    } else {
        mappings.remove(country);
    }
    let content = toml::to_string_pretty(&document)
        .map_err(|error| format!("Cannot serialize country mappings: {error}"))?;
    let parent = path
        .parent()
        .filter(|dir| !dir.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut temporary = tempfile::NamedTempFile::new_in(parent)
        .map_err(|error| format!("Cannot prepare country mappings: {error}"))?;
    temporary
        .write_all(content.as_bytes())
        .map_err(|error| format!("Cannot write country mappings: {error}"))?;
    temporary
        .as_file()
        .sync_all()
        .map_err(|error| format!("Cannot flush country mappings: {error}"))?;
    temporary
        .persist(path)
        .map_err(|error| format!("Cannot save {}: {error}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiword_names_and_explicit_mappings_build_the_expected_urls() {
        for (country, slug) in [
            ("South Korea", "South-Korea"),
            (
                "North Korean Occ. South Korea",
                "North-Korean-Occ.-South-Korea",
            ),
            ("British Canada", "British-Canada"),
        ] {
            let expected = format!("https://www.stampworld.com/en/stamps/{slug}/");
            assert_eq!(country_page_url(country), expected);
            assert_eq!(slug_from_url(&expected).expect("valid URL"), slug);
            assert_eq!(
                listing_url(country, 0, 9999, &HashMap::new()),
                format!("{expected}?page=")
            );
            assert_eq!(
                listing_url(country, 1840, 2026, &HashMap::new()),
                format!("{expected}Postage%20stamps/1840-2026?page=")
            );
            assert_eq!(
                listing_url_from_country_url(&expected, 1840, 2026).expect("explicit URL"),
                format!("{expected}Postage%20stamps/1840-2026?page=")
            );
        }
        let mappings = HashMap::from([("Local name".into(), "British-Canada".into())]);
        assert_eq!(
            listing_url("Local name", 0, 9999, &mappings),
            "https://www.stampworld.com/en/stamps/British-Canada/?page="
        );
        assert_eq!(
            country_page_url(" South   Korea "),
            country_page_url("South Korea")
        );
    }

    #[test]
    fn invalid_urls_cannot_be_saved_as_country_paths() {
        for url in [
            "https://evil.example/en/stamps/Korea/",
            "http://www.stampworld.com/en/stamps/Korea/",
            "https://www.stampworld.com/en/stamps/Korea/?page=2",
            "https://www.stampworld.com/en/stamps/Korea/Postage%20stamps/1970-1980",
            "https://www.stampworld.com/en/stamps/Korea%2Fother/",
            "https://user@www.stampworld.com/en/stamps/Korea/",
            "https://www.stampworld.com:444/en/stamps/Korea/",
            "https://www.stampworld.com/en/stamps/Korea/#stamp",
            "https://www.stampworld.com/en/stamps/Korea%5Cother/",
            "https://www.stampworld.com/en/stamps/Korea%00/",
        ] {
            assert!(slug_from_url(url).is_err(), "{url}");
        }
    }

    #[test]
    fn saved_mappings_survive_reload_and_preserve_other_settings() {
        let temp = tempfile::tempdir().expect("test directory");
        let path = temp.path().join(MAPPINGS_FILE);
        std::fs::write(
            &path,
            "[other]\nsetting = 42\n[slug_overrides]\nSwitzerland = 'Switzerland'\n",
        )
        .expect("existing settings");
        save_mapping_at(&path, "South Korea", Some("South-Korea")).expect("save");
        let mappings = crate::config::try_load_slug_overrides(path.to_str().expect("test path"))
            .expect("reload");
        assert_eq!(mappings["South Korea"], "South-Korea");
        assert_eq!(mappings["Switzerland"], "Switzerland");
        save_mapping_at(&path, "South Korea", None).expect("reset");
        let text = std::fs::read_to_string(&path).expect("saved file");
        assert!(text.contains("setting = 42"));
        assert!(!text.contains("South Korea"));
        std::fs::write(&path, "invalid = [").expect("invalid file");
        assert!(save_mapping_at(&path, "South Korea", Some("South-Korea")).is_err());
        assert_eq!(
            std::fs::read_to_string(path).expect("unchanged"),
            "invalid = ["
        );
    }

    #[test]
    fn first_save_creates_a_mapping_file_and_invalid_table_is_not_overwritten() {
        let temp = tempfile::tempdir().expect("test directory");
        let path = temp.path().join(MAPPINGS_FILE);
        save_mapping_at(&path, "Local display name", Some("British-Canada")).expect("first save");
        let mappings = crate::config::try_load_slug_overrides(path.to_str().expect("test path"))
            .expect("reload");
        assert_eq!(
            listing_url("Local display name", 0, 9999, &mappings),
            "https://www.stampworld.com/en/stamps/British-Canada/?page="
        );
        std::fs::write(&path, "slug_overrides = 42").expect("invalid table");
        assert!(save_mapping_at(&path, "South Korea", Some("South-Korea")).is_err());
        assert_eq!(
            std::fs::read_to_string(path).expect("unchanged"),
            "slug_overrides = 42"
        );
    }
}
