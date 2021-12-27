use scraper::{Html, Selector};

const BASE_URL: &str = "https://www.fishbase.de";

pub async fn get_url(english_name: &str) -> Result<Option<String>, reqwest::Error> {
    let client = reqwest::Client::new();
    let params = [
        ("crit1_fieldname", "COMNAMES.ComName"),
        ("lang", "English"),
        ("resultPage", "1"),
        ("crit1_fieldtype", "CHAR"),
        ("crit1_operator", "EQUAL"),
        ("CommonName", english_name),
    ];
    let res = client
        .post(format!(
            "{}{}",
            BASE_URL, "/ComNames/CommonNameSearchList.php"
        ))
        .form(&params)
        .send()
        .await?;
    let url_selector = Selector::parse(".notranslate > a").unwrap();
    let text = res.text().await?;
    let fragment = Html::parse_document(&text);

    let link = fragment
        .select(&url_selector)
        .find_map(|element| element.value().attr("href"))
        .map(|link| format!("{}{}{}", BASE_URL, "/ComNames/", link));

    if let Some(link) = link {
        let res = reqwest::Client::new().get(link).send().await?;
        let text = res.text().await?;
        let html = Html::parse_document(&text);
        let image_selector = Selector::parse(r#"a[style="text-decoration:none;"] > img"#).unwrap();

        let image_link = html
            .select(&image_selector)
            .find_map(|element| element.value().attr("src"))
            .map(|link| format!("{}{}", BASE_URL, link));
        Ok(image_link)
    } else {
        Ok(None)
    }
}
