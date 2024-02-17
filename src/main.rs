mod log;
mod option;

use crate::option::Args;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let client = log::LogsClient::new(
        &args.region,
        &args.endpoint_url,
        &args.log_group_name,
        &args.log_stream_name,
    )
    .await;
    client.initialize_log_stream(args.remake_log_stream).await?;
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.is_empty() {
            break;
        }
        client.put(chrono::Utc::now().timestamp_millis(), &input).await?;
    }
    Ok(())
}
