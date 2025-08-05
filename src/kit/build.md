# `kit build`

short: `kit b`

`kit build` builds the indicated package directory, or the current working directory if none supplied, e.g.,

```
kit build foo
```

or

```
kit build
```

`kit build` builds each process in the package and places the `.wasm` binaries into the `pkg/` directory for installation with [`kit start-package`](./start-package.md).
It automatically detects what language each process is, and builds it appropriately (from amongst the supported `rust`, `python`, and `javascript`).

## Discussion

`kit build` builds a Hyperware package directory.
Specifically, it iterates through all directories within the given package directory and looks for `src/lib.??`, where the `??` is the file extension.
Currently, `rs` is supported, corresponding to processes written in `rust`.
Note that a package may have more than one process and those processes need not be written in the same language.

After compiling each process, it places the output `.wasm` binaries within the `pkg/` directory at the top-level of the given package directory.
Here is an example of what a package directory will look like after using `kit build`:

```
my-rust-chat
├── Cargo.lock
├── Cargo.toml
├── metadata.json
├── pkg
│   ├── manifest.json
│   ├── my-rust-chat.wasm
│   ├── scripts.json
│   └── send.wasm
├── my-rust-chat
│   └── ...
└── send
    └── ...
```

The `pkg/` directory is then zipped and can be injected into the node with [`kit start-package`](./start-package.md).

`kit build` also builds the UI if it is found in `pkg/ui/`.
There must exist a `ui/package.json` file with a `scripts` object containing the following arguments:
```json
"scripts": {
  "build": "tsc && vite build",
  "copy": "mkdir -p ../pkg/ui && rm -rf ../pkg/ui/* && cp -r dist/* ../pkg/ui/",
  "build:copy": "npm run build && npm run copy",
}
```

Additional UI dev info can be found [here](../apis/frontend_development.md).
To both `build` and `start-package` in one command, use `kit build-start-package`.

## Arguments

```
$ kit build --help
Build a Hyperware package

Usage: kit build [OPTIONS] [DIR]

Arguments:
  [DIR]  The package directory to build [default: /home/nick/git/kit]

Options:
      --no-ui
          If set, do NOT build the web UI for the process; no-op if passed with UI_ONLY
      --ui-only
          If set, build ONLY the web UI for the process; no-op if passed with NO_UI
  -i, --include <INCLUDE>
          Build only these processes/UIs (can specify multiple times) [default: build all]
  -e, --exclude <EXCLUDE>
          Build all but these processes/UIs (can specify multiple times) [default: build all]
  -s, --skip-deps-check
          If set, do not check for dependencies
      --features <FEATURES>
          Pass these comma-delimited feature flags to Rust cargo builds
  -p, --port <NODE_PORT>
          localhost node port; for remote see https://book.hyperware.ai/hosted-nodes.html#using-kit-with-your-hosted-node [default: 8080]
  -d, --download-from <NODE>
          Download API from this node if not found
  -w, --world <WORLD>
          Fallback WIT world name
  -l, --local-dependency <DEPENDENCY_PACKAGE_PATH>
          Path to local dependency package (can specify multiple times)
  -a, --add-to-api <PATH>
          Path to file to add to api.zip (can specify multiple times)
      --rewrite
          Rewrite the package (enables `Spawn!()`) [default: don't rewrite]
      --hyperapp
          Build using the Hyperapp framework [default: don't use Hyperapp framework]
  -r, --reproducible
          Make a reproducible build using Docker
  -f, --force
          Force a rebuild
  -v, --verbose
          If set, output stdout and stderr
  -h, --help
          Print help
```

### Optional positional arg: `DIR`

The package directory to build; defaults to the current working directory.

### `--no-ui`

Do not build the web UI for the process.
Does nothing if passed with `--ui-only`.

### `--ui-only`

Build ONLY the UI for a package with a UI.
Otherwise, for a package with a UI, both the package and the UI will be built.

### `--include`

short: `-i`

Only build these processes/UIs within the package.
Can be specified multiple times.

If not specified, build all.

### `--exclude`

short: `-e`

Do not build these processes/UIs within the package.
Can be specified multiple times.

If not specified, build all.

### `--skip-deps-check`

short: `-s`

Don't check for dependencies.

### `--features`

Build the package with the given [cargo features](https://doc.rust-lang.org/cargo/reference/features.html).

Features can be used like shown [here](https://doc.rust-lang.org/cargo/reference/features.html#command-line-feature-options).
Currently the only feature supported system-wide is `simulation-mode`.

### `--port`

short: `-p`

Node to pull dependencies from.
A package's dependencies can be satisfied by either:
1. A live node, the one running at the port given here, or
2. By local dependencies (specified using [`--local-dependency`](#--local-dependency), below).

### `--download-from`

short: `-d`

The mirror to download dependencies from (default: package `publisher`).

### `--world`

short: `-w`

[WIT `world`](../system/process/wit_apis.md) to use.
Not required for Rust processes; use for py or js.

### `--local-dependency`

short: `-l`

A path to a package that satisfies a build dependency.
Can be specified multiple times.

### `--add-to-api`

short: `-a`

A path to a file to include in the API published alongside the package.
Can be specified multiple times.

### `--rewrite`

Rewrite the package, allowing use of the experimental `Spawn!()` macro.

### `--hyperapp`

Specify that the package is an experimental Hyperapp, not a vanilla package.

Hyperapps are still a work-in-progress.
They enable true async/await for sending messages and offer a more "event-based" programming model than vanilla packages.
See [here](https://github.com/hyperware-ai/hyperprocess-macro) for more details.

### `--reproducible`

short: `-r`

Make a reproducible build with a deterministic hash.

Rust does not produce reproducible builds unless:
1. The path of the source is the same.
2. Compiler versions match (e.g., `rustc`, `gcc`, `ld`).
3. `build.rs` is deterministic.

`kit` allows reproducible builds by building the package inside a Docker image, which controls 1 and 2.

The Docker image is published for `x86_64` Linux machines specifically, but also works on `x86_64` MacOS machines.

### `--force`

short: `-f`

Don't check if package doesn't need to be rebuilt: just build it.

### `--verbose`

short: `-v`

Always output stdout and stderr if set.
