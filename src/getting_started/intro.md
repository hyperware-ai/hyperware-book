# Introduction

The Hyperware Book describes the Hyperware stack, both in conceptual and practical terms.

Hyperware is a decentralized operating system, peer-to-peer app framework, and node network designed to simplify the development and deployment of decentralized applications.
It is also a _sovereign cloud computer_, in that Hyperware can be deployed anywhere and act as a server controlled by anyone.
Ultimately, Hyperware facilitates the writing and distribution of software that runs on privately-held, personal server nodes or node clusters.

Hyperware eliminates boilerplate and reduces the complexity of p2p software development by providing four basic and necessary primitives:

| Primitive        | Description                                                                                    |
| ---------------- | ---------------------------------------------------------------------------------------------- |
| Networking       | Passing messages from peer to peer.                                                            |
| Identity         | Linking permanent system-wide identities to individual nodes.                                  |
| Data Persistence | Storing data and saving it in perpetuity.                                                      |
| Global State     | Reading shared global state (blockchain) and composing actions with this state (transactions). |

The focus of this book is how to build and deploy applications on Hyperware.

## Architecture Overview

Applications are composed of processes, which hold state and pass messages.
Hyperware's kernel handles the startup and teardown of processes, as well as message-passing between processes, both locally and across the network.
Processes are programs compiled to Wasm, which export a single `init()` function.
They can be started once and complete immediately, or they can run "forever".

Hyperware peers are identified by their onchain username in the "HNS": Hyperware Name System, which is modeled after ENS.
The modular architecture of the HNS allows for any Ethereum NFT, including ENS names themselves, to generate a unique Hyperware identity once it is linked to a HNS entry.

Data persistence and blockchain access, as fundamental primitives for p2p apps, are built directly into the kernel.
The filesystem is abstracted away from the developer, and data is automatically persisted across an arbitrary number of encrypted remote backups as configured at the user-system-level.
Accessing global state in the form of the Ethereum blockchain is now trivial, with chain reads and writes handled by built-in system runtime modules.

Several other I/O primitives also come with the kernel: an HTTP server and client framework, as well as a simple key-value store.
Together, these tools can be used to build performant and self-custodied full-stack applications.

Finally, by the end of this book, you will learn how to deploy applications to the Hyperware network, where they will be discoverable and installable by any user with a node.
