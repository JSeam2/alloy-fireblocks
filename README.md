# <h1 align="center"> alloy-fireblocks </h1>

 Provides [alloy-rs](https://github.com/alloy-rs/alloy)-compatible Signer and Middleware
 implementations for the [Fireblocks Vaults API](https://fireblocks.com).
 This is based off the [ethers-fireblocks]9https://github.com/gakonst/ethers-fireblocks) repo.

## Documentation

Clone the repository and run `cd alloy-fireblocks/ && cargo doc --open`

## Add alloy-fireblocks to your repository

```toml
[dependencies]

alloy-fireblocks = { git = "https://github.com/jseam2/alloy-fireblocks" }
```

## Test
To test, obtain an `API_KEY` and `PRIVATE_KEY` from Fireblocks and create the files respectively.

To run test with outputs
```shell
cargo test -- --nocapture
```