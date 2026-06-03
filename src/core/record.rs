use std::borrow::Cow;

use chrono::{DateTime, Utc};

use crate::core::level::LogLevel;

#[allow(dead_code)]
pub struct LogRecord<'a>{
    pub timestamp: DateTime<Utc>,
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
            timestamp: Utc::now(),
            level,
            target: Cow::Borrowed(""),
            file: None,
            line: None,
            body: body.into(),
        }
    }
}