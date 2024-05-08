use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Network {
    pub network: String,
    pub version: u32,
    pub release: u32,
    pub height: u32,
    pub current: String,
    pub blocks: u32,
    pub peers: u32,
    pub queue_length: u32,
    pub node_state_latency: u32,
}
