use crate::client::bundle::Bundle;
use crate::client::network::Network;
use crate::client::tx::BundleTx;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::time::Duration;

pub const DEFAULT_BASE_URL: &str = "https://arweave.net";
pub const DEFAULT_TIMEOUT_MS: u64 = 5000;

mod bundle;
mod network;
mod tags;
mod tx;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("status error: {status} {message}")]
    StatusError {
        status: StatusCode,
        message: String,
    },
    #[error("url error: {0}")]
    UrlError(#[from] url::ParseError),
    #[error("bundle error: {0}")]
    BundleError(#[from] bundle::Error),
}

pub struct Client {
    client: reqwest::Client,
    base_url: url::Url,
    cache: HashMap<String, BundleTx>,
}

impl Client {
    pub fn new(base_url: String, timeout: u64, db_file: String) -> Self {
        let client =
            reqwest::Client::builder().timeout(Duration::from_millis(timeout)).build().unwrap();
        let base_url = url::Url::parse(base_url.as_str()).unwrap();

        Self {
            client,
            base_url,
            cache: Default::default(),
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

    pub async fn get_bundle_tx(&mut self, id: &str) -> Result<&BundleTx, Error> {
        if !self.cache.contains_key(id) {
            let url = self.base_url.join(format!("/tx/{}", id).as_str())?;
            let response = self.client.get(url).send().await.map_err(Error::from)?;

            return match response.status() {
                StatusCode::OK => {
                    let mut tx = response.json::<BundleTx>().await.map_err(Error::from)?;
                    let tx_bundle_data = self.get_bundle_data(&tx).await?;
                    tx.data = tx_bundle_data;

                    self.cache.insert(id.to_string(), tx);
                    Ok(self.cache.get(id).unwrap())
                }
                status => Err(Error::StatusError {
                    status,
                    message: response.text().await.unwrap(),
                }),
            };
        }

        Ok(self.cache.get(id).unwrap())
    }

    async fn get_bundle_data(&self, tx: &BundleTx) -> Result<Bundle, Error> {
        let url = self.base_url.join(format!("{}", tx.id).as_str())?;
        let response = self.client.get(url).send().await.map_err(Error::from)?;

        match response.status() {
            StatusCode::OK => {
                Ok(Bundle::new(response.bytes().await.map_err(Error::from)?.as_ref())?)
            }
            status => Err(Error::StatusError {
                status,
                message: response.text().await.unwrap(),
            }),
        }
    }
}
