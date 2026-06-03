pub mod core;
pub mod formatters;
pub mod pipeline;
pub mod sinks;
pub mod macros;

/// 将宏内部需要用到的核心组件，重导出到库的根路径下
pub use crate::core::level::LogLevel;
pub use crate::pipeline::emitter::LogEmitter;
