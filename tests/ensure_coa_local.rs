use stamp_album_gen2::flagdb;

#[test]
fn ensure_coa_nl_local() {
    match flagdb::ensure_coat_of_arms("nl") {
        Ok(path) => println!("OK: coat path={}", path),
        Err(e) => panic!("ensure_coat_of_arms failed: {}", e),
    }
}
