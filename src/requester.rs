pub struct Requester {
    url: String,
    user_agent: Option<String>,
}

impl Requester {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            user_agent: None,
        }
    }

    pub fn with_user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_string());
        self
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn user_agent(&self) -> Option<&str> {
        self.user_agent.as_deref()
    }
}
