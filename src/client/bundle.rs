use crate::client::tags::Base64Tags;
use crate::utils;
use serde::{de, Deserialize, Deserializer};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("total binary length must be greater than 32")]
    LengthError,
    #[error("binary length for entry ids does not match the size declared")]
    LengthMismatch,
    #[error("failed to parse byte array")]
    BytesError(#[from] utils::Error),
}

#[derive(Default)]
pub struct Bundle {
    items: Vec<BundleItem>,
}

impl Bundle {
    pub fn new(data: &[u8]) -> Result<Self, Error> {
        if data.len() < 32 {
            return Err(Error::LengthError);
        }

        // Even though we try to read 32 bytes into a u64, the function only supports 8 bytes
        // and overflow is checked. In the future, this should support a u256, as the ANS-104
        // allows it.
        let items: u64 = utils::byte_array_to_u64(&data[0..32])?;

        if data.len() < (32 + items * 64) as usize {
            return Err(Error::LengthMismatch);
        }

        Ok(Self {
            items: vec![BundleItem::new(data)],
        })
    }
}

impl<'de> Deserialize<'de> for Bundle {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Vis;
        impl de::Visitor<'_> for Vis {
            type Value = Bundle;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a bundle object with DataItems array")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(Bundle::default())
            }
        }
        deserializer.deserialize_str(Vis)
    }
}

#[derive(Deserialize, Default)]
pub struct BundleItem {
    id: String,
    owner: String,
    target: String,
    anchor: String,
    tags: Base64Tags,
    data: String,
    signature: String,
    signature_type: u8,
}

impl BundleItem {
    pub fn new(data: &[u8]) -> Self {
        Self {
            id: Default::default(),
            owner: Default::default(),
            target: Default::default(),
            anchor: Default::default(),
            tags: Base64Tags(vec![]),
            data: Default::default(),
            signature: Default::default(),
            signature_type: 0,
        }
    }
}
