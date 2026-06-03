use crate::core::level::LogLevel;
use crate::core::record::LogRecord;
use crate::pipeline::processor::LogProcessor;
use std::borrow::Cow;

pub struct LogEmitter;

impl LogEmitter {
    /// 核心发射方法：收集所有上下文信息并构建 LogRecord
    pub fn emit(
        level: LogLevel,
        target: &'static str,
        file: &'static str,
        line: u32,
        body: String,
    ) {
        let mut record = LogRecord::new(level, body);
        record.target = Cow::Borrowed(target);
        record.file = Some(file);
        record.line = Some(line);

        LogProcessor::global().process(&record);
    }
}
