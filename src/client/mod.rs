use crate::client::bundle::Bundle;
use crate::client::network::Network;
use crate::client::tx::BundleTx;
use apache_avro::Schema;
use reqwest::StatusCode;
use std::time::Duration;

pub const DEFAULT_BASE_URL: &str = "https://arweave.net";
pub const DEFAULT_TIMEOUT_MS: u64 = 5000;

mod network;
mod tags;

pub mod bundle;
mod signatures;
pub mod tx;

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

pub const TAGS_AVRO_SCHEMA: &str = r#"
{
	"type": "array",
	"items": {
		"type": "record",
		"name": "Tag",
		"fields": [
			{ "name": "name", "type": "bytes" },
			{ "name": "value", "type": "bytes" }
		]
	}
}"#;

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    base_url: url::Url,
    tags_schema: Schema,
}

impl Client {
    pub fn new(base_url: String, timeout: u64) -> Self {
        let client =
            reqwest::Client::builder().timeout(Duration::from_millis(timeout)).build().unwrap();
        let base_url = url::Url::parse(base_url.as_str()).unwrap();

        Self {
            client,
            base_url,
            tags_schema: Schema::parse_str(TAGS_AVRO_SCHEMA).unwrap(),
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

    pub async fn get_bundle(&self, id: &str) -> Result<(BundleTx, Bundle), Error> {
        let url = self.base_url.join(format!("/tx/{}", id).as_str())?;
        let response = self.client.get(url).send().await.map_err(Error::from)?;

        return match response.status() {
            StatusCode::OK => {
                let tx = response.json::<BundleTx>().await.map_err(Error::from)?;
                let bundle = self.get_bundle_data(&tx).await?;

                Ok((tx, bundle))
            }
            status => Err(Error::StatusError {
                status,
                message: response.text().await.unwrap(),
            }),
        };
    }

    pub async fn get_bundle_data(&self, tx: &BundleTx) -> Result<Bundle, Error> {
        let url = self.base_url.join(format!("{}", tx.id).as_str())?;
        let response = self.client.get(url).send().await.map_err(Error::from)?;

        match response.status() {
            StatusCode::OK => Ok(Bundle::new(
                response.bytes().await.map_err(Error::from)?.as_ref(),
                &self.tags_schema,
            )?),
            status => Err(Error::StatusError {
                status,
                message: response.text().await.unwrap(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_BUNDLE_TX: &str = "aJ3PrkyJ6GpdwwUxxXFHiB40cEg-GPRUWcKUI6wCgPQ";

    #[tokio::test]
    async fn test_get_bundle() {
        let client = Client::new(DEFAULT_BASE_URL.to_string(), DEFAULT_TIMEOUT_MS);
        let (bundle_tx, bundle) =
            client.get_bundle(TEST_BUNDLE_TX).await.unwrap_or_else(|e| panic!("{:?}", e));

        assert_eq!(bundle_tx.format, 2);
        assert_eq!(bundle_tx.id, "aJ3PrkyJ6GpdwwUxxXFHiB40cEg-GPRUWcKUI6wCgPQ");
        assert_eq!(
            bundle_tx.last_tx,
            "2iAqen10b8K0lVB3xmtp8plsd0GZzrc_yAoG8C9Fccz67Zc9U0vsvvP2S3S7tMtN"
        );

        assert_eq!(bundle.items.len(), 122);
        assert_eq!(bundle.items[0].id, "eWABlTtLgOcrcWHWJNRBGSBSRmwN9_Rlm_IetJuir3o");
    }
}
