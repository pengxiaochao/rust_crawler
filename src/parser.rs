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

/// HTML 解析结果
#[derive(Clone, Debug)]
pub struct HtmlParseResult {
    /// 页面标题
    pub title: Option<String>,
    /// 提取的链接
    pub links: Vec<String>,
    /// 原始内容长度
    pub content_length: usize,
}

/// HTML 解析器：处理 HTML 内容，提取链接和标题
#[derive(Clone, Default)]
pub struct HtmlParser;

impl HtmlParser {
    /// 创建新的 HTML 解析器实例
    pub fn new() -> Self {
        Self
    }

    /// 简单提取 HTML 中的链接（使用正则模式匹配）
    fn extract_links(content: &str) -> Vec<String> {
        let mut links = Vec::new();
        // 简单的链接提取：查找 href="..." 模式
        for part in content.split("href=\"") {
            if let Some(end) = part.find('"') {
                let link = &part[..end];
                if link.starts_with("http") {
                    links.push(link.to_string());
                }
            }
        }
        links
    }

    /// 简单提取 HTML 标题
    fn extract_title(content: &str) -> Option<String> {
        let lower = content.to_lowercase();
        if let Some(start) = lower.find("<title>") {
            let title_start = start + 7;
            if let Some(end) = lower[title_start..].find("</title>") {
                return Some(content[title_start..title_start + end].trim().to_string());
            }
        }
        None
    }
}

#[async_trait]
impl Parser for HtmlParser {
    type Output = HtmlParseResult;

    async fn parse(&self, content: &str) -> Result<Self::Output> {
        Ok(HtmlParseResult {
            title: Self::extract_title(content),
            links: Self::extract_links(content),
            content_length: content.len(),
        })
    }
}
