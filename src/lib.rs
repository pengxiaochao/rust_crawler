/// 解析器模块：负责内容解析
pub mod parser;
/// 下载器模块：负责网络请求
pub mod downloader;
/// 请求器模块：构建请求
pub mod requester;
/// 保存器模块：数据持久化
pub mod saver;
/// 调度器模块：任务调度
pub mod scheduler;

// 重导出主要组件
pub use downloader::{Downloader, DownloaderConfig};
pub use parser::{HtmlParser, Parser};
pub use requester::Requester;
pub use saver::Saver;
pub use scheduler::Scheduler;
