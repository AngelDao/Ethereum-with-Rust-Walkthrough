// extern crate ethers;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use std::convert::TryFrom;
mod contracts;
use dotenv::dotenv;
// use std::env;

#[tokio::main]
async fn main() {
    println!("\nLiquity Liquidator Start!\n");
    dotenv().ok();

    let pk = dotenv::var("PRIVATE_KEY").unwrap();
    let wallet: LocalWallet = pk.parse().expect("fail parse");
    // println!("{}", pk);
    let provider_id = dotenv::var("PROVIDER_ID").unwrap();
    let url = format!("https://mainnet.infura.io/v3/{}", provider_id);

    // connect provider
    let provider_service = Provider::<Http>::try_from(url).expect("failed");

    let provider = SignerMiddleware::new(provider_service, wallet);
    // connect contracts
    let [trove_manager, sorted_troves, price_feed] = contracts::get_contracts(&provider);
    // get liquity troves
    
}

