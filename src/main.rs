use std::sync::Arc;
use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Provider, StreamExt, Ws},
};
use anyhow::Result;
use dotenv::dotenv;

abigen!(Raisin, "./abi.json");

const RAISIN_ADDRESS: &str = "0x7e37Cd627C75DB9b76331F484449E5d98D5C82c5";

#[tokio::main]
async fn main() -> Result<()> { 
    dotenv()?; 
    let api_key = std::env::var("API_KEY")?;
    let provider = Provider::<Ws>::connect(api_key).await?;
    let client = Arc::new(provider);
    let address: Address = RAISIN_ADDRESS.parse()?;
    let contract = Raisin::new(address, client);
    listen_all_events(&contract).await?;
    Ok(())
}

async fn listen_all_events(contract: &Raisin<Provider<Ws>>) -> Result<()> {
    // optionally sync from recent? 
    let events = contract.events().from_block(3000000);
    let mut stream = events.stream().await?.take(1);

    while let Some(Ok(evt)) = stream.next().await {
        match evt {
            RaisinEvents::FundStartedFilter(f) => println!("{f:?}"),
            RaisinEvents::TokenDonatedFilter(f) => println!("{f:?}"),
            RaisinEvents::TokenAddedFilter(f) => println!("{f:?}"),
            RaisinEvents::TokenRemovedFilter(f) => println!("{f:?}"),
            RaisinEvents::FundEndedFilter(f) => println!("{f:?}"),
            RaisinEvents::WithdrawFilter(f) => println!("{f:?}"),
            RaisinEvents::FeeChangedFilter(f) => println!("{f:?}"),
            RaisinEvents::PartnershipActivatedFilter(f) => println!("{f:?}"),
            RaisinEvents::PartnershipDeactivatedFilter(f) => println!("{f:?}"),
            RaisinEvents::VaultChangedFilter(f) => println!("{f:?}"),
            RaisinEvents::OwnershipTransferredFilter(f) => println!("{f:?}"),
            RaisinEvents::RefundFilter(f) => println!("{f:?}"),
        }
    }

    Ok(())
}