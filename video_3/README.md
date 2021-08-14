# Getting Troves from Ethereum/Liquity Contracts

## Liquity Contracts

### TroveManager

This contract allows you to interact with Troves, like the name states this simply manages the troves, by doing things like, redeeming collateral, liquidations, etc

### SortedTroves

This contract stores all Troves in a doubly linked list and for each trove stores the amount of collateral and debt the Trove has.

### PriceFeed

The Liquity contracts use Chainlink price feeds to determine Ether price so, this is the price feed, the liquidator should use.

## Video 3 changelog

- `get_troves.rs`
  _uses_ `sorted_troves` _and_ `trove_manager` _to get n number of troves from liquity starting from the lowest CR to highest_

## Additional

### Rust Ownership and Borrowing

- Ownership rules:
  1.  each value in rust has a variable that is called its owner
  2.  there can only be one owner at a time
  3.  when the value goes out of scope the value will be dropped.
- Borrowing:
  _can keep ownership in the scope and pass read only borrows by using_ `&` _in front of the varialbe you are passing to be borrowed_

Check out [chapter 4](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) of the Rust book to learn more
