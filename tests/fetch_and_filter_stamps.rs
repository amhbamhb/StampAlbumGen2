use stamp_album_gen2::album_globals::generator_new::helpers;

#[test]
fn test_fetch_and_filter_stamps_basic() {
    let conn = stamp_album_gen2::db_utils::init_db(":memory:").expect("open");

    conn.execute(
        "INSERT INTO stamps (country, series, year, month_num, sg_number, uniq_key) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params!["TestCountry", "S1", 1900, 0, "1", "testcountry|S1|1"],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO stamps (country, series, year, month_num, sg_number, uniq_key) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params!["TestCountry", "S1", 1910, 0, "2", "testcountry|S1|2"],
    )
    .unwrap();

    // No filtering
    let stamps = helpers::fetch_and_filter_stamps(&conn, "S1", None, None).expect("fetch");
    assert_eq!(stamps.len(), 2);

    // Filter years >= 1905
    let stamps2 = helpers::fetch_and_filter_stamps(&conn, "S1", Some(1905), None).expect("fetch2");
    assert_eq!(stamps2.len(), 1);
    assert_eq!(stamps2[0].attrs.get("year").unwrap(), "1910");
}
