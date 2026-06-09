use std::sync::{Arc, OnceLock, RwLock};

use crate::{Configurable, LogHandler, config::{ConfigTrait, s3_config::S3Config}};

static INSTANCE: OnceLock<Arc<RwLock<S3Sink>>> = OnceLock::new();

pub struct S3Sink{
    config: S3Config,
}

impl Configurable for S3Sink {
    fn new() -> Arc<RwLock<Self>> where Self: Sized {
        INSTANCE.get_or_init(|| {
            let s3_sink = Self::create_s3_sink();
            let ins = Arc::new(RwLock::new(s3_sink));
            
            ins
        }).clone()
    }

    fn reload() {
        if let Some(instance) = INSTANCE.get(){
            let new_sink = Self::create_s3_sink();
            if let Ok(mut sink) = instance.write() {
                *sink = new_sink;
                println!("The s3 config has updated");
            }
        }
    }
}

impl LogHandler for S3Sink {
    fn handle(&self, record: &crate::entity::record::LogRecord) -> Result<(), std::io::Error> {
        if !self.config.is_valid(){
            println!("Something err while handling S3, you can choose to remove S3 option")
            return Ok(());
        }
        
        Ok(())
    }
}

impl S3Sink {
    
    fn create_s3_sink() -> S3Sink{
        let config = S3Config::load().unwrap_or_else(|_| {
            S3Config::default()
        });
        S3Sink { config }
    }
}