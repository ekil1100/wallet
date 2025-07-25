use crate::wallet::Wallet;
use anyhow::Result;

pub async fn new_wallet(save_path: Option<String>) -> Result<()> {
    let wallet = Wallet::new()?;
    
    println!("🔐 New wallet generated!");
    println!("Address: {}", wallet.address());
    println!("Private Key: 0x{}", wallet.private_key());
    
    if let Some(path) = save_path {
        wallet.save_to_file(&path)?;
        println!("✅ Wallet saved to: {}", path);
    } else {
        println!("\n⚠️  Make sure to save your private key securely!");
    }
    
    Ok(())
}

pub async fn import_wallet(private_key: &str, save_path: Option<String>) -> Result<()> {
    let wallet = Wallet::from_private_key(private_key)?;
    
    println!("✅ Wallet imported successfully!");
    println!("Address: {}", wallet.address());
    
    if let Some(path) = save_path {
        wallet.save_to_file(&path)?;
        println!("✅ Wallet saved to: {}", path);
    }
    
    Ok(())
}

pub async fn show_address(wallet_path: Option<String>) -> Result<()> {
    if let Some(path) = wallet_path {
        let wallet = Wallet::load_from_file(&path)?;
        println!("Address: {}", wallet.address());
    } else {
        println!("Please specify a wallet file with --wallet");
    }
    
    Ok(())
}