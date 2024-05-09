mod client;
mod utils;

use crate::client::{Client, DEFAULT_BASE_URL, DEFAULT_TIMEOUT_MS};
use argh::FromArgs;
use std::io::Write;
use tracing::level_filters::LevelFilter;
use tracing::{error, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("currency parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("client error: {0}")]
    ClientError(#[from] client::Error),
    #[error("args error")]
    ArgsError,
}

#[derive(FromArgs, Debug)]
/// Axer CLI args.
pub struct Args {
    /// network base url
    #[argh(option, default = "default_base_url()")]
    pub url: String,

    /// network timeout in ms
    #[argh(option, default = "default_timeout_ms()")]
    pub timeout: u64,

    /// index db filename
    #[argh(option, default = "default_db_filename()")]
    pub db_file: String,

    /// output directory for parsed files
    #[argh(option, default = "default_out_dir()", short = 'o')]
    pub out_dir: String,

    /// arweave bundle transaction ID
    #[argh(option)]
    pub tx_id: Option<String>,

    /// batch filename, enables batch mode
    #[argh(option, short = 'b')]
    pub batch_file: Option<String>,

    /// interactive mode
    #[argh(switch, short = 'i')]
    pub interactive: bool,
}

fn default_base_url() -> String {
    DEFAULT_BASE_URL.to_string()
}

fn default_timeout_ms() -> u64 {
    DEFAULT_TIMEOUT_MS
}

fn default_db_filename() -> String {
    "db.b".to_string()
}

fn default_out_dir() -> String {
    "out/".to_string()
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Error> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .init();

    let args: Args = argh::from_env();
    info!("running with {args:?}");

    let mut client = Client::new(args.url, args.timeout, args.db_file);

    let info = client.get_network_info().await?;
    info!("connected to: {info}");

    match args.interactive {
        true => handle_interactive(&mut client).await,
        false => match args.tx_id {
            Some(_) => {
                let tx = client.get_bundle_tx(&args.tx_id.unwrap()).await?;
                info!("transaction: {tx}");
                Ok(())
            }
            None => {
                error!("transaction id required -- either use -i flag or --tx-id <id>");
                Err(Error::ArgsError)
            }
        },
    }
}

async fn handle_interactive(client: &mut Client) -> Result<(), Error> {
    println!("Enter an Arweave bundle transaction id or q to save db and exit");

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "q" {
            info!("saving db");
            // TODO: save cache
            info!("bye");
            return Ok(());
        }

        let tx_id = input.trim();

        let tx = client.get_bundle_tx(tx_id).await;
        match tx {
            Ok(tx) => {
                info!("transaction: {tx}");
            }
            Err(e) => {
                error!("error: {e:?}");
            }
        }
    }
}
