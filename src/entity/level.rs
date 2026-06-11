use serde::{Deserialize, Serialize};
use strum::EnumString;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumString, Serialize, Deserialize)]
pub enum LogLevel {
    Dbg, Log, Warn, Err
}