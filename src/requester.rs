/// 请求器：负责构建和管理 HTTP 请求
pub struct Requester {
    /// 目标 URL
    url: String,
    /// 可选的 User-Agent 头
    user_agent: Option<String>,
}

impl Requester {
    /// 创建新的请求器实例
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            user_agent: None,
        }
    }

    /// 设置 User-Agent
    pub fn with_user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_string());
        self
    }

    /// 获取 URL
    pub fn url(&self) -> &str {
        &self.url
    }

    /// 获取 User-Agent
    pub fn user_agent(&self) -> Option<&str> {
        self.user_agent.as_deref()
    }
}
