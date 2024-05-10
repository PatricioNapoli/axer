use std::collections::HashMap;

pub struct SigConfig {
    pub sig_name: &'static str,
    pub sig_length: u32,
    pub pub_length: u32,
}

pub fn get_sig_types() -> HashMap<u64, SigConfig> {
    HashMap::from([
        (
            1,
            SigConfig {
                sig_name: "arweave",
                sig_length: 512,
                pub_length: 512,
            },
        ),
        (
            2,
            SigConfig {
                sig_name: "ed25519",
                sig_length: 64,
                pub_length: 32,
            },
        ),
        (
            3,
            SigConfig {
                sig_name: "ethereum",
                sig_length: 65,
                pub_length: 65,
            },
        ),
        (
            4,
            SigConfig {
                sig_name: "solana",
                sig_length: 64,
                pub_length: 32,
            },
        ),
    ])
}
