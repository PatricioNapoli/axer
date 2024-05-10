use crate::bundle::tags::Base64Tags;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BundleTx {
    pub format: u8,
    pub id: String,
    pub last_tx: String,
    pub owner: String,
    pub tags: Base64Tags,
    pub target: String,
    pub quantity: String,
    pub data: String,
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
