mod client;

use crate::client::{Client, DEFAULT_BASE_URL, DEFAULT_TIMEOUT_MS};
use argh::FromArgs;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[derive(FromArgs, Debug)]
/// Axer CLI args.
pub struct Args {
    /// network base url
    #[argh(option, default = "default_base_url()")]
    pub url: String,

    /// network timeout in ms
    #[argh(option, default = "default_timeout_ms()")]
    pub timeout: u64,
}

fn default_base_url() -> String {
    DEFAULT_BASE_URL.to_string()
}

fn default_timeout_ms() -> u64 {
    DEFAULT_TIMEOUT_MS
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Error> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .init();

    let args: Args = argh::from_env();
    info!("running with {args:?}");

    let client: Client = Client::new(args.url, args.timeout);
    let res = client.get("/").await.unwrap();
    let txt = res.text().await.unwrap();
    info!("response: {txt}");

    Ok(())
}