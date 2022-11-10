use ethers_core::types::Address;
use ethers_providers::{Provider, Http, Middleware};
use std::convert::TryFrom;

let provider = Provider::<Http>::try_from(
    "https://mainnet.infura.io/v3/7ea6d7a435384334ae07f5cadb12f93b"
)?;

let block = provider.get_block(100u64).await?;
println!("Got block: {}", serde_json::to_string(&block)?);

let addr = "0x89d24a6b4ccb1b6faa2625fe562bdd9a23260359".parse::<Address>()?;
let code = provider.get_code(addr, None).await?;
println!("Got code: {}", serde_json::to_string(&code)?);