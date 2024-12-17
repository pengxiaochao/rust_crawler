# Rust 异步网络爬虫框架

一个基于 Rust 实现的高性能异步网络爬虫框架，采用模块化设计，支持自定义解析器和数据保存方式。

## 特性

- 异步并发处理
- 可配置的请求延迟和并发限制
- 模块化的设计架构
- 类型安全的数据处理
- 支持自定义解析器和保存器

## 核心组件

### 1. 下载器 (Downloader)
- 负责处理 HTTP 请求
- 支持并发请求限制
- 可配置请求延迟，避免对目标站点造成压力
- 使用信号量控制并发数

### 2. 解析器 (Parser)
- 定义了统一的解析接口
- 支持自定义解析逻辑
- 类型安全的数据输出
- 异步处理能力

### 3. 保存器 (Saver)
- 提供数据持久化接口
- 支持多种保存方式（文件、数据库等）
- 异步保存能力
- 类型安全的数据处理

### 4. 调度器 (Scheduler)
- 管理爬虫任务的调度
- 处理请求队列
- 协调解析结果的传递
- 支持任务的异步处理

### 5. 请求器 (Requester)
- 构建 HTTP 请求
- 支持自定义 User-Agent
- 灵活的请求配置

## 快速开始

1. 添加依赖到 `Cargo.toml`:
```toml
[dependencies]
rust_crawler = { path = "." }
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
```

2. 创建自定义解析器:
```rust
#[derive(Clone)]
struct MyParser;

#[async_trait::async_trait]
impl Parser for MyParser {
    type Output = String;
    
    async fn parse(&self, content: &str) -> Result<Self::Output> {
        // 实现您的解析逻辑
        Ok(content.to_string())
    }
}
```

3. 创建自定义保存器:
```rust
struct MySaver;

#[async_trait::async_trait]
impl Saver<String> for MySaver {
    async fn save(&self, data: String) -> Result<()> {
        // 实现您的保存逻辑
        println!("保存数据: {}", data);
        Ok(())
    }
}
```

4. 运行爬虫:
```rust
#[tokio::main]
async fn main() -> Result<()> {
    let config = DownloaderConfig::new(3, 1000);
    let downloader = Arc::new(Downloader::with_config(config));
    let mut scheduler = Scheduler::<MyParser>::new(10);
    let parser = Arc::new(MyParser);
    let saver = Arc::new(MySaver);
    
    // 设置爬虫逻辑...
}
```

## 设计思路

1. **模块化设计**
   - 每个组件都有明确的职责
   - 组件之间通过特征（trait）进行解耦
   - 便于扩展和维护

2. **异步处理**
   - 使用 tokio 作为异步运行时
   - 充分利用系统资源
   - 提高爬取效率

3. **类型安全**
   - 利用 Rust 的类型系统确保数据安全
   - 编译时错误检查
   - 避免运行时错误

4. **资源控制**
   - 并发限制
   - 请求延迟
   - 避免对目标站点造成过大压力

## 后续改进方向

1. **功能增强**
   - 添加代理支持
   - 增加重试机制
   - 支持 Cookie 管理
   - 添加更多的请求方法支持

2. **性能优化**
   - 优化内存使用
   - 改进调度算法
   - 添加缓存机制

3. **可用性提升**
   - 提供更多示例
   - 完善错误处理
   - 增加日志系统
   - 添加监控指标

4. **扩展性改进**
   - 提供插件系统
   - 支持中间件
   - 添加更多预设的解析器
   - 支持分布式部署

## 贡献指南

欢迎提交 Issue 和 Pull Request 来帮助改进项目。在提交代码前，请确保：

1. 代码符合 Rust 代码规范
2. 添加了适当的测试
3. 更新了相关文档
4. 遵循现有的代码风格

## 许可证

MIT License