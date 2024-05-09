use serde::Deserialize;
use std::fmt::{Display, Formatter};

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

impl Display for Network {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Network {{ network: {}, version: {}, release: {}, blocks: {}, peers: {} }}",
            self.network, self.version, self.release, self.blocks, self.peers,
        )
    }
}
