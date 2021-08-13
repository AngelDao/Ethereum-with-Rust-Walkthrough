# Connecting to Ethereum/Liquity Contracts

## Liquity Trove Liquidation Mechanics and Incentives

### Trove, CR, MCR, TCR explanation

To start, a [Trove](https://docs.liquity.org/faq/borrowing#what-is-a-trove) is what a loan is labeled in the Liquity protocol, it can also be thought of as a [CDP](https://settle.finance/blog/cdp-explained-makerdao/), like in the MakerDAO protocol.

A Trove stores your current debt and collateral which is where a Trove's Collateralization Ratio(CR) is derived from.

`CR = (eth_collateral * current_eth_usd_price)/lusd_debt `

The CR determines if a Trove can be liquidated by comparing it to Liquity's _current_ [Minimum Collateral Ratio](https://docs.liquity.org/faq/borrowing#what-is-the-minimum-collateral-ratio-mcr-and-the-recommended-collateral-ratio)(MCR).

The MCR is determined by what Liquity's current [Total Collateral Ratio](https://docs.liquity.org/faq/recovery-mode#what-is-the-total-collateral-ratio)(TCR). If the TCR is above or equal to 1.5 then the MCR is 1.1, if the TCR is below 1.5 then the MCR is at most 1.5.

**So if a Troves CR is below the current MCR it can be liquidated**

### Liquidation Incentives

For liquidating a Trove, the Liquidator gets 200 LUSD and a 0.5% of the Trove collateral.

## Video 2 changelog

- `.env`
  _Stores the [Infura](https://infura.io)_ `PROJECT_ID` _and the_ `PRIVATE_KEY` _for the wallet that will be used to send txs_

- `Cargo.toml`
  _This file stores app details and the dependencies use in app, the dependencies are:_
  ```
  [dependencies]
  ethers = "0.2.2"
  serde_json = "1.0.59"
  tokio = { version = "1", features = ["full"] }
  dotenv = "0.15.0"
  rust-crypto = "0.2"
  ```
- `Cargo.lock`
  _This file stores the versioning details and dependencies for each of the crates will be generated after running_ `cargo build` _or_ `cargo run`.

- **target/**
  _This dir stores the application compiled version where it gets executed from, the dir is created from_ `cargo build`
- **src/**

  - `main.rs`
    _The_ `main()` _function is special: it is always the first code that runs in every executable Rust program._
  - `contracts.rs`
    _Connects the application to all the contracts used_
  - **utils/**

    - `addresses.rs`
      _Addresses for all the contracts used, from Ethereum mainnet_
    - **abis/**

      - `abis.rs`
        _imports all the contracts into one file making exporting cleaner_
      - `IPriceFeedABI.rs`
        _abi for the Chainlink[ ETH/USD price feed](https://docs.chain.link/docs/get-the-latest-price/#javascript)_
      - `ISortedTrovesABI.rs`
        _abi for Liquity [SortedTroves](https://github.com/liquity/dev/blob/main/packages/lib-ethers/abi/SortedTroves.json) contract_
      - `ITroveMaangerABI.rs`
        _abi for Liquity [TroveManager](https://github.com/liquity/dev/blob/main/packages/lib-ethers/abi/TroveManager.json) contract_
