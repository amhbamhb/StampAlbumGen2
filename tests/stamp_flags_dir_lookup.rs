use std::io::Write;

#[test]
fn stamp_flags_dir_lookup_found_coa_svg() {
    // Create a temp dir and put coats_of_arms/ca.svg inside it
    let dir = tempfile::tempdir().expect("tempdir");
    let base = dir.path();
    let coa_dir = base.join("coats_of_arms");
    std::fs::create_dir_all(&coa_dir).expect("create coa dir");

    let svg_path = coa_dir.join("ca.svg");
    let svg_content = r#"<?xml version='1.0' encoding='UTF-8'?>
<svg xmlns='http://www.w3.org/2000/svg' width='200' height='160'>
    <rect width='200' height='160' fill='#ffffff'/>
    <text x='100' y='85' font-size='24' text-anchor='middle' fill='#000000'>CA</text>
</svg>"#;
    let mut f = std::fs::File::create(&svg_path).expect("create svg");
    f.write_all(svg_content.as_bytes()).expect("write svg");

    // Point STAMP_FLAGS_DIR at the temp dir and call ensure_coat_of_arms
    std::env::set_var("STAMP_FLAGS_DIR", base);
    let res = stamp_album_gen2::flagdb::ensure_coat_of_arms("ca");
    // Cleanup env var
    std::env::remove_var("STAMP_FLAGS_DIR");

    assert!(res.is_ok(), "ensure_coat_of_arms should find local svg");
    let path = res.unwrap();
    assert!(
        std::path::Path::new(&path).exists(),
        "returned path should exist"
    );
    // Should be either svg or png
    let ext = std::path::Path::new(&path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    assert!(ext.eq_ignore_ascii_case("svg") || ext.eq_ignore_ascii_case("png"));
}
