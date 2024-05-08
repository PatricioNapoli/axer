use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::prelude::*;
use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

pub struct Base64(pub Vec<u8>);

impl std::fmt::Debug for Base64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = &base64::display::Base64Display::new(&self.0, &URL_SAFE_NO_PAD);
        write!(f, "{}", string)
    }
}

impl FromStr for Base64 {
    type Err = base64::DecodeError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let result = BASE64_URL_SAFE_NO_PAD.decode(str)?;
        Ok(Self(result))
    }
}

impl<'de> Deserialize<'de> for Base64 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Vis;
        impl de::Visitor<'_> for Vis {
            type Value = Base64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a base64 string")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                BASE64_URL_SAFE_NO_PAD
                    .decode(v)
                    .map(Base64)
                    .map_err(|_| de::Error::custom("failed to decode base64 string"))
            }
        }
        deserializer.deserialize_str(Vis)
    }
}
