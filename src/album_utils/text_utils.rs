use regex::Regex;

/// Cleans problematic Unicode and formatting from text for AlbumEasy output
pub fn clean_text(s: &str) -> String {
    // Remove 'Â' or 'â' before fractions/currency (UTF-8 encoding artifacts)
    let re = Regex::new(r"[Ââ]([½¼¾⅓⅔⅛⅜⅝⅞£€$¥])").unwrap();
    let mut s = re.replace_all(s, "$1").to_string();

    // Replace stray 'Â' or 'â' with nothing
    s = s.replace('â', "");
    s = s.replace('Â', "");

    // REVERSE LOGIC: Convert Unicode fractions BACK to ASCII for AlbumEasy compatibility
    // First, handle fractions that need spacing (e.g., 11⅝ → 11 5/8)
    let fraction_chars = ["½", "¼", "¾", "⅓", "⅔", "⅛", "⅜", "⅝", "⅞"];
    let fraction_replacements = [
        "1/2", "1/4", "3/4", "1/3", "2/3", "1/8", "3/8", "5/8", "7/8",
    ];

    for (fraction_char, replacement) in fraction_chars.iter().zip(fraction_replacements.iter()) {
        // Use regex to add space between number and fraction (e.g., 11⅝ → 11 5/8)
        let pattern = format!(r"(\d)({})", fraction_char);
        let re = Regex::new(&pattern).unwrap();
        s = re
            .replace_all(&s, format!("$1 {}", replacement))
            .to_string();
    }

    // Handle standalone fractions (not preceded by numbers)
    s = s.replace("½", "1/2");
    s = s.replace("¼", "1/4");
    s = s.replace("¾", "3/4");
    s = s.replace("⅓", "1/3");
    s = s.replace("⅔", "2/3");
    s = s.replace("⅛", "1/8");
    s = s.replace("⅜", "3/8");
    s = s.replace("⅝", "5/8");
    s = s.replace("⅞", "7/8");

    // Replace double quotes
    s = s.replace('"', "'");

    // Replace newlines and tabs with spaces
    s = s.replace('\n', " ");
    s = s.replace('\r', " ");
    s = s.replace('\t', " ");

    // Replace multiple spaces with single space
    let re_spaces = Regex::new(r"\s+").unwrap();
    s = re_spaces.replace_all(&s, " ").to_string();

    // Trim whitespace
    s = s.trim().to_string();

    // Replace non-ASCII except allowed symbols with space
    s.chars()
        .map(|c| match c {
            '£' | '€' | '$' | '¥' | '½' | '¼' | '¾' | '⅓' | '⅔' | '⅛' | '⅜' | '⅝' | '⅞' => {
                c
            }
            _ if c.is_ascii() => c,
            _ => ' ',
        })
        .collect()
}

/// Sanitizes a filename by replacing filesystem-incompatible characters
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| {
            match c {
                '*' => 's',  // star -> s
                '?' => 'q',  // question -> q
                '<' => 'l',  // less -> l
                '>' => 'g',  // greater -> g
                ':' => 'c',  // colon -> c
                '"' => 'd',  // double quote -> d
                '|' => 'p',  // pipe -> p
                '\\' => 'b', // backslash -> b
                '/' => 'f',  // forward slash -> f
                _ if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' => c,
                _ => '_', // replace other non-alphanumeric with underscore
            }
        })
        .collect()
}
