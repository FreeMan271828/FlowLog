use serde::Deserialize;
use strum::EnumString;

use crate::{core::record::LogRecord};

pub mod console;
pub mod file;

#[derive(PartialEq, Eq, EnumString, Debug, Clone, Deserialize)]
pub enum SinkType{
    Console, File, S3, Elastic,
}

pub trait Sink{
    fn redirect(record: &LogRecord);
}

pub fn sink(sink_type: &SinkType, record: &LogRecord){
    match sink_type {
        SinkType::Console => console::Console::redirect(record),
        SinkType::File => todo!(),
        SinkType::S3 => todo!(),
        SinkType::Elastic => todo!(),
    }
}