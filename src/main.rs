use rust_crawler::{Downloader, Parser, Scheduler, Saver, downloader::DownloaderConfig};
use std::sync::Arc;
use anyhow::Result;
use futures::future::join_all;

#[derive(Clone)]
struct SimpleParser;

#[async_trait::async_trait]
impl Parser for SimpleParser {
    type Output = String;
    async fn parse(&self, content: &str) -> Result<Self::Output> {
        Ok(content.to_string())
    }
}

struct SimpleSaver;

#[async_trait::async_trait]
impl Saver<String> for SimpleSaver {
    async fn save(&self, data: String) -> Result<()> {
        println!("Saved content length: {}", data.len());
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let urls = vec![
        "https://www.rust-lang.org".to_string(),
        "https://docs.rs".to_string(),
        "https://crates.io".to_string(),
    ];

    let config = DownloaderConfig::new(3, 1000);
    let downloader = Arc::new(Downloader::with_config(config));
    let mut scheduler = Scheduler::<SimpleParser>::new(10);
    let parser = Arc::new(SimpleParser);
    let saver = Arc::new(SimpleSaver);

    // 分离 scheduler 为发送者和接收者
    let (sender, receiver) = scheduler.split();
    let sender = Arc::new(sender);
    let receiver = Arc::new(tokio::sync::Mutex::new(receiver));

    // 添加URLs到调度器
    sender.add_requests(urls).await?;

    // 启动下载任务
    let download_tasks = (0..3).map(|_| {
        let downloader = downloader.clone();
        let receiver = receiver.clone();
        let sender = sender.clone();
        let parser = parser.clone();
        
        tokio::spawn(async move {
            loop {
                let req = {
                    let mut receiver = receiver.lock().await;
                    receiver.get_request().await
                };
                
                match req {
                    Some(req) => {
                        if let Ok(content) = downloader.download(&req).await {
                            if let Ok(parsed) = parser.parse(&content).await {
                                let _ = sender.add_parsed_data(parsed).await;
                            }
                        }
                    }
                    None => break,
                }
            }
        })
    });

    // 启动保存任务
    let save_tasks = (0..3).map(|_| {
        let receiver = receiver.clone();
        let saver = saver.clone();
        
        tokio::spawn(async move {
            loop {
                let data = {
                    let mut receiver = receiver.lock().await;
                    receiver.get_parsed_data().await
                };
                
                match data {
                    Some(data) => {
                        let _ = saver.save((*data).clone()).await;
                    }
                    None => break,
                }
            }
        })
    });

    // 等待所有任务完成
    let all_tasks = download_tasks.chain(save_tasks).collect::<Vec<_>>();
    join_all(all_tasks).await;

    Ok(())
}
