use aws_sdk_s3::{Client, types::Object, primitives::ByteStream, presigning::PresigningConfig};
use std::sync::Arc;
use std::time::Duration;
use anyhow::Result;

#[derive(Clone)]
pub struct S3Service {
    client: Arc<Client>,
    bucket: String,
}

const DEFAULT_EXPIRATION: Duration = Duration::from_secs(3600);

impl S3Service {
    pub async fn new(
        config: crate::config::Config,
        aws_shared_config: aws_config::SdkConfig
    ) -> Self {
        S3Service {
            client: Arc::from(create_s3_client(&aws_shared_config)),
            bucket: config.s3_bucket,
        }
    }

    pub async fn upload_file(&self, key: &str, content: Vec<u8>) -> Result<()> {
        let byte_stream = ByteStream::from(content);

        self.client.put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(byte_stream)
            .send()
            .await?;

        Ok(())
    }

    pub async fn list_objects(&self) -> Result<Vec<Object>> {
        let resp = self.client.list_objects_v2()
            .bucket(&self.bucket)
            .send()
            .await?;

        let objects = resp.contents.unwrap_or_default();
        Ok(objects)
    }

    pub async fn get_presigned_url(&self, key: &str) -> Result<String> {
        let presigning_config = PresigningConfig::expires_in(DEFAULT_EXPIRATION)?;

        let presigned_request = self.client.get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(presigning_config)
            .await?;

        Ok(presigned_request.uri().to_string())
    }
}

fn create_s3_client(conf: &aws_config::SdkConfig) -> Client {
    let s3_config_builder = aws_sdk_s3::config::Builder::from(conf);
    Client::from_conf(s3_config_builder.build())
}
