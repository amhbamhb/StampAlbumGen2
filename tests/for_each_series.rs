use rusqlite::Connection;
use stamp_album_gen2::album_globals::generator_new;

#[test]
fn test_for_each_series_basic() {
    let conn = Connection::open_in_memory().expect("open");
    conn.execute_batch(
        "CREATE TABLE stamps (
            id INTEGER PRIMARY KEY,
            country TEXT,
            series TEXT,
            year INTEGER,
            month TEXT,
            month_num INTEGER,
            day TEXT,
            sg_number TEXT
        )",
    )
    .unwrap();

    conn.execute(
        "INSERT INTO stamps (series, year, sg_number) VALUES (?1, ?2, ?3)",
        rusqlite::params!["S1", 1950, "1"],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO stamps (series, year, sg_number) VALUES (?1, ?2, ?3)",
        rusqlite::params!["S2", 1951, "1"],
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
