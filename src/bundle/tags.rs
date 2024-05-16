use apache_avro::{from_avro_datum, from_value, Schema};
use base64::prelude::BASE64_URL_SAFE_NO_PAD as base64;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("avro error: {0}")]
    AvroError(#[from] apache_avro::Error),
    #[error("base64 error: {0}")]
    Base64Error(#[from] base64::DecodeError),
    #[error("utf8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Base64(pub String);
impl Base64 {
    pub fn decode(&self) -> Result<String, Error> {
        Ok(String::from_utf8(base64.decode(&self.0.as_bytes())?)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Tag<T> {
    pub name: T,
    pub value: T,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Base64Tags(pub Vec<Tag<Base64>>);

impl Base64Tags {
    pub fn from_avro(schema: &Schema, value: Vec<u8>) -> Result<Self, Error> {
        let mut b = value.as_slice();
        let value = from_avro_datum(&schema, &mut b, None)?;
        Ok(from_value::<Base64Tags>(&value)?)
    }
}

impl Display for Base64Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for tag in &self.0 {
            write!(
                f,
                "{:?}={:?};",
                tag.name.decode().unwrap(),
                tag.value.decode().unwrap()
            )?;
        }
        Ok(())
    }
}
