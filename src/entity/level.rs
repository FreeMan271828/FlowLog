use serde::Deserialize;
use strum::EnumString;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumString, Deserialize)]
pub enum LogLevel {
    Dbg, Log, Warn, Err
}