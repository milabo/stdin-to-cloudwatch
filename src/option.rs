#[derive(Debug, clap::Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true
)]
pub struct Args {
    /// The name of log group.
    #[clap(required = true)]
    pub log_group_name: String,
    /// The name of log stream.
    #[clap(required = true)]
    pub log_stream_name: String,
    /// Delete log stream if exists by designated name.
    #[clap(short, long, default_value = "false")]
    pub remake_log_stream: bool,
    /// AWS region.
    #[clap(long)]
    pub region: Option<String>,
    /// AWS endpoint URL.
    /// (for LocalStack: http://localhost:4566)
    #[clap(long)]
    pub endpoint_url: Option<String>,
}
