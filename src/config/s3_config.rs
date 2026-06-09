use std::borrow::Cow;

use config::{Config, File};
use serde::Deserialize;

use crate::{config::ConfigTrait, constants};

#[derive(Debug, Deserialize, Clone,)]
pub struct S3Config{
    access_key : Cow<'static, str>,
    secret_key : Cow<'static, str>,
    providor_name : Cow<'static, str>,
    bucket : Cow<'static, str>,
    prefix : Cow<'static, str>, 
    region : Cow<'static, str>,
    end_point_url : Cow<'static, str>,
    force_path_style : bool,
}

impl Default for S3Config {
    fn default() -> Self {
        Self { 
            access_key: Cow::Borrowed(""),
            secret_key: Cow::Borrowed(""), 
            providor_name: Cow::Borrowed(""), 
            bucket: Cow::Borrowed(""), 
            prefix: Cow::Borrowed(""), 
            region: Cow::Borrowed("us-east-1"), 
            end_point_url: Cow::Borrowed(""), 
            force_path_style: true, 
        }
    }
}

impl ConfigTrait for S3Config {
    fn load() -> Result<Self, config::ConfigError> where Self: Sized {
        let s = Config::builder()
            .add_source(File::with_name(constants::CONFIG_PATH).required(false))
            .build()?;
        s.try_deserialize()
    }
}

impl S3Config {
    pub fn is_valid(&self) -> bool{
        let standard = Cow::Borrowed("");
        if standard.eq(&self.access_key) || standard.eq(&self.secret_key) ||
            standard.eq(&self.bucket) || standard.eq(&self.prefix) ||
            standard.eq(&self.end_point_url) {
            return false;
        }
        true
    }
}