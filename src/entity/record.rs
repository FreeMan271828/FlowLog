use std::borrow::Cow;

use chrono::{DateTime, Local};

use crate::entity::level::LogLevel;

#[allow(dead_code)]
#[derive(Debug)]
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

    pub fn as_bytes(&self) -> Vec<u8> {
        format!(
            "[{}] [{:?}] ({}:{}) - {}\n",
            self.timestamp.format("%H:%M:%S%.3f"),
            self.level,
            self.file.unwrap_or("unknown"),
            self.line.unwrap_or(0),
            self.body
        ).into_bytes()
    }
}