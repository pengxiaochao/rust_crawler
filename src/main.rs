use rust_crawler::{Downloader, Parser, Scheduler, Saver, downloader::DownloaderConfig};
use std::sync::Arc;
use anyhow::Result;
use futures::future::join_all;

/// 简单解析器：将内容直接作为字符串返回
#[derive(Clone)]
struct SimpleParser;

#[async_trait::async_trait]
impl Parser for SimpleParser {
    type Output = String;
    async fn parse(&self, content: &str) -> Result<Self::Output> {
        Ok(content.to_string())
    }
}

/// 简单保存器：打印内容长度
struct SimpleSaver;

#[async_trait::async_trait]
impl Saver<String> for SimpleSaver {
    async fn save(&self, data: String) -> Result<()> {
        println!("Saved content length: {}", data.len());
        Ok(())
    }
}

/// 主函数：启动爬虫系统
#[tokio::main]
async fn main() -> Result<()> {
    // 待爬取的 URL 列表
    let urls = vec![
        "https://www.baidu.com".to_string(),
        "https://www.163.com".to_string(),
        "https://www.au92.com".to_string(),
    ];

    // 初始化组件
    let config = DownloaderConfig::new(3, 1000);
    let downloader = Arc::new(Downloader::with_config(config));
    let mut scheduler = Scheduler::<SimpleParser>::new(10);
    let parser = Arc::new(SimpleParser);
    let saver = Arc::new(SimpleSaver);

    // 分离调度器为发送端和接收端
    let (sender, receiver) = scheduler.split();
    let sender = Arc::new(sender);
    // 使用单独的 Mutex 分别管理请求接收和解析数据接收
    let receiver = Arc::new(tokio::sync::Mutex::new(receiver));

    // 添加URLs到调度器
    sender.add_requests(urls).await?;
    
    // 创建一个通道用于通知下载完成
    let (download_done_tx, mut download_done_rx) = tokio::sync::mpsc::channel::<()>(1);

    // 启动下载任务
    let download_tasks: Vec<_> = (0..3).map(|worker_id| {
        let downloader = downloader.clone();
        let receiver = receiver.clone();
        let sender = sender.clone();
        let parser = parser.clone();
        let download_done_tx = download_done_tx.clone();
        
        tokio::spawn(async move {
            loop {
                let req = {
                    let mut receiver = receiver.lock().await;
                    receiver.get_request().await
                };
                
                match req {
                    Some(req) => {
                        match downloader.download(&req).await {
                            Ok(content) => {
                                match parser.parse(&content).await {
                                    Ok(parsed) => {
                                        if let Err(e) = sender.add_parsed_data(parsed).await {
                                            eprintln!("[Worker {}] Failed to send parsed data: {}", worker_id, e);
                                        }
                                    }
                                    Err(e) => eprintln!("[Worker {}] Parse error for {}: {}", worker_id, req.url(), e),
                                }
                            }
                            Err(e) => eprintln!("[Worker {}] Download error for {}: {}", worker_id, req.url(), e),
                        }
                    }
                    None => {
                        // 通道关闭，通知保存任务
                        let _ = download_done_tx.send(()).await;
                        break;
                    }
                }
            }
        })
    }).collect();
    
    // 丢弃原始的 download_done_tx，这样当所有下载任务完成时通道会关闭
    drop(download_done_tx);

    // 启动保存任务
    let save_tasks: Vec<_> = (0..3).map(|worker_id| {
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
                        if let Err(e) = saver.save((*data).clone()).await {
                            eprintln!("[Saver {}] Save error: {}", worker_id, e);
                        }
                    }
                    None => break,
                }
            }
        })
    }).collect();

    // 等待下载完成信号
    let _ = download_done_rx.recv().await;

    // 等待所有任务完成
    let all_tasks = download_tasks.into_iter().chain(save_tasks.into_iter()).collect::<Vec<_>>();
    join_all(all_tasks).await;

    println!("Crawler finished!");
    Ok(())
}
