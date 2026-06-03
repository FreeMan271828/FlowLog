use logger::{debug, err, log, warn};

#[test]
pub fn test_logging_outputs(){
    debug!("Test dbg");
    log!("Test log");
    warn!("警告：内存占用 {} 过高！", 90);
    err!("Test Err");
}