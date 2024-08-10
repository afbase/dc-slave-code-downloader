use anyhow::Result;
use reqwest::Client;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use indicatif::ProgressBar;
use futures_util::StreamExt;

pub async fn download_image(jpeg_url: &str, source_url: &str, progress_bar: ProgressBar) -> Result<()> {
    let client = Client::new();
    let response = client.get(jpeg_url).send().await?;
    let total_size = response.content_length().unwrap_or(0);
    progress_bar.set_length(total_size);

    let page_number = source_url.split("sp=").nth(1)
        .and_then(|s| s.split('&').next())
        .unwrap_or("unknown");

    let filename = format!("{}.jpg", page_number);
    let path = Path::new(&filename);

    let mut file = File::create(path).await?;
    let mut downloaded = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        progress_bar.set_position(downloaded);
    }

    progress_bar.finish_with_message(format!("Downloaded {}", filename));
    Ok(())
}