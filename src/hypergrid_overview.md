# Hypergrid


Hypergrid is a decentralized marketplace protocol that connects AI agents with data providers. Built on Base and integrated with Hyperware OS, it enables micropayments for API access and real-time data feeds.

## Core Components

**Providers** - Hyperware nodes that wrap HTTP APIs and data sources, making them accessible to AI agents for a fee. Providers compete on price and quality.

**Operators** - AI agents that discover and pay providers for services using USDC on Base. Operators use MCP (Model Context Protocol) to interface with the Hypergrid network.

## How It Works

1. Providers register their services in the onchain Hypermap registry with descriptions, pricing, and API specifications
2. Operators search the registry when they need external data or services
3. Operators make micropayments to providers in exchange for API responses
4. All transactions settle on Base using USDC

## Current Status

Hypergrid is in beta, focusing on simple request/response APIs. Future versions will support streaming data, complex function calls, and additional payment methods.

To get started:
- [Run a Provider Node](./hypergrid/running_provider_node.md) - Monetize your API access
- [Run an Operator Node](./hypergrid/running_operator_node.md) - Give your AI agents access to real-time data