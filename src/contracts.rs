use ethers::middleware::SignerMiddleware;
use ethers::{
    abi::Abi,
    contract::Contract,
    providers::{Http, Provider},
    signers::LocalWallet,
    types::Address,
};
#[path = "./utils/abis/abis.rs"]
mod abis;
#[path = "./utils/addresses.rs"]
mod addresses;

fn get_sorted_troves(
    provider: &SignerMiddleware<Provider<Http>, LocalWallet>,
) -> Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> {
    let abi_original: String = abis::sorted_troves();
    let abi: Abi = serde_json::from_str(&abi_original).expect("failed");
    let address: Address = (addresses::contracts()).i_sorted_trove;
    let contract = Contract::new(address, abi, provider);
    return contract;
}

fn get_trove_manager(
    provider: &SignerMiddleware<Provider<Http>, LocalWallet>,
) -> Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> {
    let abi_original: String = abis::trove_manager();
    let abi: Abi = serde_json::from_str(&abi_original).expect("failed");
    let address: Address = (addresses::contracts()).i_trove_manager;
    let contract = Contract::new(address, abi, provider);
    return contract;
}

fn get_price_feed(
    provider: &SignerMiddleware<Provider<Http>, LocalWallet>,
) -> Contract<&SignerMiddleware<Provider<Http>, LocalWallet>> {
    let abi_original: String = abis::price_feed();
    let abi: Abi = serde_json::from_str(&abi_original).expect("failed");
    let address: Address = (addresses::contracts()).i_price_feed_v3;
    let contract = Contract::new(address, abi, provider);
    return contract;
}

pub fn get_contracts(
    provider: &SignerMiddleware<Provider<Http>, LocalWallet>,
) -> [Contract<&SignerMiddleware<Provider<Http>, LocalWallet>>; 3] {
    let static_provider = &provider;
    let trove_manager = get_trove_manager(static_provider);
    let sorted_troves = get_sorted_troves(static_provider);
    let price_feed = get_price_feed(static_provider);
    return [trove_manager, sorted_troves, price_feed];
}
