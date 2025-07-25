use crate::utils::provider::get_provider;
use crate::wallet::Wallet;
use anyhow::Result;
use ethers::prelude::*;
use ethers::utils::parse_ether;
use std::str::FromStr;

pub async fn send_transaction(to: &str, amount: f64, wallet_path: Option<String>) -> Result<()> {
    if wallet_path.is_none() {
        println!("Please specify a wallet file with --wallet");
        return Ok(());
    }

    let wallet = Wallet::load_from_file(&wallet_path.unwrap())?;
    let from_address = wallet.address();
    let provider = get_provider()?;
    let client = wallet.with_provider(provider);

    let to_address = Address::from_str(to)?;
    let value = parse_ether(amount)?;

    println!("📤 Preparing transaction...");
    println!("   From: {}", from_address);
    println!("   To: {:?}", to_address);
    println!("   Amount: {} ETH", amount);

    let tx = TransactionRequest::new().to(to_address).value(value);

    let pending_tx = client.send_transaction(tx, None).await?;
    println!("🚀 Transaction sent! Hash: {:?}", pending_tx.tx_hash());

    println!("⏳ Waiting for confirmation...");
    let receipt = pending_tx.await?;

    if let Some(receipt) = receipt {
        println!("✅ Transaction confirmed!");
        println!("   Block: {:?}", receipt.block_number);
        println!("   Gas used: {:?}", receipt.gas_used);
    }

    Ok(())
}
