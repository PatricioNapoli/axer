use crate::client::bundle::Bundle;
use crate::client::tags::Base64Tags;
use serde::Deserialize;
use std::fmt::Display;

/// A bundle transaction.
///
/// Quantity is missing from this struct to avoid Currency parsing.
#[derive(Deserialize)]
pub struct BundleTx {
    pub format: u8,
    pub id: String,
    pub last_tx: String,
    pub owner: String,
    pub tags: Base64Tags,
    pub target: String,
    pub data: Bundle,
    pub data_root: String,
    pub data_size: String,
    pub reward: String,
    pub signature: String,
}

impl Display for BundleTx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Bundle Transaction {{ id: {}, last_tx: {}, tags: {}, data_size: {}}}",
            self.id, self.last_tx, self.tags, self.data_size
        )
    }
}
