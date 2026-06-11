# FlowLog

FlowLog 是一个用 Rust 编写的高性能、模块化日志处理与流转框架。它支持灵活的日志管道（Pipeline）配置，并内置了多种输入源、格式化工具和输出后端（Sinks）。

## 特性

* **多后端输出 (Sinks)**：支持控制台、本地文件、AWS S3、Elasticsearch 以及链式组合输出。
* **灵活的格式化 (Formatters)**：内置纯文本、JSON 格式，并支持高度自定义格式。
* **管道化处理 (Pipeline)**：采用“发射器 -> 处理器 -> 接收器”的清晰架构，方便扩展中间件。
* **配置文件热加载**：通过notify和观察者模式实现配置文件的观察和各个处理器的重载。

---

## 快速上手

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

level = "Log"               # 日志输出级别：Dbg, Log, Warn, Err
enable_async = true         # 是否开启异步日志流
buffer_size = 2048          # 异步模式下的缓冲区大小（队列容量）

# 日志深潜方式，Console, File, S3, Elastic
sink_type = ["S3", "Console"]

# 文件深潜的配置
dir_path = "./log"          # 日志的文件输出文件夹
max_size = 10485760         # 单个文件的最大大小, 10MB
rotate_num = 1              # 最多保留的文件数目

# S3的配置
access_key = ""
secret_key = ""
provider_name = "freeman"
bucket = "log-data"
prefix = "test"  # 不同的项目设置不同的前缀
region = "us-east-1"
end_point_url = ""
force_path_style = true

# S3深潜的配置
put_size = 10485760  # 本地临时文件上传的大小
put_min_ratio = 0.4  # 本地临时文件上传的最小比例
```

---

## 核心架构

* **Emitter**：负责捕获系统或业务产生的日志，并将其转化为标准的 `Record`。
* **Translator**：常用于日志的动态过滤（如根据 `Level` 筛选）以及发送日志数据给各个Sink。
* **Sink**：负责将格式化后的数据发送至最终的目的地（终端、文件或云端）。

不同的处理器之间采用松耦合开发，通过集成函数数组来互相调用。

在entity中定义了全局实体类
在tools中包含有工具的集成驱动，包括s3、file
在pipeline中实现了发射器和中转器，为macros提供实现
在service中实现了具体的存储方式，有Console、S3、File，每一个都有config和具体的service。
在lib中定义了
- get_reload：聚合service的reload函数
- sink：日志深潜，聚合了全部的service的使用，把SinkType转化成具体的行为