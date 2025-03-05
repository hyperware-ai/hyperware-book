# kit chain

short: `kit c`

`kit chain` starts a local fakechain with foundry's [anvil](https://github.com/foundry-rs/foundry/tree/master/crates/anvil), e.g.,

```
kit chain
```

The default port is `8545` and the chain ID is `31337`.

## Discussion

`kit chain` starts an anvil node with the arguments `--load-state kinostate.json`.
This json file includes the HNS (Hyperware Name System) & `app-store` contracts, and is included in the `kit` binary.

The [`kinostate.json`](https://github.com/hyperware-ai/kit/blob/master/src/chain/kinostate) files can be found written into `/tmp/hyperdrive-kit-cache/kinostate-{hash}.json` upon running the command.

Note that while the `hns-indexer` and `app-store` apps in fake nodes use this chain to index events, any events loaded from a json statefile, aren't replayed upon restarting anvil.

## Arguments

```
$ kit chain --help
Start a local chain for development

Usage: kit chain [OPTIONS]

Options:
  -p, --port <PORT>        Port to run the chain on [default: 8545]
  -v, --version <VERSION>  Version of Hyperdrive to run chain for [default: latest] [possible values: latest, v1.1.0]
  -v, --verbose            If set, output stdout and stderr
  -h, --help               Print help
```
