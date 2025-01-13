# <h1 align="center"> alloy-fireblocks </h1>

 Provides [alloy-rs](https://github.com/alloy-rs/alloy)-compatible Signer and Middleware
 implementations for the [Fireblocks Vaults API](https://fireblocks.com).
 This is based off the [ethers-fireblocks]9https://github.com/gakonst/ethers-fireblocks) repo.

## Documentation

Clone the repository and run `cd alloy-fireblocks/ && cargo doc --open`

## Add ethers-fireblocks to your repository

```toml
[dependencies]

alloy-fireblocks = { git = "https://github.com/jseam2/alloy-fireblocks" }
```

To use the example, you must have the following env vars set:

```
export FIREBLOCKS_API_SECRET_PATH=<path to your fireblocks.key>
export FIREBLOCKS_API_KEY=<your fireblocks api key>
export FIREBLOCKS_SOURCE_VAULT_ACCOUNT=<the vault id being used for sending txs>
```
