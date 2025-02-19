# Networking Protocol

### 1. Protocol Overview and Motivation

The Hyperware networking protocol is designed to be performant, reliable, private, and peer-to-peer, while still enabling access for nodes without a static public IP address.

The networking protocol is NOT designed to be all-encompassing, that is, the only way that two nodes will ever communicate.
Many Hyperware runtimes (including Hyperdrive) will provide userspace access to HTTP server/client capabilities, TCP sockets, and much more.
Some applications will choose to use such facilities to communicate.
This networking protocol is merely a common language that every Hyperware node is guaranteed to speak.
For this reason, it is the protocol on which system processes will communicate, and it will be a reasonable default for most applications.

In order for nodes to attest to their identity without any central authority, all networking information is made available onchain.
Networking information can take two forms: direct or routed.
The former allows for completely direct peer-to-peer connections, and the latter allows nodes without a physical network configuration that permits direct connections to route messages through a peer.

The networking protocol can and will be implemented in multiple underlying protocols.
Since the protocol is encrypted, a secure underlying connection with TLS or HTTPS is never necessary.
WebSockets are prioritized to make purely in-browser nodes a possibility.
The other transport protocols with slots in the onchain identity data structure are: TCP, UDP, and WebTransport.

Currently, only WebSockets and TCP are implemented in the runtime.
As part of the protocol, nodes identify the supported transport protocols of their counterparty and choose the optimal one to use.
Even nodes that do not share common transport protocols may communicate via routers.
Direct nodes must have at least one transport protocol in common.
It is strongly recommended that all nodes support WebSockets, including future browser-based nodes and mobile-nodes.

### 2. Onchain Networking Information

All nodes must publish an Ed25519 EdDSA networking public key onchain using the protocol registry contract.
A new key transaction may be posted at any time, but because agreement on networking keys is required to establish a connection and send messages between nodes, changes to onchain networking information will temporarily disrupt networking.
Therefore, all nodes must have robust access to the onchain PKI, meaning: multiple backup options and multiple pathways to read onchain data.
Because it may take time for a new networking key to proliferate to all nodes, (anywhere from seconds to days depending on chain indexing access) a node that changes its networking key should expect downtime immediately after doing so.

Nodes that wish to make direct connections must post an IP and port onchain.
This is done by publishing `note` keys in [Hypermap](../getting_started/Hypermap.md).
In particular, the networking protocol expects the following pattern of data available:

1. A `~net-key` note AND
2. Either:
   a. A `~routers` note OR
   b. An `~ip` note AND at least one of:
      - `~tcp-port` note
      - `~udp-port` note
      - `~ws-port` note
      - `~wt-port` note

Nodes with onchain networking information (an IP address and at least one port) are referred to as **direct** nodes, and ones without are referred to as **indirect** or **routed** nodes.

If a node is indirect, it must initiate a connection with at least one of its allowed routers in order to begin networking.
Until such a connection is successfully established, the indirect node is offline.
In practice, an indirect node that wants reliable access to the network should (1) have many routers listed onchain and (2) connect to as many of them as possible on startup.
In order to acquire such routers in practice, a node will likely need to provide some payment or service to them.

### 3. Protocol Selection

When one node seeks to send a message to another node, it first checks to see if it has an existing route to send it on.
If it does, that route is used.
If not, the node will use the information available about the other node to try and establish a route.

If the target node is direct, the route may be direct, using one of the available transport methods.
If a direct node presents multiple ports using notes in Hypermap, the priority is currently:

1. TCP
2. WS

As more protocols are supported by various runtimes, this priority list will expand.

Once a transport method is selected, if the connection fails, the target will be considered offline.
A node does not need to try every route available: if a direct node presents a port, it must service connections on that method to be considered online.

If the target node is indirect, the route must be established through one of their routers.
As many routers as can be attempted within the message's timeout may be tried.
The selection of which routers to try in what order is implementation-specific.
When a router is being attempted, the transport method will be determined as in a standard direct connection, described above.
If a router is offline, the next router is tried.
If no routers are online, the indirect node will be considered offline.

### 4. WebSockets Protocol

