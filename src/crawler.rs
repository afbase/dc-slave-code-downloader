use anyhow::{Result, anyhow};
use reqwest::Client;
use scraper::{Html, Selector};

pub fn generate_urls(count: u32) -> Vec<String> {
    (1..=count)
        .map(|i| format!("https://www.loc.gov/resource/llsc.001/?sp={}&st=image", i))
        .collect()
}

pub async fn fetch_largest_jpeg_url(url: &str) -> Result<String> {
    let client = Client::new();
    let response = client.get(url).send().await?.text().await?;
    let document = Html::parse_document(&response);

    let form_selector = Selector::parse(".resource-download-form").unwrap();
    let option_selector = Selector::parse("option[value$='.jpg']").unwrap();

    let form = document.select(&form_selector).next()
        .ok_or_else(|| anyhow!("Form not found"))?;

    let largest_jpeg = form.select(&option_selector)
        .max_by_key(|option| {
            option.value().attr("value")
                .and_then(|url| url.split('/')
                    .last()
                    .and_then(|filename| filename.split('x').next())
                    .and_then(|width| width.parse::<u32>().ok())
                )
                .unwrap_or(0)
        })
        .ok_or_else(|| anyhow!("No JPEG option found"))?;

    largest_jpeg.value().attr("value")
        .ok_or_else(|| anyhow!("No value attribute found"))
        .map(|s| s.to_string())
}