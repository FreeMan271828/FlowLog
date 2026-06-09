use std::sync::{Arc, OnceLock, RwLock};
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::primitives::ByteStream;
use crate::{Configurable, LogHandler, config::{ConfigTrait, s3_config::S3Config}};

/// S3存储采用双磁盘异步写入
/// 在当前文件夹下使用隐形文件.tmp_log_a，持续写入，如果文件大小超过阈值，那么就创建.tmp_log_b (a和b都是有时间顺序的随机数)，然后把之前的文件后台上传到S3，并删除
/// 上传和删除旧文件的过程采用后台线程执行，受锁保护，不会受到配置文件更新的影响
/// 若写入的时候终止，则恢复后继续写入

static INSTANCE: OnceLock<Arc<RwLock<S3Sink>>> = OnceLock::new();

pub struct S3Sink{
    config: S3Config,
    s3_client : aws_sdk_s3::Client,
    rt: tokio::runtime::Runtime,
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
            println!("Something err while handling S3, you can choose to remove S3 option");
            return Ok(());
        }
        
        Ok(())
    }
}

impl S3Sink {
    fn create_and_write(&self, relative_path: &str, content: &[u8]) -> Result<(), std::io::Error> {
        let bucket = self.config.bucket.clone().into_owned();
        let key = format!("{}{}", self.config.prefix, relative_path);
        let client = self.s3_client.clone();

        // 构建字节流
        let body = ByteStream::from(bytes::Bytes::copy_from_slice(content));

        let result = self.rt.block_on(async move {
            client.put_object()
                .bucket(bucket)
                .key(key)
                .body(body)
                .content_type("text/plain")
                .send()
                .await
        });

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err.to_string())),
        }
    }

    fn list_files(&self, sub_path: &str) -> Result<Vec<String>, std::io::Error> {
        let bucket = self.config.bucket.clone().into_owned();
        // 拼装完整的前缀路径
        let prefix = format!("{}{}", self.config.prefix, sub_path);
        let client = self.s3_client.clone();

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
                            // 排除路径本身（S3中以 / 结尾的空对象通常代表文件夹）
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

    fn create_s3_sink() -> S3Sink{
        let config = S3Config::load().unwrap_or_else(|_| {
            S3Config::default()
        });
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        // 2. 异步初始化 AWS 客户端并阻塞等待完成
        let s3_client = rt.block_on(async {
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

            // 创建 S3 专属配置以支持 force_path_style
            let s3_config_builder = aws_sdk_s3::config::Builder::from(&aws_config)
                .force_path_style(config.force_path_style);

            aws_sdk_s3::Client::from_conf(s3_config_builder.build())
        });

        S3Sink { config, s3_client, rt }
    }
}