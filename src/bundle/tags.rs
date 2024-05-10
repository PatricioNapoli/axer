use apache_avro::{from_avro_datum, from_value, Schema};
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("avro error: {0}")]
    AvroError(#[from] apache_avro::Error),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Tag<T> {
    pub name: T,
    pub value: T,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Base64Tags(pub Vec<Tag<String>>);

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
                String::from_utf8_lossy(
                    BASE64_STANDARD_NO_PAD.decode(tag.name.as_bytes()).unwrap().as_slice()
                ),
                String::from_utf8_lossy(
                    BASE64_STANDARD_NO_PAD.decode(tag.value.as_bytes()).unwrap().as_slice()
                )
            )?;
        }
        Ok(())
    }
}
