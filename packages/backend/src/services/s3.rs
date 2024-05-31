use aws_sdk_s3::{Client, Error};
use std::sync::Arc;
use aws_config::BehaviorVersion;

pub struct S3Service {
    client: Arc<Client>,
    bucket: String,
}

impl S3Service {
    pub async fn new(config: crate::config::Config) -> Self {
        let shared_config = aws_config::defaults(BehaviorVersion::latest())
            .endpoint_url(config.s3_endpoint_url)
            .region(config.s3_region)
            .access_key_id(config.access_key_id)
            .secret_access_key(config.secret_access_key)
            .load()
            .await;


        S3Service {
            client: create_s3_client(&shared_config),
            bucket: config.s3_bucket,
        }
    }

    pub async fn upload_file(&self, key: &str, content: Vec<u8>) -> Result<(), Error> {
        Ok(())
    }
}

fn create_s3_client(conf: &aws_config::SdkConfig) -> Client {
    let s3_config_builder = aws_sdk_s3::config::Builder::from(conf);
    Client::from_conf(s3_config_builder.build())
}
