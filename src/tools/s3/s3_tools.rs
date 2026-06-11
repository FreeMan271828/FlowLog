use std::{fs, io, path::{PathBuf}, sync::{Arc, OnceLock, RwLock}};

use crate::{Configurable, tools::{ConfigTrait, s3::s3_config::S3Config}};
use aws_config::Region;
use aws_sdk_s3::{Client as Client, config::Credentials, primitives::ByteStream};

static INSTANCE: OnceLock<Arc<RwLock<S3Client>>> = OnceLock::new();

pub struct S3Client {
    pub(crate) client: Client,
    rt: tokio::runtime::Runtime,
}

impl Configurable for S3Client {
    
    fn new() -> Arc<std::sync::RwLock<Self>> where Self: Sized {
        INSTANCE.get_or_init(||{
            // 阻塞当前线程，保证创建client
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let client = rt.block_on(Self::create_s3_client());
            let s3_client = S3Client{client, rt};
            Arc::new(RwLock::new(s3_client))
        }).clone()
    }

    fn reload() {
        if let Some(instance) = INSTANCE.get(){
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let new_client = rt.block_on(Self::create_s3_client());
            let new_s3_client = S3Client{client: new_client, rt};
            if let Ok(mut client) = instance.write() {
                *client = new_s3_client;
                println!("The s3 config has updated");
            }
        }
    }
}

impl S3Client { 

    /// TODO! 目前需要读取文件内容到内存中，大文件容易卡死，需要优化，解决文件上传的校验错误问题
    /// 上传文件到配置选定的bucket和prefix文件夹
    pub fn put_file(&self, source_path: &str) -> Result<(), io::Error> {
        let config = S3Config::load().unwrap_or_else(|_| S3Config::default());
        let source_file_buf = PathBuf::from(source_path);
        
        if !source_file_buf.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Cannot find the file"));
        }
        if !source_file_buf.is_file() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("路径不是文件: {}", source_path)));
        }
        
        let file_name = source_file_buf
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "无法获取文件名"))?;
        
        let bucket = config.bucket.clone().into_owned();
        let key = if config.prefix.ends_with('/') {
            format!("{}{}", config.prefix, file_name)
        } else {
            format!("{}/{}", config.prefix, file_name)
        };
        
        let client = self.client.clone();
        let source_path_owned = source_path.to_string();

        println!("Start putting file");
        let result = self.rt.block_on(async move {
            let content = fs::read(&source_path_owned).unwrap();
            let body = ByteStream::from(content);

            client.put_object()
                .bucket(bucket.clone())
                .key(key.clone())

                .body(body)
                .send()
                .await
                .map_err(|e| {
                    eprintln!("S3 put_object failed - bucket: {}, key: {}, file: {}, error: {:?}", 
                        bucket, key, source_path_owned, e);
                    e
                })
        });
        println!("Putting file Over");
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
        }
    }


    /// 上传内容到指定文件，指定在prefix文件夹，target_path是指定写入的文件路径
    pub fn put_data(&self, target_path: &str, content: &[u8]) -> Result<(), std::io::Error> {
        let config = S3Config::load().unwrap_or_else(|_| {
            S3Config::default()
        });
        if !config.is_valid(){
            return Err(io::Error::new(io::ErrorKind::InvalidData, "The S3 config is err"));
        }
        if target_path.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput,"目标路径不能为空"));
         }
        let bucket = config.bucket.clone().into_owned();
        let key = if config.prefix.ends_with("/"){
            format!("{}{}", config.prefix, target_path)
        }
        else{
            format!("{}/{}", config.prefix, target_path)
        };
        let client = self.client.clone();
        let body = ByteStream::from(bytes::Bytes::copy_from_slice(content));
        println!("Start putting data");
        let result = self.rt.block_on(async move {
            client.put_object()
                .bucket(bucket)
                .key(key)
                .body(body)
                .content_type("text/plain")
                .send()
                .await
        });
        println!("Putting data over");
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err.to_string())),
        }
    }

    /// 列出当前指定存储桶和文件前缀下的所有文件
    pub fn list_files(&self) -> Result<Vec<String>, std::io::Error> {
        let config = S3Config::load().unwrap_or_else(|_| {
            S3Config::default()
        });
        if !config.is_valid(){
            return Err(io::Error::new(io::ErrorKind::InvalidData, "The S3 config is err"));
        }
        let bucket = config.bucket.clone().into_owned();
        // 拼装完整的前缀路径、
        let prefix = if config.prefix.ends_with("/"){
            format!("{}", config.prefix)
        }
        else {
            format!("{}/", config.prefix)
        };
        let client = self.client.clone();
        let result = self.rt.block_on(async move {
            client.list_objects_v2()
                .bucket(bucket)
                .prefix(prefix.clone()) // 过滤指定路径
                .send()
                .await
        });

        match result {
            Ok(output) => {
                let mut files = Vec::new();
                if let Some(contents) = output.contents {
                    for object in contents {
                        if let Some(key) = object.key {
                            if !key.ends_with('/') {
                                files.push(key);
                            }
                        }
                    }
                }
                Ok(files)
            }
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err.to_string())),
        }
    }

    /// 异步创建S3客户端
    async fn create_s3_client() -> Client{
        let config = S3Config::load().unwrap_or_else(|_| {
            S3Config::default()
        });
        let credentials = Credentials::new(
            config.access_key.clone().into_owned(),
            config.secret_key.clone().into_owned(),
            None,
            None,
            "freeman"
        );

        let mut config_loader = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .credentials_provider(credentials)
            .region(Region::new(config.region.clone().into_owned()));

        // 如果配置了自定义 Endpoint（如 MinIO, Ceph, LocalStack）
        if !config.end_point_url.is_empty() {
            config_loader = config_loader.endpoint_url(config.end_point_url.clone().into_owned());
        }

        let aws_config = config_loader.load().await;

        let s3_config_builder = aws_sdk_s3::config::Builder::from(&aws_config)
            .force_path_style(config.force_path_style);

        aws_sdk_s3::Client::from_conf(s3_config_builder.build())
    }   
}

#[test]
fn test_s3_client_update_file(){
    let binding = S3Client::new();
    let s3_sink = binding.read().expect("Err while building s3");
    let _ = s3_sink.put_file("README.md").expect("Some thing err");
}

#[test]
fn test_s3_client_write(){
    let binding = S3Client::new();
    let s3_sink = binding.read().expect("Err while building s3");
    let _ = s3_sink.put_data("test.log", "test".as_bytes());
    let _ = s3_sink.list_files();
}

#[test]
fn test_s3_client_read(){
    let binding = S3Client::new();
    let s3_sink = binding.read().expect("Err while building s3");
    let ret = s3_sink.list_files();
    println!("{:?}", ret.unwrap());
}