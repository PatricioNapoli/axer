use crate::Error;
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::str::FromStr;

pub const WINSTONS_PER_AR: u64 = 1_000_000_000_000;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Currency {
    arweave: u64, //integer
    winston: u64, //decimal
}

impl From<u128> for Currency {
    fn from(u: u128) -> Self {
        let s = u.to_string();
        let mut arweave: u64 = 0;
        let winston: u64;
        if s.len() <= 12 {
            winston = u as u64;
        } else {
            let d = s.split_at(s.len() - 12);
            winston = (u % (WINSTONS_PER_AR as u128)) as u64;
            arweave = d.0.parse::<u64>().unwrap();
        }

        Self {
            arweave,
            winston,
        }
    }
}

impl FromStr for Currency {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let split: Vec<&str> = s.split('.').collect();
        if split.len() == 2 {
            Ok(Currency {
                arweave: split[0].parse::<u64>().map_err(Error::ParseIntError)?,
                winston: split[1].parse::<u64>().map_err(Error::ParseIntError)?,
            })
        } else {
            Ok(Currency {
                winston: split[0].parse::<u64>().map_err(Error::ParseIntError)?,
                ..Currency::default()
            })
        }
    }
}

impl<'de> Deserialize<'de> for Currency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match Value::deserialize(deserializer)? {
            Value::String(s) => Currency::from_str(&s).expect("Could not deserialize"),
            Value::Number(num) => {
                Currency::from(num.as_u64().expect("Could not deserialize") as u128)
            }
            _ => return Err(de::Error::custom("Wrong type")),
        })
    }
}
