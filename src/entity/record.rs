use std::borrow::Cow;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::entity::level::LogLevel;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct LogRecord<'a>{
    pub timestamp: DateTime<Local>,
    pub level: LogLevel,
    pub target: Cow<'a, str>,
    pub file: Option<&'a str>,
    pub line: Option<u32>,
    pub body: Cow<'a, str>,
}

#[allow(dead_code)]
impl<'a> LogRecord<'a> {
    pub fn new(level: LogLevel, body: impl Into<Cow<'a, str>>) -> Self {
        Self {
            timestamp: Local::now(),
            level,
            target: Cow::Borrowed(""),
            file: None,
            line: None,
            body: body.into(),
        }
    }
}