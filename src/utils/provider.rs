use anyhow::Result;
use ethers::prelude::*;
use std::env;

pub fn get_provider() -> Result<Provider<Http>> {
    let rpc_url =
        env::var("ETH_RPC_URL").unwrap_or_else(|_| "https://eth.llamarpc.com".to_string());

    println!("🌐 Using RPC: {}", rpc_url);

    let provider = Provider::<Http>::try_from(rpc_url)?;
    Ok(provider)
}
