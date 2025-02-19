# Quick Start

## Run two fake nodes and chat between them

```sh
# Get Rust and `kit` Hyperware development tools
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --git https://github.com/hyperware-ai/kit --locked

# Start two fake nodes, each in a new terminal on ports 8080 and 8081:
## First new terminal:
kit boot-fake-node

## Second new terminal:
kit boot-fake-node --home /tmp/hyperware-fake-node-2 --port 8081 --fake-node-name fake2

# Back in the original terminal that is not running a fake node:
## Create and build a chat app from a template:
kit new my-chat-app
kit build my-chat-app

## Load the chat app into each node & start it:
kit start-package my-chat-app
kit start-package my-chat-app --port 8081

## Chat between the nodes:
kit inject-message my-chat-app:my-chat-app:template.os '{"Send": {"target": "fake2.dev", "message": "hello from the outside world"}}'
kit inject-message my-chat-app:my-chat-app:template.os '{"Send": {"target": "fake.dev", "message": "replying from fake2.dev using first method..."}}' --node fake2.dev
kit inject-message my-chat-app:my-chat-app:template.os '{"Send": {"target": "fake.dev", "message": "and second!"}}' -p 8081

# Or, from the terminal running one of the fake nodes:
## First fake node terminal:
m our@my-chat-app:my-chat-app:template.os '{"Send": {"target": "fake2.dev", "message": "hello world"}}'

## Second fake node terminal:
m our@my-chat-app:my-chat-app:template.os '{"Send": {"target": "fake.dev", "message": "wow, it works!"}}'
```

## Next steps

The first chapter of the [My First Hyperware Application](../my_first_app/build_and_deploy_an_app.md) tutorial is a more detailed version of this Quick Start.
Alternatively, you can learn more about `kit` in the [`kit` documentation](../kit/kit-dev-toolkit.md).

If instead, you want to learn more about high-level concepts, start with the [Introduction](./intro.md) and work your way through the book in-order.
