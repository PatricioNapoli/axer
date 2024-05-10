use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Deserialize)]
pub struct Cache<T> {
    pub data: HashMap<String, T>,
    file: String,
}

impl<T: Serialize + DeserializeOwned> Cache<T> {
    pub fn from_file(file: String) -> Self {
        let path = Path::new(file.as_str());
        let mut data = HashMap::<String, T>::new();

        if path.exists() {
            let file = std::fs::read_to_string(path).unwrap();
            data = serde_json::from_reader(file.as_bytes()).unwrap();
        } else {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        }

        Self {
            data,
            file,
        }
    }

    pub fn flush(&self) {
        let file = std::fs::File::create(self.file.as_str()).unwrap();
        serde_json::to_writer(file, &self.data).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CACHE: &str = "res/test_cache.json";

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Test {
        pub name: String,
    }

    #[test]
    fn test_cache() {
        let cache = Cache::<Test>::from_file(TEST_CACHE.to_string());
        assert_eq!(cache.data.get("1").unwrap().name, "duck");
    }

    #[test]
    fn test_cache_flush() {
        let mut cache = Cache::<Test>::from_file(TEST_CACHE.to_string());
        assert_eq!(cache.data.get("1").unwrap().name, "duck");

        cache.data.remove("2");
        cache.data.insert(
            "2".to_string(),
            Test {
                name: "goose".to_string(),
            },
        );
        cache.flush();

        let cache = Cache::<Test>::from_file(TEST_CACHE.to_string());
        assert_eq!(cache.data.get("2").unwrap().name, "goose");
    }
}
