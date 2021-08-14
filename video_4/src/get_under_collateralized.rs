use super::get_troves;
use ethers::{
    contract::Contract,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::LocalWallet,
};

async fn get_eth_usd_price(
    price_feed: Contract<&SignerMiddleware<Provider<Http>, LocalWallet>>,
) -> f64 {
    let (_id, price, _start, _end, _round): (u128, u128, u128, u128, u128) = price_feed
        .method::<_, _>("latestRoundData", ())
        .expect("fail method")
        .call()
        .await
        .expect("fail wait");
    return price as f64 / 1e8;
}

async fn get_tcr(
    trove_manager: &Contract<&SignerMiddleware<Provider<Http>, LocalWallet>>,
    price: f64,
) -> f64 {
    let tcr: u128 = trove_manager
        .method::<_, _>("getTCR", (price * 1e18) as u128)
        .expect("fail method")
        .call()
        .await
        .expect("fail wait");
    return tcr as f64 / 1e18;
}

pub async fn run(
    trove_manager: &Contract<&SignerMiddleware<Provider<Http>, LocalWallet>>,
    price_feed: Contract<&SignerMiddleware<Provider<Http>, LocalWallet>>,
    troves: Vec<get_troves::Trove>,
) -> Vec<get_troves::Trove> {
    let mut to_liquidate: Vec<get_troves::Trove> = vec![];
    let eth_usd: f64 = get_eth_usd_price(price_feed).await;
    let tcr = get_tcr(trove_manager, eth_usd).await;
    println!("Liquity TCR: {}\n", tcr);
    let mcr: f64;
    let stabilty_mode: bool;

    if tcr < 1.5 {
        stabilty_mode = true;
        mcr = 1.5;
    } else {
        stabilty_mode = false;
        mcr = 1.1;
    }

    println!("Liquity current MCR: {}\n", mcr);

    println!("Lowest CR Troves by CR:\n");

    for t in troves {
        let cr = t.collateral * eth_usd / t.debt;
        println!("{} %", cr);
        if cr < mcr && !stabilty_mode {
            to_liquidate.push(t);
        } else if cr <= mcr && stabilty_mode {
            to_liquidate.push(t);
        }
    }
    println!("\nTroves below MCR: {}", to_liquidate.len());
    return to_liquidate;
}
