#[derive(Clone)]
pub struct Config {
    pub s3_bucket: String,
    pub s3_region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            s3_bucket: std::env::var("S3_BUCKET")?,
            s3_region: std::env::var("S3_REGION")?,
            access_key_id: std::env::var("ACCESS_KEY_ID")?,
            secret_access_key: std::env::var("SECRET_ACCESS_KEY")?,
        })
    }
}
