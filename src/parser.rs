use crate::downloader::Downloader;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Parser: Send + Sync {
    type Output: Send + Sync + Clone + 'static;
    async fn parse(&self, content: &str) -> Result<Self::Output>;
}

pub struct HtmlParser {
    downloader: Downloader,
}

impl HtmlParser {
    pub fn new(downloader: Downloader) -> Self {
        Self { downloader }
    }

    pub async fn download_and_parse<P: Parser + Sync>(&self, url: &str, parser: &P) -> Result<P::Output> {
        let requester = crate::requester::Requester::new(url);
        let content = self.downloader.download(&requester).await?;
        parser.parse(&content).await
    }
}
