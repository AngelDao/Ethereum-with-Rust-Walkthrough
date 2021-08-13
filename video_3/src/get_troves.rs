use ethers::{
    contract::Contract,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::LocalWallet,
    types::Address,
};
use std::fmt;

pub struct Trove {
    pub owner: Address,
    pub collateral: f64,
    pub debt: f64,
    pub stake: f64,
    pub status: String,
    pub index: u16,
}

impl fmt::Display for Trove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"
Trove {{
    owner:      {},
    collateral: {},
    debt:       {},
    stake:      {},
    status:     {},
    index:      {} 
}}
            "#,
            self.owner, self.collateral, self.debt, self.stake, self.status, self.index
        )
    }
}

fn convert_to_trove(response: (u128, u128, u128, u8, u16), owner: Address) -> Trove {
    let (debt, collateral, stake, status, index) = response;
    let status_array: [String; 5] = [
        "nonExistent".to_string(),
        "active".to_string(),
        "closedByOwner".to_string(),
        "closedByLiquidation".to_string(),
        "closedByRedemption".to_string(),
    ];
    let formatted_coll = collateral as f64 / 1e18;
    let formatted_stake = stake as f64 / 1e18;
    let formatted_debt = debt as f64 / 1e18;
    // println!("collateral ETH {:?}", formatted_coll);
    // println!("stake ETH {:?}", formatted_stake);
    // println!("debt LUSD {:?}", formatted_debt);
    Trove {
        owner,
        collateral: formatted_coll,
        debt: formatted_debt,
        stake: formatted_stake,
        status: status_array[status as usize].to_string(),
        index,
    }
}

pub async fn run(
    sorted_troves: Contract<&SignerMiddleware<Provider<Http>, LocalWallet>>,
    trove_manager: &Contract<&SignerMiddleware<Provider<Http>, LocalWallet>>,
) -> Vec<Trove> {
    let mut troves: Vec<Trove> = vec![];
    // println!("entered");
    let mut last_trove: Address = sorted_troves
        .method::<_, Address>("getLast", ())
        .expect("fail method")
        .call()
        .await
        .expect("fail wait");

    let mut is_first: bool = true;
    let mut i: u8 = 0;
    loop {
        i += 1;
        let owner_address: Address;
        if is_first {
            owner_address = last_trove;
        } else {
            owner_address = sorted_troves
                .method::<_, Address>("getPrev", last_trove)
                .expect("fail method")
                .call()
                .await
                .expect("fail wait");
        }
        // println!("{}", i);
        if owner_address
            == "0x0000000000000000000000000000000000000000"
                .parse::<Address>()
                .expect("fail")
        {
            break;
        }
        // println!("{}", owner_address);
        let trove_pre = trove_manager
            .method::<_, (u128, u128, u128, u8, u16)>("Troves", owner_address)
            .expect("fail method")
            .call()
            .await
            .expect("fail wait");
        // println!("{:?}", trove_pre);
        let trove: Trove = convert_to_trove(trove_pre, owner_address);
        // println!("{:?}", trove.index);
        // println!("{:?}", owner_address);
        troves.push(trove);
        last_trove = owner_address;
        if is_first {
            is_first = false;
        }

        if i > 19 {
            break;
        }
    }
    return troves;
}
