// 引入所需的依赖
use crate::parser::Parser;
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;

/// 保存器特征：定义数据保存的接口
#[async_trait]
pub trait Saver<T> {
    /// 保存数据的异步方法
    async fn save(&self, data: T) -> Result<()>;
}

/// 文件保存器：将数据保存到文件系统
pub struct FileSaver<P: Parser> {
    parser: P,
    save_path: String,
}

impl<P: Parser> FileSaver<P> {
    /// 创建新的文件保存器实例
    pub fn new(parser: P, save_path: &str) -> Self {
        Self {
            parser,
            save_path: save_path.to_string(),
        }
    }
}

/// 为文件保存器实现 Saver 特征
#[async_trait]
impl<P: Parser + Send + Sync> Saver<P::Output> for FileSaver<P>
where
    P::Output: serde::Serialize + Send,
{
    /// 将数据序列化为 JSON 并写入文件
    async fn save(&self, data: P::Output) -> Result<()> {
        let path = Path::new(&self.save_path);
        let json = serde_json::to_string_pretty(&data)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }
}
