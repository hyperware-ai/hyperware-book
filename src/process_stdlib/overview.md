# `process_lib` Overview

This page serves as an introduction to the [process standard library](https://github.com/hyperware-ai/process_lib), which makes writing Rust apps on Hyperware easy.
The full documentation can be found [here](https://docs.rs/hyperware_process_lib), and the crate lives [here](https://crates.io/crates/hyperware_process_lib).

In your `Cargo.toml` file, use a version tag like this:
```toml
hyperware_process_lib = "1.0.2"
```

**Make sure to use a recent version of the `process_lib` while the system is in beta and active development.**

The major version of the `process_lib` will always match the major version of Hyperware.

Since Hyperware apps use the [WebAssembly Component Model](https://component-model.bytecodealliance.org/), they are built on top of a [WIT](https://component-model.bytecodealliance.org/design/wit.html) (Wasm Interface Type) [package](https://github.com/hyperware-ai/hyperdrive-wit).
[`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen) is used to generate Rust code from a WIT file.
The generated code then contains the core types and functions that are available to all Hyperware apps.

However, the types themselves are unwieldy to use directly, and runtime modules present APIs that can be drastically simplified by using helper functions and types in the process standard library.

Almost all code examples in this book make use of the `process_lib`.
For specific examples of its usage, check out the [docs](https://docs.rs/hyperware_process_lib) or just follow the tutorials later in this book.
