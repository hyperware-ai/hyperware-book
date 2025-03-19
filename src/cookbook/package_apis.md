# Exporting & Importing Package APIs

Hyperware packages can export APIs, as discussed [here](../system/process/wit_apis.md).
Processes can also import APIs.
These APIs can consist of types as well as functions.
This recipe focuses on:
1. Simple examples of exporting and importing APIs (find the full code [here](https://github.com/hyperware-ai/hyperware-book/tree/main/code/remote-file-storage)).
2. Demonstrations of `kit` tooling to help build and export or import APIs.

## Exporting an API

APIs are defined in a WIT file.
A brief summary of more [thorough discussion](../system/process/wit_apis.md#high-level-overview) is provided here:
1. [WIT (Wasm Interface Type)](https://component-model.bytecodealliance.org/design/wit.html) is a language to define APIs.
   Hyperware packages may define a WIT API by placing a WIT file in the top-level `api/` directory.
2. Processes define a [WIT `interface`](https://component-model.bytecodealliance.org/design/wit.html#interfaces).
3. Packages define a [WIT `world`](https://component-model.bytecodealliance.org/design/wit.html#worlds).
4. APIs define their own WIT `world` that `export`s at least one processes WIT `interface`.

### Example: Remote File Storage Server

#### WIT API

```rust
{{#includehidetest ../../code/remote-file-storage/server/api/server:template.os-v0.wit}}
```

As summarized [above](#exporting-an-api), the `server` process defines an `interface` of the same name, and the package defines the `world server-template-dot-os-v0`.
The API is defined by `server-template-dot-os-api-v0`: the functions in the `server` interface are defined [below](#api-function-definitions) by `wit_bindgen::generate!()`ing that `world`.

The example covered in this document shows an `interface` that has functions exported.
However, for `interface`s that export only types, no `-api-` world (like `server-template-dot-os-api-v0` here) is required.
Instead, the WIT API alone suffices to export the types, and the importer writes a `world` that looks like [this, below](#wit-api-1).
For example, consider the `chat` template's `api/` and its usage in the `test/` package:
```
kit n my-chat
cat my-chat/api/my-chat\:template.os-v0.wit
cat my-chat/test/my-chat-test/api/my-chat-test\:template.os-v0.wit
```

#### API Function Definitions

```rust
{{#includehidetest ../../code/remote-file-storage/server/server-api/src/lib.rs}}
```

Functions must be defined if exported in an interface, as they are here.
Functions are defined by creating a directory just like a process directory, but with a slightly different `lib.rs` (see [directory structure](https://github.com/hyperware-ai/hyperware-book/tree/main/code/remote-file-storage/server/server-api)).
Note the definition of `struct Api`, the `impl Guest for Api`, and the `export!(Api)`:
```rust
{{#include ../../code/remote-file-storage/server/server-api/src/lib.rs:93:94}}

...

{{#include ../../code/remote-file-storage/server/server-api/src/lib.rs:115:116}}
```
The `export`ed functions are defined here.
Note the function signatures match those defined in the WIT API.

#### Process

A normal process: the [`server`](https://github.com/hyperware-ai/hyperware-book/tree/main/code/remote-file-storage/server/server/src/lib.rs) handles Requests from consumers of the API.

```rust
{{#includehidetest ../../code/remote-file-storage/server/server/src/lib.rs}}
```

## Importing an API

### Dependencies

#### `metadata.json`

The [`metadata.json`](https://github.com/hyperware-ai/hyperware-book/blob/main/code/remote-file-storage/client/metadata.json#L14-L16) file has a `properties.dependencies` field.
When the `dependencies` field is populated, [`kit build`](../kit/build.md) will fetch that dependency from either:
1. [A livenet Hyperware hosting it](#../kit/build.md#--port).
2. [A local path](#../kit/build.#--local-dependency).
3. An HTTP endpoint (coming soon).

#### Fetching Dependencies

`kit build` resolves dependencies in a few ways.

The first is from a livenet Hyperware node hosting the depenency.
This method requires a [`--port`](#../kit/build.md#--port) (or `-p` for short) flag when building a package that has a non-empty `dependencies` field.
That `--port` corresponds to the node hosting the API dependency.

To host an API, your node must either:
1. Have that package downloaded by the `app-store`.
2. Be a live node, in which case it will attempt to contact the publisher of the package, and download the package.
Thus, when developing on a fake node, you must first build and start any dependencies on your fake node before building packages that depend upon them: see [usage example below](#remote-file-storage-usage-example).

The second way `kit build` resolves dependencies is with a [local path](#../kit/build.#--local-dependency).

### Example: Remote File Storage Client Script

#### WIT API

```rust
{{#includehidetest ../../code/remote-file-storage/client/api/client:template.os-v0.wit}}
```

#### Process

The `client` process here is a [script](../cookbook/writing_scripts.md).
In general, importers of APIs are just processes, but in this case, it made more sense for this specific functionality to write it as a script.
The `Args` and `Command` `struct`s set up command-line parsing and are unrelated to the WIT API.

```rust
{{#includehidetest ../../code/remote-file-storage/client/client/src/lib.rs}}
```

## Remote File Storage Usage Example

### Build

```
# Start fake node to host server.
kit f

# Start fake node to host client.
kit f -o /tmp/hyperware-fake-node-2 -p 8081 -f fake2.dev

# Build & start server.
## Note starting is required because we need a deployed copy of server's API in order to build client.
## Below is it assumed that `hyperware-book` is the CWD.
kit bs src/../code/remote-file-storage/server

# Build & start client.
## Here the `-p 8080` is to fetch deps for building client (see the metadata.json dependencies field).
kit b src/../code/remote-file-storage/client -p 8080 && kit s src/../code/remote-file-storage/client -p 8081
```

An alternative way to satisfy the `server` dependency of `client`:
```
## The `-l` satisfies the dependency using a local path.
kit b src/../code/remote-file-storage/client -l src/../code/remote-file-storage/server
```

### Usage

```
# In fake2.dev terminal:
## Put a file onto fake.dev.
client:client:template.os put-file fake.dev -p client:template.os/pkg/manifest.json -n manifest.json

## Check the file was Put properly.
client:client:template.os list-files fake.dev

## Put a different file.
client:client:template.os put-file fake.dev -p client:template.os/pkg/scripts.json -n scripts.json

## Check the file was Put properly.
client:client:template.os list-files fake.dev

## Read out a file.
client:client:template.os get-file fake.dev -n scripts.json
```
