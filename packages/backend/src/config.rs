#[derive(Clone)]
pub struct Config {
    pub s3_bucket: String,
    pub aws_endpoint_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            s3_bucket: std::env::var("S3_BUCKET")?,
            aws_endpoint_url: std::env::var("AWS_ENDPOINT_URL")?,
        })
    }
}
