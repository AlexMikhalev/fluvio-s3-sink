use fluvio_connector_common::{connector, secret::SecretString};

#[connector(config, name="s3")]
#[derive(Debug)]
pub(crate) struct S3Config {
    #[allow(dead_code)]
    pub region: Option<String>,
    pub bucket: String,
    pub access_key_id: SecretString,
    pub secret_access_key: SecretString,

}

