use serde::Deserialize;
use strum::EnumString;

use crate::{Configurable, LogHandler};
use crate::{entity::record::LogRecord};
use crate::sinks::console_sink::ConsoleSink;
use crate::sinks::file_sink::FileSink;

pub mod console_sink;
pub mod s3_sink;
pub mod file_sink;

/// 对于sink_type到具体执行的映射
pub fn sink(sink_type: &SinkType, record: &LogRecord) -> Result<(), std::io::Error> {
    match sink_type {
        SinkType::Console => ConsoleSink::new().read().unwrap().redirect(record),
        SinkType::File => FileSink::new().read().unwrap().redirect(record),
        SinkType::S3 => todo!(),
        SinkType::Elastic => todo!(),
    }
}


#[derive(PartialEq, Eq, EnumString, Debug, Clone, Deserialize)]
pub enum SinkType{
    Console, File, S3, Elastic,
}