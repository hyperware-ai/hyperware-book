# Welcome to the Hyperware Book

This book is the comprehensive technical guide for developers building on Hyperware. Hyperware is AI and Dapp development infrastructure designed for building at lightspeed. It provides a "batteries included" framework to simplify the creation and deployment of decentralized applications, including those that leverage AI.

At its core, Hyperware enables you to write, run, and distribute software from privately held personal nodes. The system is composed of a Rust kernel, Wasm processes, and an on-chain namespace called Hypermap, which collectively handle networking, identity, data persistence, and global state.

### Key Sections of the Documentation

This guide is organized into several major sections to help you find the information you need:

* **Getting Started:** The perfect place to begin. It includes a [Quick Start guide](./quick_start.md), an [Introduction to core concepts](./intro.md), and [Installation instructions](./install.md).

* **[System Components](../system/system_components.md):** A deeper dive into the architecture of Hyperware, describing processes, the networking protocol, the HTTP server and client, file and database systems, and more.

* **[Process Standard Library](../process_stdlib/overview.md):** An introduction to the `process_lib`, a standard library that makes writing Rust applications on Hyperware simple and efficient.

* **[Kit: Development Toolkit](../kit/kit-dev-toolkit.md):** A reference for `kit`, the command-line interface (CLI) toolkit designed to make developing on Hyperware ergonomic and straightforward.

* **[Tutorials and Guides](../my_first_app/build_and_deploy_an_app.md):**
    * **[My First Hyperware Application](../my_first_app/build_and_deploy_an_app.md):** A step-by-step tutorial to set up your environment, learn the `kit` tools, and build and deploy your first application from the ground up.
    * **[In-Depth Guide: Chess App](../chess_app/chess_app.md):** A more advanced guide that walks through building a simple, complete chess application on Hyperware.

* **[Cookbook](../cookbook/cookbook.md):** A collection of practical, bite-sized recipes and how-to guides for common programming techniques and patterns you'll encounter while developing for Hyperware.

* **[API Reference](../apis/api_reference.md):** Detailed documentation on the Hyperware runtime module APIs. Note that for most use cases, it's recommended to use the higher-level functions provided in the [Process Standard Library](../process_stdlib/overview.md).

* **[Hosted Nodes User Guide](../hosted-nodes.md):** A guide for developers using the hosted node service provided by Sybil Technologies, including how to manage your node and use `kit` with it remotely.

* **[Audits and Security](../audits-and-security.md):** Information regarding security audits of the Hyperware stack.

* **[Glossary](../glossary.md):** A helpful reference that defines the various technical terms used throughout the Hyperware ecosystem.

Happy building!
