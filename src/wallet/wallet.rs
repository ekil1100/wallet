use anyhow::Result;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WalletError {
    #[error("Invalid private key")]
    InvalidPrivateKey,
    #[error("File operation failed: {0}")]
    FileError(String),
    #[error("Serialization failed: {0}")]
    SerializationError(String),
}

#[derive(Serialize, Deserialize)]
pub struct WalletData {
    pub address: String,
    pub private_key: String,
}

pub struct Wallet {
    pub signer: LocalWallet,
}

impl Wallet {
    pub fn new() -> Result<Self> {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        Ok(Self { signer: wallet })
    }

    pub fn from_private_key(private_key: &str) -> Result<Self> {
        let key = private_key.trim_start_matches("0x");
        let wallet = LocalWallet::from_str(key).map_err(|_| WalletError::InvalidPrivateKey)?;
        Ok(Self { signer: wallet })
    }

    pub fn address(&self) -> String {
        format!("{:?}", self.signer.address())
    }

    pub fn private_key(&self) -> String {
        format!("{}", hex::encode(self.signer.signer().to_bytes()))
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let wallet_data = WalletData {
            address: self.address(),
            private_key: self.private_key(),
        };

        let json = serde_json::to_string_pretty(&wallet_data)
            .map_err(|e| WalletError::SerializationError(e.to_string()))?;

        fs::write(path, json).map_err(|e| WalletError::FileError(e.to_string()))?;

        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<Self> {
        let json = fs::read_to_string(path).map_err(|e| WalletError::FileError(e.to_string()))?;

        let wallet_data: WalletData = serde_json::from_str(&json)
            .map_err(|e| WalletError::SerializationError(e.to_string()))?;

        Self::from_private_key(&wallet_data.private_key)
    }

    pub fn with_provider(
        self,
        provider: Provider<Http>,
    ) -> SignerMiddleware<Provider<Http>, LocalWallet> {
        SignerMiddleware::new(provider, self.signer)
    }
}
