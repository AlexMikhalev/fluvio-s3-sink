use anyhow::Result;
use async_trait::async_trait;

use fluvio::consumer::Record;
use fluvio::Offset;
use fluvio_connector_common::{
    tracing::{debug, error, info},
    LocalBoxSink, Sink,
};

use crate::config::S3Config;

use opendal::services::S3;
use opendal::Operator;
use opendal::layers::LoggingLayer;


#[derive(Debug)]
pub(crate) struct S3Sink {
    pub(crate) region: String,
    pub(crate) bucket: String,
    pub(crate) access_key_id: String,
    pub(crate) secret_access_key: String,
}

impl S3Sink {
    pub(crate) fn new(config: &S3Config) -> Result<Self> {
        let region = &config.region.clone();
        let bucket = &config.bucket.clone();
        let access_key_id = &config.access_key_id.resolve()?;
        let secret_access_key = &config.secret_access_key.resolve()?;

        Ok(Self {
            region: region.clone().unwrap_or("us-east-1".to_string()),
            bucket: bucket.clone(),
            access_key_id: access_key_id.to_string(),
            secret_access_key: secret_access_key.to_string()})
    }
}

#[async_trait]
impl Sink<Record> for S3Sink {
    async fn connect(self, _offset: Option<Offset>) -> Result<LocalBoxSink<Record>> {
        info!("Connecting to S3");
        let mut builder = S3::default();
        builder.region(&self.region);
        builder.bucket(&self.bucket);
        builder.access_key_id(&self.access_key_id);
        builder.secret_access_key(&self.secret_access_key);
        // Init an operator
        let op = Operator::new(builder)?
        // Init with logging layer enabled.
        .layer(LoggingLayer::default())
        .finish();
        let unfold = futures::sink::unfold(op, |mut op, record: Record| async move {
            let key = if let Some(key) = record.key() {
                String::from_utf8_lossy(key).to_string()
            }else{
                record.timestamp().to_string()
            };
            op.write(&key, record.value()).await?;
            Ok::<_, anyhow::Error>(op)
        });
        Ok(Box::pin(unfold))
    }
}