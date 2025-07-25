use anyhow::Result;
use clap::{Parser, Subcommand};
use dotenv::dotenv;

mod commands;
mod utils;
mod wallet;

use commands::{account, balance, transaction};

#[derive(Parser)]
#[command(name = "wallet")]
#[command(about = "A lightweight Web3 CLI wallet for Ethereum", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate a new wallet")]
    New {
        #[arg(short, long, help = "Save wallet to file")]
        save: Option<String>,
    },
    #[command(about = "Import wallet from private key")]
    Import {
        #[arg(short, long, help = "Private key (without 0x prefix)")]
        key: String,
        #[arg(short, long, help = "Save wallet to file")]
        save: Option<String>,
    },
    #[command(about = "Show wallet address")]
    Address {
        #[arg(short, long, help = "Load wallet from file")]
        wallet: Option<String>,
    },
    #[command(about = "Check balance")]
    Balance {
        #[arg(short, long, help = "Address to check (default: current wallet)")]
        address: Option<String>,
        #[arg(short, long, help = "Load wallet from file")]
        wallet: Option<String>,
    },
    #[command(about = "Send transaction")]
    Send {
        #[arg(short, long, help = "Recipient address")]
        to: String,
        #[arg(short, long, help = "Amount in ETH")]
        amount: f64,
        #[arg(short, long, help = "Load wallet from file")]
        wallet: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::New { save } => {
            account::new_wallet(save).await?;
        }
        Commands::Import { key, save } => {
            account::import_wallet(&key, save).await?;
        }
        Commands::Address { wallet } => {
            account::show_address(wallet).await?;
        }
        Commands::Balance { address, wallet } => {
            balance::check_balance(address, wallet).await?;
        }
        Commands::Send { to, amount, wallet } => {
            transaction::send_transaction(&to, amount, wallet).await?;
        }
    }

    Ok(())
}
