use scraper::{Html, Selector};

#[test]
fn description_extraction_prefers_td6_with_fallback() {
    // Sample row based on observed StampWorld structure where description is in td[6]
    let html = r#"
    <table>
      <tr class="stamp_tr">
        <th>1063</th>
        <td>Type</td>
        <td>Denomination</td>
        <td>Color</td>
                <td></td>
                <td>100</td>
                <td></td>
                <td>The 100th Anniversary\n of Forestry Legislation</td>
        <td>Mint</td>
      </tr>
    </table>
    "#;

    let document = Html::parse_document(html);
    let tr_selector = Selector::parse("tr.stamp_tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    let mut found = false;
    for tr in document.select(&tr_selector) {
        let tds: Vec<_> = tr.select(&td_selector).collect();
        // Extraction logic: prefer tds[6] when available, otherwise fallback to tds[4]
        let description = if tds.len() > 6 {
            tds[6].text().collect::<String>().trim().to_string()
        } else if tds.len() > 4 {
            tds[4].text().collect::<String>().trim().to_string()
        } else {
            String::new()
        };
        assert!(description.contains("100th Anniversary"), "description did not contain expected text, got: '{}'", description);
        found = true;
    }
    assert!(found, "no stamp_tr row found in test HTML");
}
