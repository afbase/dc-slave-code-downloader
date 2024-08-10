use anyhow::Result;
mod crawler;
mod downloader;
mod rate_limiter;
use rate_limiter::RateLimiter;
use log::{info, error};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting JPEG crawler");

    let urls = crawler::generate_urls(116);
    let rate_limiter = Arc::new(RateLimiter::new(1));
    let multi_progress = Arc::new(MultiProgress::new());
    let overall_progress = multi_progress.add(ProgressBar::new(urls.len() as u64));
    overall_progress.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
        .unwrap()
        .progress_chars("##-"));

    let semaphore = Arc::new(Semaphore::new(5)); // Limit concurrent downloads

    let futures = urls.into_iter().enumerate().map(|(i, url)| {
        let rate_limiter = Arc::clone(&rate_limiter);
        let overall_progress = overall_progress.clone();
        let multi_progress = Arc::clone(&multi_progress);
        let semaphore = Arc::clone(&semaphore);

        async move {
            let _permit = semaphore.acquire().await.unwrap();
            rate_limiter.wait().await;

            let progress_bar = multi_progress.add(ProgressBar::new(100));
            progress_bar.set_style(ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.green/red} {percent}% {msg}")
                .unwrap()
                .progress_chars("##-"));
            progress_bar.set_message(format!("Page {}", i + 1));

            match crawler::fetch_largest_jpeg_url(&url).await {
                Ok(jpeg_url) => {
                    if let Err(e) = downloader::download_image(&jpeg_url, &url, progress_bar).await {
                        error!("Failed to download image from {}: {}", url, e);
                    }
                }
                Err(e) => error!("Failed to fetch JPEG URL from {}: {}", url, e),
            }

            overall_progress.inc(1);
        }
    });

    futures::future::join_all(futures).await;

    overall_progress.finish_with_message("Crawling complete");
    multi_progress.clear().unwrap();

    info!("JPEG crawler finished");
    Ok(())
}