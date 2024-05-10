<div align="center">

<a style="margin-right:15px" href="#"><img src="https://forthebadge.com/images/badges/made-with-rust.svg" alt="Made with Rust"/></a>


<a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-brightgreen.svg" alt="License MIT"/></a>
<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-1.78-orange.svg" alt="Rust 1.77"/></a>

</div>

# AXER

(Arweave Indexer) A simple Arweave CLI for connecting to arweave net, index a bundle transaction by its id, and dump it to a file. 

### Features

- Interactive, single and batch modes
- Simple json file-based cache of transactions
- Parses the whole bundle binary including the AVRO tags
- Tokio based async requests

# Usage

You can run the help command to see the available options:
```bash
$ cargo run -- --help

Usage: axer [--url <url>] [--timeout <timeout>] [--db-file <db-file>] [-o <out-dir>] [--tx-id <tx-id>] [-b <batch-file>] [-i]

Axer CLI - Arweave bundle explorer.

Options:
  --url             network base url
  --timeout         network timeout in ms
  --db-file         index db filename
  -o, --out-dir     output directory for parsed files
  --tx-id           arweave bundle transaction ID, enables single mode
  -b, --batch-file  batch filename, enables batch mode
  -i, --interactive enables interactive mode
  --help            display usage information
```

For example, for a single transaction:
```bash
$ cargo run -- --tx-id aJ3PrkyJ6GpdwwUxxXFHiB40cEg-GPRUWcKUI6wCgPQ
```

For batch mode, batch files are text files with one transaction ID per line and used as follows:
```bash
$ cargo run -- -b batch_ids.txt
```

These are the argument defaults:
```bash
{ 
  url: "https://arweave.net", 
  timeout: 5000, 
  db_file: "cache.json", 
  out_dir: "out/", 
  tx_id: None, 
  batch_file: None, 
  interactive: false
}
```

After running any mode, the output directory will contain the parsed bundle array files in json format for each transaction.  
In interactive mode, tx cache is only saved when exiting with 'q'.

# Logs

For a single mode run, the console logs will show the following:
```bash
$ cargo run -- --tx-id aJ3PrkyJ6GpdwwUxxXFHiB40cEg-GPRUWcKUI6wCgPQ

2024-05-10T17:16:12.218731Z  INFO axer::cli: running with Args { url: "https://arweave.net", timeout: 5000, db_file: "cache.json", out_dir: "out/", tx_id: Some("aJ3PrkyJ6GpdwwUxxXFHiB40cEg-GPRUWcKUI6wCgPQ"), batch_file: None, interactive: false }
2024-05-10T17:16:12.962364Z  INFO axer::cli: connected to: Network { network: arweave.N.1, version: 5, release: 69, blocks: 1421559, peers: 280 }
2024-05-10T17:16:12.962566Z  INFO axer::cli: running single mode for transaction: aJ3PrkyJ6GpdwwUxxXFHiB40cEg-GPRUWcKUI6wCgPQ
2024-05-10T17:16:14.456013Z  INFO axer::cli: transaction: Bundle Transaction { id: aJ3PrkyJ6GpdwwUxxXFHiB40cEg-GPRUWcKUI6wCgPQ, last_tx: 2iAqen10b8K0lVB3xmtp8plsd0GZzrc_yAoG8C9Fccz67Zc9U0vsvvP2S3S7tMtN, tags: "App"="everPay";"Version"="2.0.0";"Owner"="uGx-QfBXSwABKxjha-00dI7vvfyqIYblY6Z5L6cyTFM";"parent_id"="Lac6dfslKmKfWOogPcc1kTlcz9_cIYvz48E_sV9_vkE";"Bundle-Format"="binary";"Bundle-Version"="2.0.0";, data_size: 398979}
2024-05-10T17:16:14.456400Z  INFO axer::utils::file: saving file to: out/aJ3PrkyJ6GpdwwUxxXFHiB40cEg-GPRUWcKUI6wCgPQ.json
```

Requests in batch mode are run in parallel, so the order of the transactions in the output files may vary.  
When fetching from cache, the console logs will output 'from cache'.

# Testing

Tests are available, and you can run them with:
```bash
$ cargo test
```