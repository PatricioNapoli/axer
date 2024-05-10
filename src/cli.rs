use crate::cache::Cache;
use crate::{client};
use crate::client::bundle::Bundle;
use crate::client::tx::BundleTx;
use crate::client::{Client, DEFAULT_BASE_URL, DEFAULT_TIMEOUT_MS};
use argh::FromArgs;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use tokio::task::JoinSet;
use tracing::{error, info, warn};
use crate::utils::file;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("client error: {0}")]
    ClientError(#[from] client::Error),
    #[error("args error")]
    ArgsError,
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(FromArgs, Debug, Clone)]
/// Axer CLI - Arweave bundle explorer.
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
    "cache.json".to_string()
}

fn default_out_dir() -> String {
    "out/".to_string()
}

pub struct Cli {
    client: Client,
    args: Args,
    cache: Cache<BundleTx>,
}

impl Cli {
    pub fn from_env_args() -> Self {
        let args: Args = argh::from_env();

        let a = args.clone();
        let client = Client::new(a.url, a.timeout);

        Self {
            client,
            args,
            cache: Cache::from_file(a.db_file),
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        info!("running with {:?}", self.args);

        let info = self.client.get_network_info().await?;
        info!("connected to: {info}");

        if let Some(batch_file) = self.args.batch_file.clone() {
            self.handle_batch_mode(batch_file).await?;
        } else {
            match self.args.interactive {
                true => self.handle_interactive().await?,
                false => self.handle_single().await?,
            };
        }

        Ok(())
    }

    async fn handle_batch_mode(&mut self, batch_file: String) -> Result<(), Error> {
        let file = std::fs::File::open(batch_file.as_str())?;
        let reader = std::io::BufReader::new(file).lines();

        let mut set: JoinSet<Result<(BundleTx, Bundle), Error>> = JoinSet::new();

        info!("running batch mode using file: {batch_file}");

        for line in reader.flatten() {
            async fn fetch_bundle_data(
                client: Client,
                tx: BundleTx,
            ) -> Result<(BundleTx, Bundle), Error> {
                let bundle = client.get_bundle_data(&tx).await?;
                Ok((tx, bundle))
            }

            async fn fetch_bundle(
                client: Client,
                tx_id: String,
            ) -> Result<(BundleTx, Bundle), Error> {
                let (bundle_tx, bundle) = client.get_bundle(tx_id.as_str()).await?;
                Ok((bundle_tx, bundle))
            }

            let tx_id = line.trim().to_string();

            if !self.cache.data.contains_key(&tx_id) {
                set.spawn(fetch_bundle(self.client.clone(), tx_id.clone()));
                continue;
            }

            let tx = self.cache.data.get(&tx_id).unwrap();
            info!("transaction {} was found in cache", tx_id);

            let path = self.get_bundle_path(&tx_id);
            if !path.exists() {
                warn!("bundle file not found, fetching: {}", tx_id);

                set.spawn(fetch_bundle_data(self.client.clone(), tx.clone()));
            }
        }

        while let Some(res) = set.join_next().await {
            match res {
                Ok(r) => {
                    let (tx, bundle) = r?;
                    info!("transaction: {}", tx);

                    self.cache.data.insert(tx.id.clone(), tx.clone());
                    file::save_serde_json(self.get_bundle_path(&tx.id), &bundle.items)?;
                }
                Err(e) => {
                    error!("batch task join failed: {e}");
                }
            }
        }

        Ok(())
    }

    async fn handle_single(&mut self) -> Result<(), Error> {
        match &self.args.tx_id.clone() {
            Some(tx_id) => {
                info!("running single mode for transaction: {tx_id}");

                self.get_or_fetch_bundle(tx_id).await?;
                Ok(())
            }
            None => {
                error!("transaction id required -- either use -i flag or --tx-id <id>");
                Err(Error::ArgsError)
            }
        }
    }

    async fn handle_interactive(&mut self) -> Result<(), Error> {
        println!("Enter an Arweave bundle transaction id or 'q' to quit");

        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "q" {
                return Ok(());
            }

            let tx_id = input.trim();
            self.get_or_fetch_bundle(&tx_id.to_string()).await?;
        }
    }

    async fn get_or_fetch_bundle(&mut self, tx_id: &String) -> Result<(), Error> {
        if self.cache.data.contains_key(tx_id) {
            let tx = self.cache.data.get(tx_id).unwrap();
            info!("transaction from cache: {}", tx);

            // If for some reason the bundle data file does not exist, fetch it
            let path = self.get_bundle_path(tx_id);
            if !path.exists() {
                warn!("bundle file not found, fetching: {}", tx_id);

                let bundle = self.client.get_bundle_data(tx).await?;
                file::save_serde_json(self.get_bundle_path(tx_id), &bundle.items)?;
            }
            return Ok(());
        }

        let (tx, bundle) = self.client.get_bundle(tx_id.as_str()).await?;
        info!("transaction: {}", tx);

        self.cache.data.insert(tx_id.clone(), tx.clone());
        file::save_serde_json(self.get_bundle_path(tx_id), &bundle.items)?;
        Ok(())
    }

    fn get_bundle_path(&self, tx_id: &String) -> PathBuf {
        Path::new(&self.args.out_dir).join(format!("{}.json", tx_id))
    }
}
