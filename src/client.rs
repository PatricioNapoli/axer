use std::time::Duration;

pub const DEFAULT_BASE_URL: &str = "https://arweave.net";
pub const DEFAULT_TIMEOUT_MS: u64 = 5000;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),
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

    pub async fn get(&self, path: &str) -> Result<reqwest::Response, Error> {
        let url = self.base_url.join(path).unwrap();
        self.client.get(url).send().await.map_err(Error::from)
    }
}
