# Process Semantics

## Overview

Hyperware processes are the building blocks for peer-to-peer applications.
The Hyperware runtime (e.g. Hyperdrive) handles message-passing between processes, plus the startup and teardown of said processes.
This section describes the message design as it relates to processes.

Each process instance has a globally unique identifier, or `Address`, composed of four elements.
- the publisher's node (containing a-z, 0-9, `-`, and `.`)
- the package name (containing a-z, 0-9, and `-`)
- the process name  (containing a-z, 0-9, and `-`).
  This may be a developer-selected string or a randomly-generated number as string.
- the node the process is running on (often your node: `our` for short).

A package is a set of one or more processes and optionally GUIs: a package is synonymous with an App and packages are distributed via the built-in App Store.

The way these elements compose is the following:

[`PackageId`s](https://docs.rs/hyperware_process_lib/latest/hyperware_process_lib/hyperware/process/standard/struct.PackageId.html) look like:
```
[package-name]:[publisher-node]
my-cool-software:publisher-node.os
```

[`ProcessId`s](https://docs.rs/hyperware_process_lib/latest/hyperware_process_lib/hyperware/process/standard/struct.ProcessId.html) look like:
```
[process-name]:[package-name]:[publisher-node]
process-one:my-cool-software:publisher-node.os
8513024814:my-cool-software:publisher-node.os
```

Finally, [`Address`es](https://docs.rs/hyperware_process_lib/latest/hyperware_process_lib/hyperware/process/standard/struct.Address.html) look like:

```
[node]@[process-name]:[package-name]:[publisher-node]
some-user.os@process-one:my-cool-software:publisher-node.os
```

--------

Processes are compiled to Wasm.
They can be started once and complete immediately, or they can run forever.
They can spawn other processes, and coordinate in arbitrarily complex ways by passing messages to one another.

## Process State

Hyperware processes can be stateless or stateful.
In this case, state refers to data that is persisted between process instantiations.
Nodes get turned off, intentionally or otherwise.
The kernel handles rebooting processes that were running previously, but their state is not persisted by default.

Instead, processes elect to persist data, and what data to persist, when desired.
Data might be persisted after every message ingested, after every X minutes, after a certain specific event, or never.
When data is persisted, the kernel saves it to our abstracted filesystem, which not only persists data on disk, but also across arbitrarily many encrypted remote backups as configured at the user-system-level.

This design allows for ephemeral state that lives in-memory, or truly permanent state, encrypted across many remote backups, synchronized and safe.

Processes have access to multiple methods for persisting state:

- Saving a state object with the system calls available to every process, seen [here](../../cookbook/save_state.md).
- [Using the virtual filesystem to read and write from disk](../files.md), useful for persisting state that needs to be shared between processes.
- Using the [SQLite](../../apis/sqlite.md) or [KV](../../apis/kv.md) APIs to persist state in a database.

## Requests and Responses

Processes communicate by passing messages, of which there are two kinds: `Request`s and `Response`s.

#### Addressing

When a `Request` or `Response` is received, it has an attached `Address`, which consists of: the source of the message, including the ID of the process that produced the `Request`, as well as the ID of the originating node.

The integrity of a source `Address` differs between local and remote messages.
If a message is local, the validity of its source is ensured by the local kernel, which can be trusted to label the `ProcessId` and node ID correctly.
If a message is remote, only the node ID can be validated (via networking keys associated with each node ID).
The `ProcessId` comes from the remote kernel, which could claim any `ProcessId`.
This is fine — merely consider remote `ProcessId`s a *claim* about the initiating process rather than an infallible ID like in the local case.

#### Please Respond

`Request`s can be issued at any time by a running process.
A `Request` can optionally expect a `Response`.
If it does, the `Request` will be retained by the kernel, along with an optional `context` object created by the `Request`s issuer.
A `Request` will be considered outstanding until the kernel receives a matching `Response`, at which point that `Response` will be delivered to the requester alongside the optional `context`.
`context`s allow `Response`s to be disambiguated when handled asynchronously, for example, when some information about the `Request` must be used in handling the `Response`.
`Response`s can also be handled in an async-await style, discussed [below](#awaiting-a-response).

`Request`s that expect a `Response` set a timeout value, after which, if no `Response` is received, the initial `Request` is returned to the process that issued it as an error.
[Send errors](#errors) are handled in processes alongside other incoming messages.

##### Inheriting a `Response`

If a process receives a `Request`, that doesn't mean it must directly issue a `Response`.
The process can instead issue `Request`(s) that "inherit" from the incipient `Request`, continuing its lineage.
If a `Request` does not expect a `Response` and also "inherits" from another `Request`, `Response`s to the child `Request` will be returned to the parent `Request`s issuer.
This allows for arbitrarily complex `Request`-`Response` chains, particularly useful for "middleware" processes.

There is one other use of inheritance, discussed below: [passing data in `Request` chains cheaply](#inheriting-a-lazy_load_blob).

##### Awaiting a Response

When sending a `Request`, a process can await a `Response` to that specific `Request`, queueing other messages in the meantime.
Awaiting a `Response` leads to easier-to-read code:
* The `Response` is handled in the next line of code, rather than in a separate iteration of the message-handling loop
* Therefore, the `context` need not be set.

The downside of awaiting a `Response` is that all other messages to a process will be queued until that `Response` is received and handled.
As such, certain applications lend themselves to blocking with an await, and others don't.
A rule of thumb is: await `Response`s (because simpler code) except when a process needs to performantly handle other messages in the meantime.

For example, if a `file-transfer` process can only transfer one file at a time, `Request`s can simply await `Response`s, since the only possible next message will be a `Response` to the `Request` just sent.
In contrast, if a `file-transfer` process can transfer more than one file at a time, `Request`s that await `Response`s will block others in the meantime; for performance it may make sense to write the process fully asynchronously, i.e. without ever awaiting.
The constraint on awaiting is a primary reason why it is desirable to [spawn child processes](#spawning-child-processes).
Continuing the `file-transfer` example, by spawning one child "worker" process per file to be transferred, each worker can use the await mechanic to simplify the code, while not limiting performance.

There is more discussion of child processes [here](../../cookbook/manage_child_processes.md), and an example of them in action in the [`file-transfer` cookbook](../../cookbook/file_transfer.md).

#### Message Structure

Messages, both `Request`s and `Response`s, can contain arbitrary data, which must be interpreted by the process that receives it.
The structure of a message contains hints about how best to do this:

First, messages contain a field labeled `body`, which holds the actual contents of the message.
In order to cross the [Wasm boundary](https://component-model.bytecodealliance.org/design/why-component-model.html) and be language-agnostic, the `body` field is simply a byte vector.
To achieve composability between processes, a process should be very clear, in code and documentation, about what it expects in the `body` field and how it gets parsed, usually into a language-level struct or object.

A message also contains a `lazy_load_blob`, another byte vector, used for opaque, arbitrary, or large data.
`lazy_load_blob`s, along with being suitable location for miscellaneous message data, are an optimization for shuttling messages across the Wasm boundary.
Unlike other message fields, the `lazy_load_blob` is only moved into a process if explicitly called with (`get_blob()`).
Processes can thus choose whether to ingest a `lazy_load_blob` based on the `body`/`metadata`/`source`/`context` of a given message.
`lazy_load_blob`s hold bytes alongside a `mime` field for explicit process-and-language-agnostic format declaration, if desired.
See [inheriting a `lazy_load_blob`](#inheriting-a-lazy_load_blob) for a discussion of why lazy loading is useful.

Lastly, messages contain an optional `metadata` field, expressed as a JSON-string, to enable middleware processes and other such things to manipulate the message without altering the `body` itself.

##### Inheriting a `lazy_load_blob`

The reason `lazy_load_blob`s are not automatically loaded into a process is that an intermediate process may not need to access the blob.
If process A sends a message with a blob to process B, process B can send a message that inherits to process C.
If process B does not attach a new `lazy_load_blob` to that inheriting message, the original blob from process A will be attached and accessible to C.

For example, consider again the file-transfer process discussed [above](#awaiting-a-response).
Say one node, `send.os`, is transferring a file to another node, `recv.os`.
The process of sending a file chunk will look something like:
1. `recv.os` sends a `Request` for chunk N
2. `send.os` receives the `Request` and itself makes a `Request` to the filesystem for the piece of the file
3. `send.os` receives a `Response` from the filesystem with the piece of the file in the `lazy_load_blob`;
   `send.os` sends a `Response` that inherits the blob back to `recv.os` without itself having to load the blob, saving the compute and IO required to move the blob across the Wasm boundary.

This is the second functionality of inheritance; the first is discussed above: [eliminating the need for bucket-brigading of `Response`s](#inheriting-a-response).

#### Errors

Messages that result in networking failures, like `Request`s that timeout, are returned to the process that created them as an error.
There are only two kinds of send errors: `Offline` and `Timeout`.
Offline means a message's remote target definitively cannot be reached.
Timeout is multi-purpose: for remote nodes, it may indicate compromised networking; for both remote and local nodes, it may indicate that a process is simply failing to respond in the required time.

A send error will return to the originating process the initial message, along with any optional `context`, so that the process can re-send the message, crash, or otherwise handle the failure as the developer desires.
If the error results from a `Response`, the process may optionally try to re-send a `Response`: it will be directed towards the original outstanding `Request`.

### Capabilities

Processes must acquire capabilities from the kernel in order to perform certain operations.
Processes themselves can also produce capabilities in order to give them to other processes.
For more information about the general capabilities-based security paradigm, see the paper "Capability Myths Demolished".

The kernel gives out capabilities that allow a process to message another *local* process.
It also gives a capability allowing processes to send and receive messages over the network.
A process can optionally mark itself as `public`, meaning that it can be messaged by any *local* process regardless of capabilities.

[See the capabilities chapter for more details.](./capabilities.md)

### Spawning child processes

A process can spawn "child" processes — in which case the spawner is known as the "parent".
As discussed [above](#awaiting-a-response), one of the primary reasons to write an application with multiple processes is to enable both simple code and high performance.

Child processes can be used to:
1. Run code that may crash without risking crashing the parent
2. Run compute-heavy code without blocking the parent
3. Run IO-heavy code without blocking the parent
4. Break out code that is more easily written with awaits to avoid blocking the parent

There is more discussion of child processes [here](../../cookbook/manage_child_processes.md), and an example of them in action in the [`file-transfer` cookbook](../../cookbook/file_transfer.md).

### Conclusion

This is a high-level overview of process semantics.
In practice, processes are combined and shared in **packages**, which are generally synonymous with **apps**.

#### Wasm and Hyperware

It's briefly discussed here that processes are compiled to Wasm.
The details of this are not covered in the Hyperware Book, but can be found in the documentation for [Hyperdrive](https://github.com/hyperware-ai/hyperdrive), which uses [Wasmtime](https://wasmtime.dev/), a WebAssembly runtime, to load, execute, and provide an interface for the subset of Wasm components that are valid Hyperware processes.

Wasm runs modules by default, or components, as described [here](https://component-model.bytecodealliance.org/design/why-component-model.html): components are just modules that follow some specific format.
Hyperware processes are Wasm components that have certain imports and exports so they can be run by Hyperware.
Pragmatically, processes can be compiled using the [`kit`](https://github.com/hyperware-ai/kit) developer toolkit, see documentation [here](../../kit/kit-dev-toolkit.md).


The long term goal of Hyperware is, using [WASI](https://wasi.dev/), to provide a secure, sandboxed environment for Wasm components to make use of the kernel features described in this document.
Further, Hyperware has a Virtual File System ([VFS](../files.md)) which processes can interact with to access files on a user's machine, and in the future WASI could also expose access to the filesystem for Wasm components directly.
