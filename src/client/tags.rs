use base64::prelude::*;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize)]
pub struct Tag<T> {
    pub name: T,
    pub value: T,
}

#[derive(Deserialize, Default)]
pub struct Base64Tags(pub Vec<Tag<String>>);

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
