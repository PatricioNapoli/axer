mod bundle;
mod cache;
mod cli;
mod client;
mod utils;

use crate::cli::Cli;
use tracing::error;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cli error: {0}")]
    CliError(#[from] cli::Error),
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Error> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .init();

    let mut cli = Cli::from_env_args();

    match cli.run().await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("failure: {}", e);
            Err(Error::CliError(e))
        }
    }
}
