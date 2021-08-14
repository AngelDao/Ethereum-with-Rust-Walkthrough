use super::get_troves;
use ethers::{
    contract::Contract,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::LocalWallet,
};

pub fn run(
    _trove_manager: &Contract<&SignerMiddleware<Provider<Http>, LocalWallet>>,
    troves: Vec<get_troves::Trove>,
) {
    for t in troves {
        // println!("{}", t);
        // trove_manager
        //     .method::<_, String>("liquidate", t.owner)
        //     .expect("method fail")
        //     .send();
    }
}
