use core::time;
use std::thread;

use betterlog::{config::config_watcher::start_config_watch, *};

pub fn main(){
    if let Ok(_) = start_config_watch(){
        let mut index = 0;
        loop {
            log!("index: {}", index);
            debug!("Test dbg");
            log!("Test log");
            warn!("警告：内存占用 {} 过高！", 90);
            err!("Test Err");   
            thread::sleep(time::Duration::from_secs(3));
            index = index + 1;
            if index == 20{
                break;
            }
        }
    }
}