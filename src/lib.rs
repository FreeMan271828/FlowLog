pub mod entity;
pub mod formatters;
pub mod pipeline;
pub mod sinks;
pub mod macros;
pub mod constants;
pub mod config;
pub mod tools;

use std::{io, sync::{Arc, RwLock}};

pub use crate::entity::level::LogLevel;
use crate::{entity::record::LogRecord, pipeline::translator::LogTranslator, sinks::{console_sink::ConsoleSink, file_sink::FileSink, s3_sink::S3Sink}};
pub use crate::pipeline::emitter::LogEmitter;

/// 获取全部的重新加载函数，对于config的热加载需要
pub fn get_reload() -> Vec<fn()>{
    vec![
        FileSink::reload, ConsoleSink::reload, LogTranslator::reload, S3Sink::reload
    ]
}

pub trait Configurable {    
    fn new() -> Arc<RwLock<Self>> where Self: Sized;
    fn reload();
}

// 日志处理相关的 trait
pub trait LogHandler {
    fn handle(&self, record: &LogRecord) -> Result<(), io::Error>;
    fn redirect(&self, record: &LogRecord) -> Result<(), io::Error> {
        // 默认实现可以调用 handle
        self.handle(record)
    }
}