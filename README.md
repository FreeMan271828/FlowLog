flowlog/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── core/
│   │   ├── mod.rs
│   │   ├── level.rs          # 日志级别
│   │   ├── record.rs         # 日志记录
│   │   └── config.rs         # 配置管理
│   ├── pipeline/
│   │   ├── mod.rs
│   │   ├── emitter.rs        # 发射器（日志记录产生）
│   │   └── processor.rs      # 处理器（中间处理）
│   ├── sinks/
│   │   ├── mod.rs
│   │   ├── console.rs        # 控制台输出
│   │   ├── file.rs           # 文件输出
│   │   ├── s3.rs             # S3 输出
│   │   ├── elastic.rs        # Elasticsearch 输出
│   │   ├── kafka.rs          # Kafka 输出
│   │   └── chain.rs          # 链式输出
│   ├── formatters/
│   │   ├── mod.rs
│   │   ├── text.rs           # 文本格式
│   │   ├── json.rs           # JSON 格式
│   │   └── custom.rs         # 自定义格式
│   └── macros.rs             # 宏定义
└── examples/
    ├── basic.rs
    ├── multi_sink.rs
    └── custom_sink.rs