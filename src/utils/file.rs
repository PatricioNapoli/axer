use std::path::PathBuf;
use serde::Serialize;
use tracing::info;

pub fn save_serde_json<T: Serialize>(path: PathBuf, data: &T) -> Result<(), std::io::Error> {
    if !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    }

    info!("saving file to: {}", path.to_str().unwrap());

    let json = serde_json::to_string(&data).unwrap();
    std::fs::write(path, json)?;

    Ok(())
}
