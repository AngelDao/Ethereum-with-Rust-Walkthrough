# Finding undercollateralized Troves and Liquidating

## MEV

- _malicious reording of txs_

- _very competitive_

[learn more](https://blog.chain.link/what-is-miner-extractable-value-mev/)

## Video 3 changelog

- `get_under_collateralized.rs`
  _fetch Ether USD price via_ `price_feed` _to calculate and set CR for each trove, then deterine if it is liquidatable_
- `liquidate.rs`
  _liquidate troves by call_ `liquidate()` _on_ `trove_manager` _for each under collatearlized trove_
