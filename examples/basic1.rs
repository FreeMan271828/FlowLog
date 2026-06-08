use betterlog::*;

pub fn main(){
    debug!("Test dbg");
    log!("Test log");
    warn!("警告：内存占用 {} 过高！", 90);
    err!("Test Err");
}