use crate::sinks::Sink;

pub struct Console;

impl Sink for Console {
    fn redirect(record: &crate::core::record::LogRecord) {
        println!(
            "[{}] [{:?}] ({}:{}) - {}",
            record.timestamp.format("%m-%d %H:%M:%S%.1f"),
            record.level,
            record.file.unwrap_or("unknown"),
            record.line.unwrap_or(0),
            record.body
        );
    }
}