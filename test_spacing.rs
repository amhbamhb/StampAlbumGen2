use regex::Regex;

fn main() {
    let test_cases = vec![
        "Perf: 11½ x 12, 1873",
        "Perf: 11¼ x 12, 1873", 
        "Perf: 11¾ x 12, 1873",
        "Perf: 11⅓ x 12, 1873",
        "Perf: 11⅔ x 12, 1873",
        "Perf: 11⅛ x 12, 1873",
        "Perf: 11⅜ x 12, 1873",
        "Perf: 11⅝ x 12, 1873",
        "Perf: 11⅞ x 12, 1873",
        "Perf: 12½ x 13, 1874",  // Test with different numbers
        "Size: 10¾ x 11",        // Test standalone fractions
        "Perforation: 11⅝"       // Test without other text
    ];
    
    for input in test_cases {
        let mut s = input.to_string();
        
        // Apply the same logic as the updated clean_text function
        let fraction_chars = ["½", "¼", "¾", "⅓", "⅔", "⅛", "⅜", "⅝", "⅞"];
        let fraction_replacements = ["1/2", "1/4", "3/4", "1/3", "2/3", "1/8", "3/8", "5/8", "7/8"];

        for (fraction_char, replacement) in fraction_chars.iter().zip(fraction_replacements.iter()) {
            // Use regex to add space between number and fraction (e.g., 11⅝ → 11 5/8)
            let pattern = format!(r"(\d)({})", fraction_char);
            let re = Regex::new(&pattern).unwrap();
            s = re.replace_all(&s, format!("$1 {}", replacement)).to_string();
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
        
        println!("Input:  {}", input);
        println!("Output: {}", s);
        println!("---");
    }
}
