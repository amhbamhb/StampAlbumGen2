use stamp_album_gen2::flagdb;

#[tokio::test]
async fn ensure_coa_nl_local() {
    match flagdb::ensure_coat_of_arms("nl").await {
        Ok(path) => println!("OK: coat path={}", path),
        Err(e) => panic!("ensure_coat_of_arms failed: {}", e),
    }
}
