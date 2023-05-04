mod config;
use config::S3Config;

mod sink;
use sink::S3Sink;

use opendal::services::S3;
use opendal::Operator;
use opendal::layers::LoggingLayer;
// use opendal::Result;

use fluvio_connector_common::{connector, consumer::ConsumerStream, Result};
use fluvio_connector_common::Sink;
use fluvio::consumer::Record;
use futures::SinkExt;

#[connector(sink)]
async fn start(config: S3Config, mut stream: impl ConsumerStream) -> Result<()> {
    println!("Starting s3-sink sink connector with {config:?}");
    let sink = S3Sink::new(&config)?;
    while let Some(item) = stream.next().await {
        let record = item?;
        sink.send(record).await?;
    }
    Ok(())
}

