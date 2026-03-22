use scraper::{Html, Selector};

pub fn extract_access_date(html: &str) -> Option<Vec<String>> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse("td.cell.c0").ok()?;
    let dates = doc
        .select(&selector)
        .map(|x| x.text().collect::<String>())
        .collect::<Vec<_>>();

    Some(dates)
}

pub fn extract_last_access(html: &str) -> Option<Vec<String>> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse("td.cell.c1").ok()?;
    let access = doc
        .select(&selector)
        .map(|x| x.text().collect::<String>())
        .collect::<Vec<String>>();

    Some(access)
}

pub fn extract_ip(html: &str) -> Option<Vec<String>> {
    let doc = Html::parse_fragment(html);
    let selector = Selector::parse("td.cell.c2").ok()?;
    let ips = doc
        .select(&selector)
        .map(|x| x.text().collect::<String>())
        .collect::<Vec<String>>();

    Some(ips)
}
