pub mod core;
pub mod formatters;
pub mod pipeline;
pub mod sinks;
pub mod macros;
pub mod constants;

use config::ConfigError;

pub use crate::core::level::LogLevel;
pub use crate::pipeline::emitter::LogEmitter;

pub trait Config : Default{
    fn load() -> Result<Self, ConfigError> where Self: Sized;
}