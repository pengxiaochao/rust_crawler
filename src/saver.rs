use crate::parser::Parser;
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;

#[async_trait]
pub trait Saver<T> {
    async fn save(&self, data: T) -> Result<()>;
}

pub struct FileSaver<P: Parser> {
    parser: P,
    save_path: String,
}

impl<P: Parser> FileSaver<P> {
    pub fn new(parser: P, save_path: &str) -> Self {
        Self {
            parser,
            save_path: save_path.to_string(),
        }
    }
}

#[async_trait]
impl<P: Parser + Send + Sync> Saver<P::Output> for FileSaver<P>
where
    P::Output: serde::Serialize + Send,
{
    async fn save(&self, data: P::Output) -> Result<()> {
        let path = Path::new(&self.save_path);
        let json = serde_json::to_string_pretty(&data)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }
}
