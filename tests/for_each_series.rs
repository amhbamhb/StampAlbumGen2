use stamp_album_gen2::album_globals::generator_new;

#[test]
fn test_for_each_series_basic() {
    let conn = stamp_album_gen2::db_utils::init_db(":memory:").expect("open");

    conn.execute(
        "INSERT INTO stamps (country, series, year, sg_number, uniq_key) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["TestCountry", "S1", 1950, "1", "testcountry|S1|1"],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO stamps (country, series, year, sg_number, uniq_key) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["TestCountry", "S2", 1951, "1", "testcountry|S2|1"],
    )
    .unwrap();

    let mut seen: Vec<(usize, String)> = Vec::new();
    generator_new::for_each_series(&conn, None, |i, s| {
        seen.push((i, s.to_string()));
    })
    .expect("for_each_series");

    assert_eq!(seen.len(), 2);
    assert_eq!(seen[0].1, "S1");
    assert_eq!(seen[1].1, "S2");
    assert_eq!(seen[0].0, 0usize);
    assert_eq!(seen[1].0, 1usize);
}
