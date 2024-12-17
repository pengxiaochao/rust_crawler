use crate::downloader::Downloader;
use anyhow::Result;
use async_trait::async_trait;

/// 解析器特征：定义内容解析的接口
#[async_trait]
pub trait Parser: Send + Sync {
    /// 解析器输出类型
    type Output: Send + Sync + Clone + 'static;
    /// 解析内容的异步方法
    async fn parse(&self, content: &str) -> Result<Self::Output>;
}

/// HTML 解析器：处理 HTML 内容
pub struct HtmlParser {
    downloader: Downloader,
}

impl HtmlParser {
    /// 创建新的 HTML 解析器实例
    pub fn new(downloader: Downloader) -> Self {
        Self { downloader }
    }

    /// 下载并解析内容
    pub async fn download_and_parse<P: Parser + Sync>(&self, url: &str, parser: &P) -> Result<P::Output> {
        let requester = crate::requester::Requester::new(url);
        let content = self.downloader.download(&requester).await?;
        parser.parse(&content).await
    }
}
