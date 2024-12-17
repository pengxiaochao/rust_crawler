use crate::requester::Requester;
use anyhow::Result;
use reqwest::Client;
use std::time::Duration;
use tokio::sync::Semaphore;
use std::sync::Arc;

pub struct DownloaderConfig {
    concurrent_requests: usize,
    request_delay: Duration,
}

impl Default for DownloaderConfig {
    fn default() -> Self {
        Self {
            concurrent_requests: 3,
            request_delay: Duration::from_millis(1000),
        }
    }
}

impl DownloaderConfig {
    pub fn new(concurrent_requests: usize, request_delay_ms: u64) -> Self {
        Self {
            concurrent_requests,
            request_delay: Duration::from_millis(request_delay_ms),
        }
    }
}

pub struct Downloader {
    client: Client,
    config: DownloaderConfig,
    semaphore: Arc<Semaphore>,
}

impl Downloader {
    pub fn new() -> Self {
        Self::with_config(DownloaderConfig::default())
    }

    pub fn with_config(config: DownloaderConfig) -> Self {
        Self {
            client: Client::new(),
            semaphore: Arc::new(Semaphore::new(config.concurrent_requests)),
            config,
        }
    }

    pub async fn download(&self, requester: &Requester) -> Result<String> {
        let _permit = self.semaphore.acquire().await?;
        
        let response = self.client.get(requester.url()).send().await?;
        let content = response.text().await?;
        
        tokio::time::sleep(self.config.request_delay).await;
        
        Ok(content)
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}
