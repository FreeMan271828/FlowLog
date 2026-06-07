# FlowLog

FlowLog 是一个用 Rust 编写的高性能、模块化日志处理与流转框架。它支持灵活的日志管道（Pipeline）配置，并内置了多种输入源、格式化工具和输出后端（Sinks）。

## ✨ 特性

* **多后端输出 (Sinks)**：支持控制台、本地文件、AWS S3、Elasticsearch 以及链式组合输出。
* **灵活的格式化 (Formatters)**：内置纯文本、JSON 格式，并支持高度自定义格式。
* **管道化处理 (Pipeline)**：采用“发射器 -> 处理器 -> 接收器”的清晰架构，方便扩展中间件。
* **轻量高效**：基于 Rust 构建，零成本抽象，内存安全且极致高效。

---

## 📂 项目结构

```text
FlowLog/
├── Cargo.toml          # 项目配置与依赖管理
├── README.md           # 项目文档
├── src/                # 源代码目录
│   ├── lib.rs          # 库入口
│   ├── macros.rs       # 核心宏定义
│   ├── core/           # 核心模块
│   │   ├── level.rs    # 日志级别定义 (Debug, Info, Warn, Error 等)
│   │   ├── record.rs   # 日志记录实体结构
│   │   └── config.rs   # 框架配置管理
│   ├── pipeline/       # 管道处理流
│   │   ├── emitter.rs  # 日志发射器 (日志产生源)
│   │   └── processor.rs# 日志处理器 (中间件/过滤/转换)
│   ├── sinks/          # 输出后端 (数据落地)
│   │   ├── console.rs  # 标准控制台输出
│   │   ├── file.rs     # 本地文件追加输出
│   │   ├── s3.rs       # AWS S3 对象存储对接
│   │   ├── elastic.rs  # Elasticsearch 分布式检索对接
│   │   └── chain.rs    # 链式输出 (同时投递到多个 Sink)
│   └── formatters/     # 文本格式化
│       ├── text.rs     # 标准文本/自定义文本排版
│       ├── json.rs     # 结构化 JSON 字符串转换
│       └── custom.rs   # 用户自定义格式化接口
└── examples/           # 示例代码
    ├── basic.rs        # 基础快速上手示例
    ├── multi_sink.rs   # 多后端同步输出示例
    └── custom_sink.rs  # 自定义输出后端扩展示例
```

---

## 🚀 快速上手

### 1. 添加依赖

在您的 `Cargo.toml` 中引用本仓库（本地路径或 Git 仓库）：

```toml
[dependencies]
flowlog = { path = "./path/to/FlowLog" }
```

### 2. 添加配置文件

在您的 `config` 文件夹下添加 `log.toml`，下面是实例

```toml
# config/log.toml

level = "Dbg"              # 日志输出级别：Dbg, Log, Warn, Err
enable_async = true         # 是否开启异步日志流
buffer_size = 2048          # 异步模式下的缓冲区大小（队列容量）

dir_path = "./log"         # 日志的文件输出文件夹
max_size = 10485760         # 单个文件的最大大小
rotate_num = 5              # 最多保留的文件数目

# 日志重定向深潜方式，Console, File, S3, Elastic
sink_type = ["Console", "File"]
```

### 3. 基础使用示例

运行内置的示例代码来查看效果：

```bash
cargo run --example basic1
```

核心代码片段：

```rust
use flowlog::core::{Config, Level};
use flowlog::sinks::ConsoleSink;
use flowlog::formatters::TextFormatter;

fn main() {
    debug!("Test dbg");
    log!("Test log");
    warn!("警告：内存占用 {} 过高！", 90);
    err!("Test Err");
}
```

---

## 🛠️ 核心架构概念

* **Emitter (发射器)**：负责捕获系统或业务产生的日志，并将其转化为标准的 `Record`。
* **Processor (处理器)**：常用于日志的动态过滤（如根据 `Level` 筛选）或字段脱敏。
* **Formatter (格式化器)**：将结构化的日志数据序列化为指定的文本或字节流。
* **Sink (接收器)**：负责将格式化后的数据发送至最终的目的地（终端、文件或云端）。
