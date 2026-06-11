use serde::Deserialize;
use strum::EnumString;

use crate::{Configurable, LogHandler, entity::record::LogRecord};
use console_log::console_sink::ConsoleSink;
use file_log::file_sink::FileSink;
use s3_log::s3_sink::S3Sink;

pub mod file_log;
pub mod console_log;
pub mod s3_log;
pub mod pipeline;

#[derive(PartialEq, Eq, EnumString, Debug, Clone, Deserialize)]
pub enum SinkType{
    Console, File, S3, Elastic,
}

pub fn sink(sink_type: &SinkType, record: &LogRecord) -> Result<(), std::io::Error> {
    match sink_type {
        SinkType::Console => ConsoleSink::new().read().unwrap().redirect(record),
        SinkType::File => FileSink::new().read().unwrap().redirect(record),
        SinkType::S3 => S3Sink::new().read().unwrap().redirect(record),
        SinkType::Elastic => todo!(),
    }
}

pub fn get_reload() -> Vec<fn()>{
    vec![
        S3Sink::reload, ConsoleSink::reload,
        FileSink::reload,
    ]   
}