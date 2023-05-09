# WIP fluvio to RedisJSON connector

The connector dump records (assumed JSON, see fluvio smart modules) from fluvio topics into S3 bucket 

Add secrets `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` make sure the region in config-example.yaml matches bucket region i.e. `region: "eu-west-2"`


## Install Fluvio and Fluvio Connector Development Kit (CDK)
Install fluvio:
```
curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```
following tutorial [here](https://www.fluvio.io/connectors/cdk/overview/) install CDK
```
fluvio install cdk
```

## Build connector 

Build connector:
```
cdk build
```

## Run connector
```
RUST_BACKTRACE=1 cdk test -c config-example.yaml --secrets secrets.txt
```
change REDIS_URL inside secrets.txt

For running in the infinyon cloud set secrets  via `fluvio cloud secret` command
```
fluvio cloud secret set -c AWS_ACCESS_KEY_ID ..
```
to check:
```
fluvio cloud secret list
 Secret Name  Last Modified 
 REDIS_URL    2023-05-01    
```

change topic in config-example.yaml
```
meta:
  version: 0.1.0
  name: my-s3-sink-test-connector
  type: s3-sink-sink
  topic: hackernews
s3:
  region: "eu-west-2"
  bucket: "alex-m-test"
  access_key_id:
    secret:
      name: "AWS_ACCESS_KEY_ID"
  secret_access_key:
    secret:
      name: "AWS_SECRET_ACCESS_KEY"
```
if you want to listen to the topic differently to hackers news 

Follow up [quickstart](https://www.fluvio.io/connectors/cdk/overview/) to build your own connector 
