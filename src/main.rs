mod config;
use config::S3Config;

mod sink;
use sink::S3Sink;

use fluvio_connector_common::{connector, consumer::ConsumerStream, Result, Sink};
use futures::SinkExt;

#[connector(sink)]
async fn start(config: S3Config, mut stream: impl ConsumerStream) -> Result<()> {
    println!("Starting s3-sink sink connector with {config:?}");
    let sink = S3Sink::new(&config)?;
    let mut sink = sink.connect(None).await?;
    while let Some(item) = stream.next().await {
        let record = item?;
        sink.send(record).await?;
    }
    Ok(())
}
