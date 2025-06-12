use crate::requester::Requester;
use anyhow::Result;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

/// 下载器配置
pub struct DownloaderConfig {
    /// 最大并发请求数
    concurrent_requests: usize,
    /// 请求间隔时间
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
    /// 创建新的下载器配置
    /// ### 参数
    /// * `concurrent_requests` - 最大并发请求数
    /// * `request_delay_ms` - 请求间隔时间（毫秒）
    pub fn new(concurrent_requests: usize, request_delay_ms: u64) -> Self {
        Self {
            concurrent_requests,
            request_delay: Duration::from_millis(request_delay_ms),
        }
    }
}

/// 下载器：处理 HTTP 请求
pub struct Downloader {
    /// HTTP 客户端
    client: Client,
    /// 下载器配置
    config: DownloaderConfig,
    /// 并发控制信号量
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

        let response = self
            .client
            .get(requester.url())
            .header(
                "user-agent".to_string(),
                requester.user_agent().or(Some("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")).unwrap().to_string(),
            )
            .send()
            .await?;
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
