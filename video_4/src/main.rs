// extern crate ethers;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use std::convert::TryFrom;
mod contracts;
mod get_troves;
mod get_under_collateralized;
mod liquidate;
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
    println!("contracts connected");
    // get liquity troves
    let troves: Vec<get_troves::Trove> = get_troves::run(sorted_troves, &trove_manager).await;
    println!("Troves got: {}\n", troves.len());
    // get the troves under collateralization ratio 1.1
    let under_c_troves: Vec<get_troves::Trove> =
        get_under_collateralized::run(&trove_manager, price_feed, troves).await;
    let count = under_c_troves.len();
    println!("\nTroves to be liquidated: {}", count);
    if count > 0 {
        // liquidate troves
        liquidate::run(&trove_manager, under_c_troves);
    }
}
