use serde::Deserialize;
use strum::EnumString;

use crate::{core::record::LogRecord};
use crate::sinks::console::ConsoleSink;
use crate::sinks::file::FileSink;

pub mod console;
pub mod file;

#[derive(PartialEq, Eq, EnumString, Debug, Clone, Deserialize)]
pub enum SinkType{
    Console, File, S3, Elastic,
}

pub trait Sink{
    fn redirect(self: &Self, record: &LogRecord) -> Result<(), std::io::Error>;
    fn new() -> &'static Self;
}

pub fn sink(sink_type: &SinkType, record: &LogRecord) -> Result<(), std::io::Error> {
    match sink_type {
        SinkType::Console => ConsoleSink::new().redirect(record),
        SinkType::File => FileSink::new().redirect(record),
        SinkType::S3 => todo!(),
        SinkType::Elastic => todo!(),
    }
}