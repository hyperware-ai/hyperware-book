# Install `kit`

These documents describe some ways you can use these tools, but do not attempt to be completely exhaustive.
You are encouraged to make use of the `--help` flag, which can be used for the top-level `kit` command:

```
$ kit --help
Development toolkit for Hyperware

Usage: kit <COMMAND>

Commands:
  boot-fake-node       Boot a fake node for development [aliases: f]
  boot-real-node       Boot a real node [aliases: e]
  build                Build a Hyperware package [aliases: b]
  build-start-package  Build and start a Hyperware package [aliases: bs]
  chain                Start a local chain for development [aliases: c]
  connect              Connect (or disconnect) a ssh tunnel to a remote server
  dev-ui               Start the web UI development server with hot reloading (same as `cd ui && npm i && npm run dev`) [aliases: d]
  inject-message       Inject a message to a running node [aliases: i]
  new                  Create a Hyperware template package [aliases: n]
  publish              Publish or update a package [aliases: p]
  remove-package       Remove a running package from a node [aliases: r]
  reset-cache          Reset kit cache (Hyperdrive binaries, logs, etc.)
  run-tests            Run Hyperware tests [aliases: t]
  setup                Fetch & setup kit dependencies
  start-package        Start a built Hyprware package [aliases: s]
  update               Fetch the most recent version of kit
  view-api             Fetch the list of APIs or a specific API [aliases: v]
  help                 Print this message or the help of the given subcommand(s)

Options:
  -v, --version  Print version
  -h, --help     Print help
```

or for any of the subcommands, e.g.:

```
kit new --help
```

The first chapter of the [My First Hyperware App tutorial](../my_first_app/chapter_1.md) shows the `kit` tools in action.

## Getting kit

`kit` requires Rust.
To get `kit`, run

```bash
# Install Rust if you don't have it.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install `kit`.
cargo install --git https://github.com/hyperware-ai/kit --locked
```

To update, run that same command or

```
kit update
```

You can find the source for `kit` at [https://github.com/hyperware-ai/kit](https://github.com/hyperware-ai/kit).

You can find a video guide that walks through setting up `kit` [here](https://www.youtube.com/watch?v=N8B_s_cm61k).

## Logging

Logs are printed to the terminal and stored, by default, at `/tmp/hyperdrive-kit-cache/logs/log.log`.
The default logging level is `info`.
Other valid logging levels are: `debug`, `warning` and `error`.

These defaults can be changed by setting environment variables:

Environment Variable | Description
-------------------- | -----------
`KIT_LOG_PATH`       | Set log path (default `/tmp/hyperdrive-kit-cache/logs/log.log`).
`RUST_LOG`           | Set log level (default `info`).

For example, in Bash:

```bash
export RUST_LOG=info
```
