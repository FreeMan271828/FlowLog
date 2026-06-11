use crate::{Configurable, LogHandler};
use crate::entity::level::LogLevel;
use crate::entity::record::LogRecord;
use crate::pipeline::translator::LogTranslator;
use std::borrow::Cow;
use std::io::Error;
/// 日志发送器，负责日志的组装、发送给日志处理器
pub struct LogEmitter;

impl LogEmitter {
    pub fn emit(level: LogLevel, target: &'static str,
                file: &'static str, line: u32,body: String) -> Result<(), Error>{
        let mut record = LogRecord::new(level, body);
        record.target = Cow::Borrowed(target);
        record.file = Some(file);
        record.line = Some(line);
        LogTranslator::new().read().unwrap().handle(&record)
    }
}
