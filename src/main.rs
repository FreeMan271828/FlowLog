use flowlog::{debug, err, log, warn};

pub fn main(){
    debug!("Test dbg");
    log!("Test log");
    warn!("Warn the memory usage is{}", 90);
    err!("Test Err");
}