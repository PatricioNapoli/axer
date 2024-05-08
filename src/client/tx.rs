use crate::client::base64::Base64;
use crate::client::currency::Currency;
use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct Tag<T> {
    pub name: T,
    pub value: T,
}

#[derive(Debug)]
pub struct U64(pub u64);

impl<'de> Deserialize<'de> for U64 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Vis;
        impl de::Visitor<'_> for Vis {
            type Value = U64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string backed u64 number")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                u64::from_str(v)
                    .map(U64)
                    .map_err(|_| de::Error::custom("failed to parse u64 number"))
            }
        }
        deserializer.deserialize_str(Vis)
    }
}

#[derive(Deserialize, Debug)]
pub struct BundleTx {
    pub format: u8,
    pub id: Base64,
    pub last_tx: Base64,
    pub owner: Base64,
    pub tags: Vec<Tag<Base64>>,
    pub target: Base64,
    pub quantity: Currency,
    pub data_root: Base64,
    pub data: Base64,
    pub data_size: U64,
    pub reward: U64,
    pub signature: Base64,
}