This protocol does not make use of any [WebSocket frames](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API/Writing_WebSocket_servers#exchanging_data_frames) other than Binary, Ping, and Pong.
Pings should be responded to with a Pong.
These are only used to keep the connection alive.
All content is sent as Binary frames.
Binary frames in the current protocol version (1) are limited to 10MB. This includes the full serialized `KernelMessage`.

All data structures are serialized and deserialized using [MessagePack](https://msgpack.org/index.html).

#### 4.1. Establishing a Connection

The WebSockets protocol uses the [Noise Protocol Framework](http://www.noiseprotocol.org/noise.html) to encrypt all messages end-to-end.
The parameters used are `Noise_XX_25519_ChaChaPoly_BLAKE2s`.

Using the XX pattern means following this interactive pattern:
```
  -> e
  <- e, ee, s, es
  -> s, se
```

The initiator is the node that is trying to establish a connection.

**If the target is direct**, the intiator uses the IP and port provided onchain to establish a WebSocket connection.
If the connection fails, the target is considered offline.

**If the target is indirect**, the initiator uses the IP and port of one of the target's routers to establish a WebSocket connection.
If a given router is unreachable, or fails to comport to the protocol, others should be tried until they are exhausted or too much time has passed (subject to the specific implementation).
If this process fails, the target is considered offline.

**If the target is indirect**, before beginning the XX handshake pattern, the initiator sends a `RoutingRequest` to the target.

```rust
pub struct RoutingRequest {
    pub protocol_version: u8,
    pub source: String,
    pub signature: Vec<u8>,
    pub target: String,
}
```
The `protocol_version` is the current protocol version, which is 1.
The `source` is the initiator's node ID, as provided onchain.
The `signature` must be created by the initiator's networking public key.
The content is the routing target's node ID (i.e., the node which the initiator would like to establish an e2e encrypted connection with) concatenated with the router's node ID (i.e., the node which the initiator is sending the `RoutingRequest` to, which will serve as a router for the connection if it accepts).
The `target` is the routing target's node ID that must be signed above.

Once a connection is established, the initiator sends an `e` message, containing an empty payload.

The target responds with the `e, ee, s, es` pattern, including a `HandshakePayload` serialized with MessagePack.

```rust
struct HandshakePayload {
    pub protocol_version: u8,
    pub name: String,
    pub signature: Vec<u8, Global>,
    pub proxy_request: bool,
}
```
The current `protocol_version` is 1.
The `name` is the name of the node, as provided onchain.
The `signature` must be created by the node's networking public key, visible onchain.
The content is the public key they will use to encrypt messages on this connection.
How often this key changes is implementation-specific but should be frequent.
The `proxy_request` is a boolean indicating whether the initiator is asking for routing service to another node.

As the target, or receiver of the new connection, `proxy_request` will always be false. This field is only used by the initiator.

Finally, the initiator responds with the `s, se` pattern, including a `HandshakePayload` of their own.

After this pattern is complete, the connection switches to transport mode and can be used to send and receive messages.

#### 4.2. Sending Messages

Every message sent over the connection is a `KernelMessage`, serialized with MessagePack, then encrypted using the keys exchanged in the Noise protocol XX pattern, sent in a single Binary WebSockets message.

```rust
struct KernelMessage {
    pub id: u64,
    pub source: Address,
    pub target: Address,
    pub rsvp: Rsvp,
    pub message: Message,
    pub lazy_load_blob: Option<LazyLoadBlob>
}
```

See [`Address`](https://docs.rs/hyperware_process_lib/latest/hyperware_process_lib/hyperware/process/standard/struct.Address.html), [`Rsvp`](https://github.com/hyperware-ai/hyperdrive/blob/5504f2a6c1b28eb5102aee9a56d2a278f1e5a2dd/lib/src/core.rs#L891-L894), [`Message`](https://docs.rs/hyperware_process_lib/latest/hyperware_process_lib/hyperware/process/standard/enum.Message.html),and [`LazyLoadBlob`](https://docs.rs/hyperware_process_lib/latest/hyperware_process_lib/hyperware/process/standard/struct.LazyLoadBlob.html) data types.

#### 4.3. Receiving Messages

When listening for messages, the protocol may ignore messages other than Binary, but should also respond to Ping messages with Pongs.

When a Binary message is received, it should first be decrypted using the keys exchanged in the handshake exchange, then deserialized as a `KernelMessage`.
If this fails, the message should be ignored and the connection must be closed.

Successfully decrypted and deserialized messages should have their `source` field checked for the correct node ID and then passed to the kernel.

#### 4.4. Closing a Connection

A connection can be intentionally closed by any party, at any time.
Other causes of connection closure are discussed in this section.

All connection errors must result in closing a connection.

Failure to send a message must be treated as a connection error.

Failure to decrypt or deserialize a message must be treated as a connection error.

If a `KernelMessage`'s source is not the node ID which the message recipient is expecting, it must be treated as a connection error.

These behaviors are necessary since they indicate that the networking information of a counterparty may have changed and a new connection must be established using the new data onchain.

Connections may be closed due to inactivity or load-balancing. This behavior is implementation-specific.

### 5. TCP Protocol

The TCP protocol is largely the same as the WebSockets protocol but without the use of Binary frames.
`KernelMessage`s are instead streamed.
More documentation to come — for now, read [source here](https://github.com/hyperware-ai/hyperdrive/blob/main/hyperware/src/net/tcp/utils.rs).

### 6. Connection Maintenance and Errors

The system's networking module seeks to abstract away the many complexities of p2p networking from app developers.
To this end, it reduces all networking issues to either Offline or Timeout.

Messages do not have to expect a response.
If no response is expected, a networking-level offline or timeout error may still be thrown.
Local messages will only receive timeout errors if they expect a response.

If a peer is direct, i.e. they have networking information published onchain, determining their offline status is simple: try to create a connection and send a message; if the underlying transport protocol experiences any errors while doing so, throw an 'offline' error.
If a message is not responded to before the timeout counter expires, it will throw a timeout.

If a peer is indirect, i.e. they have routers, multiple attempts must be made before either an offline error is thrown.
The specific implementation of the protocol may vary in this regard (e.g. it may try to connect to all routers, or limit the number of attempts to a subset of routers).
As with direct peers, if a message is not responded to before the timeout counter expires, it will throw a timeout.

