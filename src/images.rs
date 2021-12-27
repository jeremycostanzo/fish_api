use scraper::{Html, Selector};

const BASE_URL: &str = "https://www.fishbase.de";

pub async fn get_url(english_name: &str) -> Result<Option<String>, reqwest::Error> {
    let link = get_page_containing_image_from_name(english_name).await?;

    if let Some(link) = link {
        extract_from_page(link).await
    } else {
        Ok(None)
    }
}

async fn get_page_containing_image_from_name(
    english_name: &str,
) -> Result<Option<String>, reqwest::Error> {
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

    Ok(fragment
        .select(&url_selector)
        .find_map(|element| element.value().attr("href"))
        .map(|link| BASE_URL.to_owned() + "/ComNames/" + link))
}

async fn extract_from_page(url: String) -> Result<Option<String>, reqwest::Error> {
    let res = reqwest::Client::new().get(url).send().await?;
    let text = res.text().await?;
    let html = Html::parse_document(&text);
    let image_selector = Selector::parse(r#"a[style="text-decoration:none;"] > img"#).unwrap();

    let image_link = html
        .select(&image_selector)
        .find_map(|element| element.value().attr("src"))
        .map(|link| BASE_URL.to_owned() + link);
    Ok(image_link)
}
