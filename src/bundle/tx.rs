use crate::bundle::tags::Base64Tags;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use crate::bundle::tags;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid bundle format, expected {expected} but found '{found}'")]
    InvalidBundleFormat {
        expected: String,
        found: String,
    },

    #[error("invalid bundle version, expected {expected} but found '{found}'")]
    InvalidBundleVersion {
        expected: String,
        found: String,
    },

    #[error("tags error: {0}")]
    TagsError(#[from] tags::Error),
}

const BUNDLE_FORMAT_SUPPORTED: &str = "binary";
const BUNDLE_VERSION_SUPPORTED: &str = "2.0.0";

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

impl BundleTx {
    pub fn is_valid(&self) -> Result<(), Error> {
        let mut format = "".to_string();
        let mut version = "".to_string();

        self.tags.0.iter().for_each(|tag| {
            let name = tag.name.decode().unwrap();
            let value = tag.value.decode().unwrap();

            if name == "Bundle-Format" {
                format = value;
            } else if name == "Bundle-Version" {
                version = value;
            }
        });

        if format != BUNDLE_FORMAT_SUPPORTED {
            return Err(Error::InvalidBundleFormat {
                expected: BUNDLE_FORMAT_SUPPORTED.to_string(),
                found: format.clone(),
            });
        }

        if version != BUNDLE_VERSION_SUPPORTED {
            return Err(Error::InvalidBundleVersion {
                expected: BUNDLE_VERSION_SUPPORTED.to_string(),
                found: version.clone(),
            });
        }

        Ok(())
    }
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
