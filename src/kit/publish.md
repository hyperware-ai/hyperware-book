# `kit publish`

short: `kit p`

`kit publish` creates entries in the Hypermap, publishing the given package according to the `app-store`s protocol.
It can also be used to update or unpublish previously-published packages.
`kit publish` writes directly to the Hypermap: it does not interact with a Hyperware node.

## Example Usage

```bash
# Publish a package on the real network (Base mainnet).
kit publish --metadata-uri https://raw.githubusercontent.com/path/to/metadata.json --keystore-path ~/.foundry/keystores/dev --rpc wss://opt-mainnet.g.alchemy.com/v2/<ALCHEMY_API_KEY> --real

# Unublish a package.
kit publish --metadata-uri https://raw.githubusercontent.com/path/to/metadata.json --keystore-path ~/.foundry/keystores/dev --rpc wss://opt-mainnet.g.alchemy.com/v2/<ALCHEMY_API_KEY> --real --unpublish
```

See [Sharing with the World](../my_first_app/chapter_5.md) for a tutorial on how to use `kit publish`.

## Arguments

```
$ kit publish --help
Publish or update a package

Usage: kit publish [OPTIONS] --metadata-uri <URI> --rpc <RPC_URI> [DIR]

Arguments:
  [DIR]  The package directory to publish [default: /home/nick/git/kit]

Options:
  -k, --keystore-path <PATH>
          Path to private key keystore (choose 1 of `k`, `l`, `t`, `s`)
  -l, --ledger
          Use Ledger private key (choose 1 of `k`, `l`, `t`, `s`)
  -t, --trezor
          Use Trezor private key (choose 1 of `k`, `l`, `t`, `s`)
  -s, --safe <SAFE_CONTRACT_ADDRESS>
          Create transaction for Safe (choose 1 of `k`, `l`, `t`, `s`)
  -u, --metadata-uri <URI>
          URI where metadata lives
  -r, --rpc <RPC_URI>
          Ethereum Base mainnet RPC endpoint (wss://)
  -e, --real
          If set, deploy to real network [default: fake node]
      --unpublish
          If set, unpublish existing published package [default: publish a package]
  -g, --gas-limit <GAS_LIMIT>
          The ETH transaction gas limit [default: 1_000_000]
  -p, --priority-fee <MAX_PRIORITY_FEE_PER_GAS>
          The ETH transaction max priority fee per gas [default: estimated from network conditions]
  -f, --fee-per-gas <MAX_FEE_PER_GAS>
          The ETH transaction max fee per gas [default: estimated from network conditions]
  -m, --mock
          If set, don't actually publish: just dry-run
  -h, --help
          Print help
```

### Positional arg: `DIR`

Publish the metadata for the package in this directory.

### `--metadata-uri`

short: `-u`

The URI hosting the `metadata.json`.
You must place the `metadata.json` somewhere public before publishing your package on Hypermap.
A common place to host `metadata.json` is on your package's GitHub repo.
If you use GitHub, make sure to use the static link to the specific commit, not a branch-specific URL (e.g. `main`) that will change with new commits.
For example, `https://raw.githubusercontent.com/nick1udwig/chat/master/metadata.json` is not the correct link to use, because it will change when new commits are added.
You want to use a link like `https://raw.githubusercontent.com/nick1udwig/chat/191dce595ad00a956de04b9728f479dee04863c7/metadata.json` which will not change when new commits are added.

### `--keystore-path`

short: `-k`

Use private key from keystore given by path.
The keystore is a [Web3 Secret Storage file](https://ethereum.org/en/developers/docs/data-structures-and-encoding/web3-secret-storage/) that holds an encrypted copy of your private keys.
See the [Sharing with the World](../my_first_app/chapter_5.md) usage example for one way to create a keystore.

Must supply one and only one of `--keystore-path`, `--ledger`, `--trezor`, or `--safe`.

### `--ledger`

short: `-l`

Use private key from Ledger.

Must supply one and only one of `--keystore-path`, `--ledger`, `--trezor`, or `--safe`.

### `--trezor`

short: `-t`

Use private key from Trezor.

Must supply one and only one of `--keystore-path`, `--ledger`, `--trezor`, or `--safe`.

### `--safe`

short: `-t`

Print the calldata to create a [Safe](https://app.safe.global) transaction.

Must supply one and only one of `--keystore-path`, `--ledger`, `--trezor`, or `--safe`.

### `--rpc`

short: `-r`

The Ethereum RPC endpoint to use.
For fakenodes this runs by default at `ws://localhost:8545`.

### `--real`

short: `-e`

Manipulate the real (live) Hypermap.
Default is to manipulate the fakenode Hypermap.

### `--unpublish`

Remove a previously-published package.

### `--gas-limit`

short: `-g`

Set the gas limit for the transaction.

### `--priority-fee`

short: `-p`

Set the priority fee for the transaction.

### `--fee-per-gas`

short: `-f`

Set the price of gas for the transaction.

### `--mock`

short: `-m`

Run a dry-run, but do not actually submit the transaction.
