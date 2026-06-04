use crate::core::level::LogLevel;
use crate::core::record::LogRecord;
use crate::pipeline::processor::LogProcessor;
use std::borrow::Cow;
/// 日志发送器，负责日志的组装、发送
pub struct LogEmitter;

impl LogEmitter {
    pub fn emit(
        level: LogLevel,
        target: &'static str,
        file: &'static str,
        line: u32,
        body: String) 
    {
        let mut record = LogRecord::new(level, body);
        record.target = Cow::Borrowed(target);
        record.file = Some(file);
        record.line = Some(line);
        LogProcessor::global().process(&record);
    }
}
