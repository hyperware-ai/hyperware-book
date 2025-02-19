# `kit start-package`

short: `kit s`

`kit start-package` installs and starts the indicated package directory (or current working directory) on the given Hyperware node (at `localhost:8080` by default), e.g.,

```
kit start-package foo
```

or

```
kit start-package
```

## Discussion

`kit start-package` injects a built package into the given node and starts it.
`start-package` is designed to be used after a package has been built with [`kit build`](./build.md).
The `pkg/` directory contains metadata about the package for the node as well as the `.wasm` binaries for each process.
The final step in the `build` process is to zip the `pkg/` directory.
`kit start-package` looks for the zipped `pkg/` and then injects a message to the node to start the package.

To both `build` and `start-package` in one command, use `kit build-start-package`.

## Arguments

```
$ kit start-package --help
Start a built Hyprware package

Usage: kit start-package [OPTIONS] [DIR]

Arguments:
  [DIR]  The package directory to start [default: /home/nick]

Options:
  -p, --port <NODE_PORT>  localhost node port; for remote see https://book.hyperware.ai/hosted-nodes.html#using-kit-with-your-hosted-node [default: 8080]
  -h, --help              Print help
```

### Optional positional arg: `DIR`

The package directory to install and start on the node; defaults to current working directory.

### `--port`

short: `-p`

The localhost port of the node; defaults to `8080`.
To interact with a remote node, see [here](../hosted-nodes.md#using-kit-with-your-hosted-node).
