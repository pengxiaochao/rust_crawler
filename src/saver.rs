// 引入所需的依赖
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
pub struct FileSaver {
    /// 保存目录路径
    save_dir: String,
}

impl FileSaver {
    /// 创建新的文件保存器实例
    pub fn new(save_dir: &str) -> Self {
        Self {
            save_dir: save_dir.to_string(),
        }
    }

    /// 生成唯一文件名
    fn generate_filename(&self, extension: &str) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        format!("{}/{}.{}", self.save_dir, timestamp, extension)
    }
}

/// 为文件保存器实现 Saver 特征
#[async_trait]
impl<T> Saver<T> for FileSaver
where
    T: serde::Serialize + Send + 'static,
{
    /// 将数据序列化为 JSON 并写入文件
    async fn save(&self, data: T) -> Result<()> {
        let path_str = self.generate_filename("json");
        let path = Path::new(&path_str);
        
        // 确保目录存在
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        let json = serde_json::to_string_pretty(&data)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }
}
