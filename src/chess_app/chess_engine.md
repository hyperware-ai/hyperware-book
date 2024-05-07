# Chess Engine

Chess is a good example for a Kinode application walk-through because:
1. The basic game logic is already readily available.
   There are thousands of high-quality chess libraries across many languages that can be imported into a Wasm app that runs on Kinode.
   We'll be using [pleco](https://github.com/pleco-rs/Pleco)
2. It is a multiplayer game, showing Kinode's p2p communications and ability to serve frontends
3. It is fun!

In `my_chess/Cargo.toml`, which should be in the `my_chess/` process directory inside the `my_chess/` package directory, add `pleco = "0.5"` to your dependencies.
In your `my_chess/src/lib.rs`, replace the existing code with:

```rust
use pleco::Board;
use kinode_process_lib::{await_message, call_init, println, Address};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
});

call_init!(init);
fn init(our: Address) {
    println!("started");

    loop {
        let _ = await_message().map(|message| {
            if !message.is_request() { return };
            println!(
                "{our}: got request from {}: {}",
                message.source().process(),
                String::from_utf8_lossy(message.body())
            );
        });
    }
}
```

Now, we have access to a chess board and can manipulate it easily.

The [pleco docs](https://github.com/pleco-rs/Pleco#using-pleco-as-a-library) show everything you can do using the pleco library.
But this isn't very interesting by itself!
We want to play chess with other people.
Let's start by creating a persisted state for the chess app and a `body` format for sending messages to other nodes.

In `my_chess/src/lib.rs` add the following simple Request/Response interface and persistable game state:
```rust
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
enum ChessRequest {
    NewGame { white: String, black: String },
    Move { game_id: String, move_str: String },
    Resign(String),
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
enum ChessResponse {
    NewGameAccepted,
    NewGameRejected,
    MoveAccepted,
    MoveRejected,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Game {
    pub id: String, // the node with whom we are playing
    pub turns: u64,
    pub board: String,
    pub white: String,
    pub black: String,
    pub ended: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChessState {
    pub games: HashMap<String, Game>, // game is by opposing player id
}
```

Creating explicit `ChessRequest` and `ChessResponse` types is the easiest way to reliably communicate between two processes.
It makes message-passing very simple.
If you get a request, you can deserialize it to `ChessRequest` and ignore or throw an error if that fails.
If you get a response, you can do the same but with `ChessResponse`.
And every request and response that you send can be serialized in kind.
More advanced apps can take on different structures, but a top-level `enum` to serialize/deserialize and match on is always a good idea.

The `ChessState` `struct` shown above can also be persisted using the `set_state` and `get_state` commands exposed by Kinode's runtime.
Note that the `Game` `struct` here has `board` as a `String`.
This is because the `Board` type from pleco doesn't implement `Serialize` or `Deserialize`.
We'll have to convert it to a string using `fen()` before persisting it.
Then, we will convert it back to a `Board` with `Board::from_fen()` when we load it from state.

The code below will contain a version of the `init()` function that creates an event loop and handles ChessRequests.
First, however, it's important to note that these types already bake in some assumptions about our "chess protocol".
Remember, requests can either expect a response, or be fired and forgotten.
Unless a response is expected, there's no way to know if a request was received or not.
In a game like chess, most actions have a logical response.
Otherwise, there's no way to easily alert the user that their counterparty has gone offline, or started to otherwise ignore our moves.
For the sake of the tutorial, there are three kinds of requests and only two expect a response.
In our code, the `NewGame` and `Move` requests will always await a response, blocking until they receive one (or the request times out).
`Resign`, however, will be fire-and-forget.
While a "real" game may prefer to wait for a response, it is important to let one player resign and thus clear their state *without* that resignation being "accepted" by a non-responsive player, so production-grade resignation logic is non-trivial.

An aside: when building consumer-grade peer-to-peer apps, you'll find that there are in fact very few "trivial" interaction patterns.
Something as simple as resigning from a one-on-one game, which would be a single POST request in a client-frontend <> server-backend architecture, requires well-thought-out negotiations to ensure that both players can walk away with a clean state machine, regardless of whether the counterparty is cooperating.
Adding more "players" to the mix makes this even more complex.
To keep things clean, leverage the request/response pattern and the `context` field to store information about how to handle a given response, if you're not awaiting it in a blocking fashion.

Below, you'll find the full code for the CLI version of the app.
You can build it and install it on a node using `kit`.
You can interact with it in the terminal, primitively, like so (assuming your first node is `fake.os` and second is `fake2.os`):
```
m our@my_chess:my_chess:template.os '{"NewGame": {"white": "fake.os", "black": "fake2.os"}}'
m our@my_chess:my_chess:template.os '{"Move": {"game_id": "fake2.os", "move_str": "e2e4"}}'
```
(If you want to make a more ergonomic CLI app, consider parsing `body` as a string...)

As you read through the code, you might notice a problem with this app: there's no way to see your games!
A fun project would be to add a CLI command that shows you, in-terminal, the board for a given `game_id`.
But in the [next chapter](./frontend.md), we'll add a frontend to this app so you can see your games in a browser.

`my_chess/Cargo.toml`:
```toml
[package]
name = "my_chess"
version = "0.1.0"
edition = "2021"

[profile.release]
panic = "abort"
opt-level = "s"
lto = true

[dependencies]
anyhow = "1.0"
base64 = "0.13"
bincode = "1.3.3"
kinode_process_lib = { git = "https://github.com/kinode-dao/process_lib.git", tag = "v0.5.4-alpha" }
pleco = "0.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
url = "*"
wit-bindgen = { git = "https://github.com/bytecodealliance/wit-bindgen", rev = "efcc759" }

[lib]
crate-type = ["cdylib"]

[package.metadata.component]
package = "kinode:process"
```

`my_chess/src/lib.rs`:
```rust
#![feature(let_chains)]
use pleco::Board;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use kinode_process_lib::{
    await_message, call_init, get_typed_state, println, set_state, Address, Message, NodeId, Request, Response,
};

extern crate base64;

// Boilerplate: generate the Wasm bindings for a Kinode app
wit_bindgen::generate!({
    path: "wit",
    world: "process",
});

//
// Our "chess protocol" request/response format. We'll always serialize these
// to a byte vector and send them over `body`.
//

#[derive(Debug, Serialize, Deserialize)]
enum ChessRequest {
    NewGame { white: String, black: String },
    Move { game_id: String, move_str: String },
    Resign(String),
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
enum ChessResponse {
    NewGameAccepted,
    NewGameRejected,
    MoveAccepted,
    MoveRejected,
}

//
// Our serializable state format.
//

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Game {
    pub id: String, // the node with whom we are playing
    pub turns: u64,
    pub board: String,
    pub white: String,
    pub black: String,
    pub ended: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChessState {
    pub games: HashMap<String, Game>, // game is by opposing player id
}

/// Helper function to serialize and save the process state.
fn save_chess_state(state: &ChessState) {
    set_state(&bincode::serialize(&state.games).unwrap());
}

/// Helper function to deserialize the process state. Note that we use a helper function
/// from process_lib to fetch a typed state, which will return None if the state does
/// not exist OR fails to deserialize. In either case, we'll make an empty new state.
fn load_chess_state() -> ChessState {
    match get_typed_state(|bytes| Ok(bincode::deserialize::<HashMap<String, Game>>(bytes)?)) {
        Some(games) => ChessState { games },
        None => ChessState {
            games: HashMap::new(),
        },
    }
}

call_init!(init);
fn init(our: Address) {
    // A little printout to show in terminal that the process has started.
    println!("started");

    // Grab our state, then enter the main event loop.
    let mut state: ChessState = load_chess_state();
    main_loop(&our, &mut state);
}

fn main_loop(our: &Address, state: &mut ChessState) {
    loop {
        // Call await_message() to wait for any incoming messages.
        // If we get a network error, make a print and throw it away.
        // In a high-quality consumer-grade app, we'd want to explicitly handle
        // this and surface it to the user.
        match await_message() {
            Err(send_error) => {
                println!("got network error: {send_error:?}");
                continue;
            }
            Ok(message) => match handle_request(&our, &message, state) {
                Ok(()) => continue,
                Err(e) => println!("error handling request: {:?}", e),
            },
        }
    }
}

/// Handle chess protocol messages from ourself *or* other nodes.
fn handle_request(our: &Address, message: &Message, state: &mut ChessState) -> anyhow::Result<()> {
    // Throw away responses. We never expect any responses *here*, because for every
    // chess protocol request, we *await* its response in-place. This is appropriate
    // for direct node<>node comms, less appropriate for other circumstances...
    if !message.is_request() {
        return Err(anyhow::anyhow!("message was response"));
    }
    // If the request is from another node, handle it as an incoming request.
    // Note that we can enforce the ProcessId as well, but it shouldn't be a trusted
    // piece of information, since another node can easily spoof any ProcessId on a request.
    // It can still be useful simply as a protocol-level switch to handle different kinds of
    // requests from the same node, with the knowledge that the remote node can finagle with
    // which ProcessId a given message can be from. It's their code, after all.
    if message.source().node != our.node {
        // Deserialize the request `body` to our format, and throw it away if it
        // doesn't fit.
        let Ok(chess_request) = serde_json::from_slice::<ChessRequest>(message.body()) else {
            return Err(anyhow::anyhow!("invalid chess request"));
        };
        handle_chess_request(&message.source().node, state, &chess_request)
    // ...and if the request is from ourselves, handle it as our own!
    // Note that since this is a local request, we *can* trust the ProcessId.
    // Here, we'll accept messages from the local terminal so as to make this a "CLI" app.
    } else if message.source().node == our.node
        && message.source().process == "terminal:terminal:sys"
    {
        let Ok(chess_request) = serde_json::from_slice::<ChessRequest>(message.body()) else {
            return Err(anyhow::anyhow!("invalid chess request"));
        };
        handle_local_request(our, state, &chess_request)
    } else {
        // If we get a request from ourselves that isn't from the terminal, we'll just
        // throw it away. This is a good place to put a printout to show that we've
        // received a request from ourselves that we don't know how to handle.
        return Err(anyhow::anyhow!(
            "got request from not-the-terminal, ignoring"
        ));
    }
}

/// handle chess protocol messages from other nodes
fn handle_chess_request(
    source_node: &NodeId,
    state: &mut ChessState,
    action: &ChessRequest,
) -> anyhow::Result<()> {
    println!("handling action from {source_node}: {action:?}");

    // For simplicity's sake, we'll just use the node we're playing with as the game id.
    // This limits us to one active game per partner.
    let game_id = source_node;

    match action {
        ChessRequest::NewGame { white, black } => {
            // Make a new game with source.node
            // This will replace any existing game with source.node!
            if state.games.contains_key(game_id) {
                println!("resetting game with {game_id} on their request!");
            }
            let game = Game {
                id: game_id.to_string(),
                turns: 0,
                board: Board::start_pos().fen(),
                white: white.to_string(),
                black: black.to_string(),
                ended: false,
            };
            // Use our helper function to persist state after every action.
            // The simplest and most trivial way to keep state. You'll want to
            // use a database or something in a real app, and consider performance
            // when doing intensive data-based operations.
            state.games.insert(game_id.to_string(), game);
            save_chess_state(&state);
            // Send a response to tell them we've accepted the game.
            // Remember, the other player is waiting for this.
            Response::new()
                .body(serde_json::to_vec(&ChessResponse::NewGameAccepted)?)
                .send()
        }
        ChessRequest::Move { ref move_str, .. } => {
            // Get the associated game, and respond with an error if
            // we don't have it in our state.
            let Some(game) = state.games.get_mut(game_id) else {
                // If we don't have a game with them, reject the move.
                return Response::new()
                    .body(serde_json::to_vec(&ChessResponse::MoveRejected)?)
                    .send()
            };
            // Convert the saved board to one we can manipulate.
            let mut board = Board::from_fen(&game.board).unwrap();
            if !board.apply_uci_move(move_str) {
                // Reject invalid moves!
                return Response::new()
                    .body(serde_json::to_vec(&ChessResponse::MoveRejected)?)
                    .send();
            }
            game.turns += 1;
            if board.checkmate() || board.stalemate() {
                game.ended = true;
            }
            // Persist state.
            game.board = board.fen();
            save_chess_state(&state);
            // Send a response to tell them we've accepted the move.
            Response::new()
                .body(serde_json::to_vec(&ChessResponse::MoveAccepted)?)
                .send()
        }
        ChessRequest::Resign(_) => {
            // They've resigned. The sender isn't waiting for a response to this,
            // so we don't need to send one.
            match state.games.get_mut(game_id) {
                Some(game) => {
                    game.ended = true;
                    save_chess_state(&state);
                }
                None => {}
            }
            Ok(())
        }
    }
}

/// Handle actions we are performing. Here's where we'll send_and_await various requests.
fn handle_local_request(
    our: &Address,
    state: &mut ChessState,
    action: &ChessRequest,
) -> anyhow::Result<()> {
    match action {
        ChessRequest::NewGame { white, black } => {
            // Create a new game. We'll enforce that one of the two players is us.
            if white != &our.node && black != &our.node {
                return Err(anyhow::anyhow!("cannot start a game without us!"));
            }
            let game_id = if white == &our.node { black } else { white };
            // If we already have a game with this player, throw an error.
            if let Some(game) = state.games.get(game_id)
                && !game.ended
            {
                return Err(anyhow::anyhow!("already have a game with {game_id}"));
            };
            // Send the other player a NewGame request
            // The request is exactly the same as what we got from terminal.
            // We'll give them 5 seconds to respond...
            let Ok(Message::Response { ref body, .. }) = Request::new()
                .target((game_id.as_ref(), our.process.clone()))
                .body(serde_json::to_vec(&action)?)
                .send_and_await_response(5)? else {
                    return Err(anyhow::anyhow!("other player did not respond properly to new game request"))
                };
            // If they accept, create a new game — otherwise, error out.
            if serde_json::from_slice::<ChessResponse>(body)? != ChessResponse::NewGameAccepted {
                return Err(anyhow::anyhow!("other player rejected new game request!"));
            }
            // New game with default board.
            let game = Game {
                id: game_id.to_string(),
                turns: 0,
                board: Board::start_pos().fen(),
                white: white.to_string(),
                black: black.to_string(),
                ended: false,
            };
            state.games.insert(game_id.to_string(), game);
            save_chess_state(&state);
            Ok(())
        }
        ChessRequest::Move { game_id, move_str } => {
            // Make a move. We'll enforce that it's our turn. The game_id is the
            // person we're playing with.
            let Some(game) = state.games.get_mut(game_id) else {
                return Err(anyhow::anyhow!("no game with {game_id}"));
            };
            if (game.turns % 2 == 0 && game.white != our.node)
                || (game.turns % 2 == 1 && game.black != our.node)
            {
                return Err(anyhow::anyhow!("not our turn!"));
            } else if game.ended {
                return Err(anyhow::anyhow!("that game is over!"));
            }
            let mut board = Board::from_fen(&game.board).unwrap();
            if !board.apply_uci_move(move_str) {
                return Err(anyhow::anyhow!("illegal move!"));
            }
            // Send the move to the other player, then check if the game is over.
            // The request is exactly the same as what we got from terminal.
            // We'll give them 5 seconds to respond...
            let Ok(Message::Response { ref body, .. }) = Request::new()
                .target((game_id.as_ref(), our.process.clone()))
                .body(serde_json::to_vec(&action)?)
                .send_and_await_response(5)? else {
                    return Err(anyhow::anyhow!("other player did not respond properly to our move"))
                };
            if serde_json::from_slice::<ChessResponse>(body)? != ChessResponse::MoveAccepted {
                return Err(anyhow::anyhow!("other player rejected our move"));
            }
            game.turns += 1;
            if board.checkmate() || board.stalemate() {
                game.ended = true;
            }
            game.board = board.fen();
            save_chess_state(&state);
            Ok(())
        }
        ChessRequest::Resign(ref with_who) => {
            // Resign from a game with a given player.
            let Some(game) = state.games.get_mut(with_who) else {
                return Err(anyhow::anyhow!("no game with {with_who}"));
            };
            // send the other player an end game request — no response expected
            Request::new()
                .target((with_who.as_ref(), our.process.clone()))
                .body(serde_json::to_vec(&action)?)
                .send()?;
            game.ended = true;
            save_chess_state(&state);
            Ok(())
        }
    }
}
```
