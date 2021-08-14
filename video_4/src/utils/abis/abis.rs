#[path = "./ITroveManagerABI.rs"]
mod trove_manager;

#[path = "./ISortedTrovesABI.rs"]
mod sorted_troves;

#[path = "./IPriceFeedABI.rs"]
mod price_feed;

pub fn sorted_troves() -> String {
    return sorted_troves::i_sorted_troves_abi();
}

pub fn trove_manager() -> String {
    return trove_manager::i_trove_manager_abi();
}

pub fn price_feed() -> String {
    return price_feed::i_price_feed_v3_abi();
}
