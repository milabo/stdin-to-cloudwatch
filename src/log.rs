use aws_config::{Region, SdkConfig};
use aws_sdk_cloudwatchlogs::types::builders::InputLogEventBuilder;
use aws_sdk_cloudwatchlogs::Client;

#[derive(Debug)]
pub struct LogsClient {
    client: Client,
    group_name: String,
    stream_name: String,
}

impl LogsClient {
    pub async fn new(
        region: &Option<String>,
        endpoint_url: &Option<String>,
        group_name: &str,
        stream_name: &str,
    ) -> Self {
        let config = get_config(region, endpoint_url).await;
        Self {
            client: get_client(&config).await,
            group_name: group_name.to_owned(),
            stream_name: stream_name.to_owned(),
        }
    }

    pub async fn initialize_log_stream(&self, remake_log_stream: bool) -> anyhow::Result<()> {
        initialize_log_stream(
            &self.client,
            &self.group_name,
            &self.stream_name,
            remake_log_stream,
        )
        .await
    }

    pub async fn put(&self, timestamp: i64, message: &str) -> anyhow::Result<()> {
        put_log(
            &self.client,
            &self.group_name,
            &self.stream_name,
            timestamp,
            message,
        )
        .await
    }
}

async fn get_client(config: &SdkConfig) -> Client {
    let builder = aws_sdk_cloudwatchlogs::config::Builder::from(config);
    Client::from_conf(builder.build())
}

async fn get_config(region: &Option<String>, endpoint_url: &Option<String>) -> SdkConfig {
    let mut config = aws_config::from_env();
    if let Some(region) = region {
        config = config.region(Region::new(region.to_owned()))
    }
    if let Some(endpoint_url) = endpoint_url {
        config = config.endpoint_url(endpoint_url)
    }
    config.load().await
}

async fn initialize_log_stream(
    client: &Client,
    group_name: &str,
    stream_name: &str,
    remake_stream: bool,
) -> anyhow::Result<()> {
    if log_group_exists(client, group_name).await? {
        create_log_group(client, group_name).await?;
        eprintln!("log group created")
    } else {
        eprintln!("log group already exists")
    }

    if log_stream_exists(client, group_name, stream_name).await? {
        eprintln!("log stream already exists");
        if remake_stream {
            delete_log_stream(client, group_name, stream_name).await?;
            eprintln!("log stream deleted");
            create_log_stream(client, group_name, stream_name).await?;
            eprintln!("log stream created");
        }
    } else {
        create_log_stream(client, group_name, stream_name).await?;
        eprintln!("log stream created");
    }
    Ok(())
}

async fn log_group_exists(client: &Client, group_name: &str) -> anyhow::Result<bool> {
    let res = client
        .describe_log_groups()
        .log_group_name_pattern(group_name)
        .send()
        .await?;
    let exists = res.log_groups.filter(|x| !x.is_empty()).is_none();
    Ok(exists)
}

async fn create_log_group(client: &Client, group_name: &str) -> anyhow::Result<()> {
    client
        .create_log_group()
        .log_group_name(group_name)
        .send()
        .await?;
    Ok(())
}

async fn log_stream_exists(
    client: &Client,
    group_name: &str,
    stream_name: &str,
) -> anyhow::Result<bool> {
    let res = client
        .describe_log_streams()
        .log_group_name(group_name)
        .log_stream_name_prefix(stream_name)
        .send()
        .await?;
    if let Some(streams) = res.log_streams {
        let exists = streams
            .iter()
            .filter(|s| {
                s.log_stream_name
                    .as_deref()
                    .filter(|name| *name == stream_name)
                    .is_some()
            })
            .next()
            .is_some();
        Ok(exists)
    } else {
        Ok(false)
    }
}

async fn create_log_stream(
    client: &Client,
    group_name: &str,
    stream_name: &str,
) -> anyhow::Result<()> {
    client
        .create_log_stream()
        .log_group_name(group_name)
        .log_stream_name(stream_name)
        .send()
        .await?;
    Ok(())
}

async fn delete_log_stream(
    client: &Client,
    group_name: &str,
    stream_name: &str,
) -> anyhow::Result<()> {
    client
        .delete_log_stream()
        .log_group_name(group_name)
        .log_stream_name(stream_name)
        .send()
        .await?;
    Ok(())
}

async fn put_log(
    client: &Client,
    group_name: &str,
    stream_name: &str,
    timestamp: i64,
    message: &str,
) -> anyhow::Result<()> {
    client
        .put_log_events()
        .log_group_name(group_name)
        .log_stream_name(stream_name)
        .log_events(
            InputLogEventBuilder::default()
                .timestamp(timestamp)
                .message(message.to_owned())
                .build()?,
        )
        .send()
        .await?;
    Ok(())
}
