use crate::client::network::Network;
use crate::client::tx::BundleTx;
use std::time::Duration;

pub const DEFAULT_BASE_URL: &str = "https://arweave.net";
pub const DEFAULT_TIMEOUT_MS: u64 = 5000;

mod base64;
mod currency;
mod network;
mod tx;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("url error: {0}")]
    UrlError(#[from] url::ParseError),
}

pub struct Client {
    client: reqwest::Client,
    base_url: url::Url,
}

impl Client {
    pub fn new(base_url: String, timeout: u64) -> Self {
        let client =
            reqwest::Client::builder().timeout(Duration::from_millis(timeout)).build().unwrap();
        let base_url = url::Url::parse(base_url.as_str()).unwrap();

        Self {
            client,
            base_url,
        }
    }

    pub async fn get_network_info(&self) -> Result<Network, Error> {
        let url = self.base_url.join("/info")?;
        self.client
            .get(url)
            .send()
            .await
            .map_err(Error::from)?
            .json::<Network>()
            .await
            .map_err(Error::from)
    }

    pub async fn get_bundle_tx(&self, id: String) -> Result<BundleTx, Error> {
        let url = self.base_url.join(format!("/tx/{}", id).as_str())?;
        self.client
            .get(url)
            .send()
            .await
            .map_err(Error::from)?
            .json::<BundleTx>()
            .await
            .map_err(Error::from)
    }
}
