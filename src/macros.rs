#[macro_export]
macro_rules! logger {
    ($level:expr, $($arg:tt)+) => {
        let _ = &crate::service::pipeline::emitter::LogEmitter::
        emit(
            $level,
            module_path!(),
            file!(),
            line!(),
            format!($($arg)+),
        );
    };
}

#[macro_export]
macro_rules! debug { ($($arg:tt)+) => { $crate::logger!($crate::LogLevel::Dbg, $($arg)+); }; }

#[macro_export]
macro_rules! warn { ($($arg:tt)+) => { $crate::logger!($crate::LogLevel::Warn, $($arg)+); }; }

#[macro_export]
macro_rules! err { ($($arg:tt)+) => { $crate::logger!($crate::LogLevel::Err, $($arg)+); }; }

#[macro_export]
macro_rules! log { ($($arg:tt)+) => { $crate::logger!($crate::LogLevel::Log, $($arg)+); }; }
