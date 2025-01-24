# <h1 align="center"> alloy-fireblocks </h1>

Provides [alloy-rs](https://github.com/alloy-rs/alloy) a compatible EIP-1193 compatible provider and API client.
implementations for the [Fireblocks Vaults API](https://fireblocks.com).
This is based off the [ethers-fireblocks](https://github.com/gakonst/ethers-fireblocks) repo and [fireblocks-web3-provider](https://github.com/fireblocks/fireblocks-web3-provider).

Note that `alloy-fireblocks` approximately mirrors the functionality used in  [fireblocks-web3-provider](https://github.com/fireblocks/fireblocks-web3-provider) but there will be some discrepancies.

Note that this is a community port for fireblocks and isn't maintained by the official fireblocks team.

## Documentation

Clone the repository and run `cd alloy-fireblocks/ && cargo doc --open`

## Add alloy-fireblocks to your repository

```toml
[dependencies]

alloy-fireblocks = { git = "https://github.com/jseam2/alloy-fireblocks" }
```

## Test
To test, obtain an `API_KEY` and `PRIVATE_KEY` from Fireblocks and create the files respectively by remove `.example`.

To run test with outputs
```shell
cargo nextest run --no-capture --locked
```