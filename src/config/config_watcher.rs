use core::time;
use std::{path::Path, sync::{Arc, Mutex, OnceLock, atomic::AtomicBool, mpsc::{self}}, thread::JoinHandle};
use std::result::Result::Err;
use notify::{RecommendedWatcher, Watcher, event};

use crate::{constants, get_reload};

static WATCHER_HANDLE: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);
static STOP_HANDLE: OnceLock<Arc<AtomicBool>> = OnceLock::new();

pub fn start_config_watch() -> notify::Result<()>{
    let stop_flag = Arc::new(AtomicBool::new(false));
    STOP_HANDLE.set(stop_flag.clone()).ok();

    let handle = std::thread::spawn(||{
        if let Err(e) = watch(stop_flag){
            eprintln!("{}",e);
        }
    });
    if let Ok(mut handle_guard ) = WATCHER_HANDLE.lock(){
        *handle_guard = Some(handle);
    }
    Ok(())
}

pub fn stop_config_watch(){
    if let Some(flag) = STOP_HANDLE.get(){
        flag.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    if let Ok(mut handle_guard) = WATCHER_HANDLE.lock(){
        if let Some(handle) = handle_guard.take(){
            handle.join().ok();
        }
    }
}

fn watch(stop_flag: Arc<AtomicBool>) -> notify::Result<()>{
    let (tx, rx) = mpsc::channel();
    let mut watcher = RecommendedWatcher::new(
        move |res|{
            match res {
                Ok(event) => {
                    tx.send(event).unwrap();
                }
                _ => println!("Err while start watching"),
            }
        } , 
        notify::Config::default()
    )?;

    let path = Path::new(constants::CONFIG_PATH);
    watcher.watch(path, notify::RecursiveMode::NonRecursive)?;
    println!("Start watching config");
    loop {
        match rx.recv_timeout(time::Duration::from_secs(1)) {
            Ok(event) => {
                if let event::EventKind::Modify(_) = event.kind{
                    let reloads = get_reload();
                    for reload in reloads{
                        reload()
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if stop_flag.load(std::sync::atomic::Ordering::SeqCst){
                    println!("Receive stop flag, stopping monitoring");
                    break;
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                println!("The channel is Disconnected, stopping monitoring");
                break;
            },
        }
    }
    Ok(())
}