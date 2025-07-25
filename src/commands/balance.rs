use crate::utils::provider::get_provider;
use crate::wallet::Wallet;
use anyhow::Result;
use ethers::prelude::*;
use std::str::FromStr;

pub async fn check_balance(address: Option<String>, wallet_path: Option<String>) -> Result<()> {
    let provider = get_provider()?;

    let check_address = if let Some(addr) = address {
        Address::from_str(&addr)?
    } else if let Some(path) = wallet_path {
        let wallet = Wallet::load_from_file(&path)?;
        Address::from_str(&wallet.address())?
    } else {
        println!("Please provide an address or wallet file");
        return Ok(());
    };

    let balance = provider.get_balance(check_address, None).await?;
    let balance_eth = ethers::utils::format_ether(balance);

    println!("💰 Balance: {} ETH", balance_eth);
    println!("   Address: {:?}", check_address);

    Ok(())
}
