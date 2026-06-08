use crate::entity::record::LogRecord;
use crate::sinks::Sink;

pub struct ConsoleSink;

impl Sink for ConsoleSink {
    fn redirect(self: &ConsoleSink, record: &LogRecord) -> Result<(), std::io::Error> {
        println!(
            "[{}] [{:?}] ({}:{}) - {}",
            record.timestamp.format("%H:%M:%S%.3f"),
            record.level,
            record.file.unwrap_or("unknown"),
            record.line.unwrap_or(0),
            record.body
        );
        Ok(())
    }

    fn new() -> &'static Self{
        &ConsoleSink {}
    }
}