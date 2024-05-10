use crate::client::signatures::get_sig_types;
use crate::client::tags;
use crate::client::tags::Base64Tags;
use crate::utils;
use apache_avro::Schema;
use base64::prelude::BASE64_URL_SAFE_NO_PAD as base64;
use base64::Engine;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("bundle data length is less than 32")]
    BundleLessThanMinimum,
    #[error("bundle data length is less than expected, headers missing or incomplete")]
    BundleHeadersIncomplete,
    #[error("bundle data length is less than expected, item header incomplete")]
    ItemHeaderIncomplete,
    #[error("bundle data length is less than expected, item data incomplete")]
    ItemDataIncomplete,
    #[error("item data length is less than 2")]
    ItemDataLessThanMinimum,
    #[error("signature not supported: {sig_type:?}")]
    SignatureNotSupported {
        sig_type: u64,
    },
    #[error("item data length does not include signature")]
    ItemDataLessThanSignature,
    #[error("failed to parse tags: {0}")]
    TagsParseError(#[from] tags::Error),
    #[error("item id mismatch, expected {expected:?} but found {found:?}")]
    IdMismatch {
        expected: String,
        found: String,
    },
    #[error("failed to parse byte array")]
    BytesError(#[from] utils::Error),
}

#[derive(Serialize, PartialEq)]
pub struct Bundle {
    pub items: Vec<BundleItem>,
}

impl Bundle {
    pub fn new(data: &[u8], tags_schema: &Schema) -> Result<Self, Error> {
        if data.len() < 32 {
            return Err(Error::BundleLessThanMinimum);
        }

        // Even though we try to read 32 bytes into a u64, the function only supports 8 bytes
        // and overflow is checked. In the future, this should support a u256, as the ANS-104
        // allows it.
        let items_len: u64 = utils::byte_array_to_u64(&data[..32])?;

        if data.len() < (32 + items_len * 64) as usize {
            return Err(Error::BundleHeadersIncomplete);
        }

        let mut items: Vec<BundleItem> = vec![];
        let mut items_start = (32 + items_len * 64) as usize;
        for i in 0..items_len {
            let item_header_begin = (32 + i * 64) as usize;
            let item_header_end = item_header_begin + 64;

            if data.len() < item_header_end {
                return Err(Error::ItemHeaderIncomplete);
            }

            let header = &data[item_header_begin..item_header_end];
            let item_length = utils::byte_array_to_u64(&header[..32])? as usize;
            let item_id = base64.encode(&header[32..64]);

            if data.len() < items_start + item_length {
                return Err(Error::ItemDataIncomplete);
            }

            let item_data = &data[items_start..items_start + item_length];
            let item = BundleItem::new(item_data, tags_schema)?;
            if item.id != item_id {
                return Err(Error::IdMismatch {
                    expected: item_id,
                    found: item.id,
                });
            }

            items.push(item);
            items_start += item_length;
        }

        Ok(Self {
            items,
        })
    }
}

#[derive(Serialize, PartialEq)]
pub struct BundleItem {
    pub id: String,
    owner: String,
    target: String,
    anchor: String,
    tags: Base64Tags,
    data: String,
    signature: String,
    signature_type: u16,
}

impl BundleItem {
    pub fn new(data: &[u8], tags_schema: &Schema) -> Result<Self, Error> {
        if data.len() < 2 {
            return Err(Error::ItemDataLessThanMinimum);
        }

        let sig_types = get_sig_types();

        let sig_type = utils::byte_array_to_u64(&data[..2])?;
        if !sig_types.contains_key(&sig_type) {
            return Err(Error::SignatureNotSupported {
                sig_type,
            });
        }

        let sig_type = sig_types.get(&sig_type).unwrap();
        let sig_length = sig_type.sig_length as usize;
        let pub_length = sig_type.pub_length as usize;

        if data.len() < 2 + sig_length {
            return Err(Error::ItemDataIncomplete);
        }

        let sig_bytes = &data[2..2 + sig_length];
        let signature = base64.encode(sig_bytes);
        let id_hash = utils::sha256(sig_bytes);
        let id = base64.encode(id_hash);

        if data.len() < sig_length + pub_length + 2 {
            return Err(Error::ItemDataLessThanSignature);
        }

        let owner = base64.encode(&data[2 + sig_length..2 + sig_length + pub_length]);

        let mut target = String::new();
        let mut anchor = String::new();
        let pos = 2 + sig_length + pub_length;

        let mut anchor_byte = pos + 1;
        let mut tags_start = pos + 2;

        if data.len() < pos {
            return Err(Error::ItemDataIncomplete);
        }

        let target_available = data[pos] == 1;
        if target_available {
            tags_start += 32;
            anchor_byte += 32;

            if data.len() < pos + 1 + 32 {
                return Err(Error::ItemDataIncomplete);
            }

            target = base64.encode(&data[pos + 1..pos + 1 + 32]);
        }

        let anchor_available = data[anchor_byte] == 1;
        if anchor_available {
            tags_start += 32;

            if data.len() < anchor_byte + 1 + 32 {
                return Err(Error::ItemDataIncomplete);
            }

            anchor = base64.encode(&data[anchor_byte + 1..anchor_byte + 1 + 32]);
        }

        let tag_count = utils::byte_array_to_u64(&data[tags_start..tags_start + 8])?;
        let tag_bytes_length: usize;
        let tag_bytes: Vec<u8>;

        let mut tags = Base64Tags(vec![]);

        if tag_count > 0 {
            if data.len() < tags_start + 16 {
                return Err(Error::ItemDataIncomplete);
            }

            tag_bytes_length =
                utils::byte_array_to_u64(&data[tags_start + 8..tags_start + 16])? as usize;
            if data.len() < tags_start + 16 + tag_bytes_length {
                return Err(Error::ItemDataIncomplete);
            }

            tag_bytes = data[tags_start + 16..tags_start + 16 + tag_bytes_length].to_vec();
            tags = Base64Tags::from_avro(tags_schema, tag_bytes)?;
        }

        Ok(Self {
            id,
            owner,
            target,
            anchor,
            tags,
            data: Default::default(),
            signature,
            signature_type: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::TAGS_AVRO_SCHEMA;
    use std::path::Path;

    const TEST_BUNDLE: &str = "res/test_bundle";
    const TEST_BUNDLE_JSON: &str = "res/test_bundle.json";

    #[test]
    fn test_bundle_new() {
        let path = Path::new(TEST_BUNDLE);

        if !path.exists() {
            panic!("test bundle file not found at {}", TEST_BUNDLE);
        }

        let test_bundle = std::fs::read(TEST_BUNDLE).unwrap();

        let bundle =
            Bundle::new(test_bundle.as_slice(), &Schema::parse_str(TAGS_AVRO_SCHEMA).unwrap())
                .unwrap();
        let json = serde_json::to_string(&bundle.items).unwrap();

        let test_bundle_json = std::fs::read_to_string(TEST_BUNDLE_JSON).unwrap();

        assert_eq!(json, test_bundle_json);
    }
}
