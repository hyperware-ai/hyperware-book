# Writing Data to ETH

For this cookbook entry, you'll create and deploy a simple `Counter` contract onto a fake local chain, and write a Hyperware app to interact with it.

Using `kit`, create a new project with the `echo` template:

```
kit new counter --template echo
cd counter
```

Now you can create a `contracts` directory within `counter` using `forge init contracts`.
If foundry is not installed, it can be installed with:

```
curl -L https://foundry.paradigm.xyz | bash
```

You can see the simple `Counter.sol` contract in `contracts/src/Counter.sol`:

```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Counter {
    uint256 public number;

    function setNumber(uint256 newNumber) public {
        number = newNumber;
    }

    function increment() public {
        number++;
    }
}
```

You can write a simple script to deploy it at a predictable address, create the file `scripts/Deploy.s.sol`:

```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console, VmSafe} from "forge-std/Script.sol";
import {Counter} from "../src/Counter.sol";

contract DeployScript is Script {
    function setUp() public {}

    function run() public {
        VmSafe.Wallet memory wallet = vm.createWallet(
            0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
        );
        vm.startBroadcast(wallet.privateKey);

        Counter counter = new Counter();
        console.log("Counter deployed at address: ", address(counter));
        vm.stopBroadcast();
    }
}
```

Now boot a fakechain, either with `kit f` which boots one at port 8545 in the background, or with `kit c`.

Then you can run:

```
forge script --rpc-url http://localhost:8545 script/Deploy.s.sol --broadcast
```

You'll see a printout that looks something like this:

```
== Logs ==
  Counter deployed at address:  0x0DCd1Bf9A1b36cE34237eEaFef220932846BCD82
```

Great!
Now you'll write the Hyperware app to interact with it.

You're going to use some functions from the `eth` library in `hyperware_process_lib`:

```rust
use hyperware_process_lib::eth;
```

Also you'll need to request the capability to message `eth:distro:sys`, so you can add it to the `request_capabilities` field in `pkg/manifest.json`.

Next, you'll need some sort of ABI in order to interact with the contracts.
The crate `alloy-sol-types` gives us a solidity macro to either define contracts from JSON, or directly in the rust code.
You'll add it to `counter/Cargo.toml`:

```
alloy-sol-types = "0.7.6"
```

Now, importing the following types from the crate:

```rust
use alloy_sol_types::{sol, SolCall, SolValue};
```

You can do the following:

```rust
sol! {
    contract Counter {
        uint256 public number;

        function setNumber(uint256 newNumber) public {
            number = newNumber;
        }

        function increment() public {
            number++;
        }
    }
}
```

Pretty cool, you can now do things like define a setNumber() call just like this:

```rust
let contract_call = setNumberCall { newNumber: U256::from(58)};
```

Start with a simple setup to read the current count, and print it out!

```rust
use hyperware_process_lib::{await_message, call_init, eth::{Address as EthAddress, Provider, TransactionInput, TransactionRequest, U256}, println, Address, Response};
use alloy_sol_types::{sol, SolCall, SolValue};
use std::str::FromStr;

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
});

sol! {
    contract Counter {
        uint256 public number;

        function setNumber(uint256 newNumber) public {
            number = newNumber;
        }

        function increment() public {
            number++;
        }
    }
}

pub const COUNTER_ADDRESS: &str = "0x0DCd1Bf9A1b36cE34237eEaFef220932846BCD82";

fn read(provider: &Provider) -> anyhow::Result<U256> {
    let counter_address = EthAddress::from_str(COUNTER_ADDRESS).unwrap();
    let count = Counter::numberCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .to(counter_address)
        .input(count.into());

    let x = provider.call(tx, None);

    match x {
        Ok(b) => {
            let number = U256::abi_decode(&b, false)?;
            println!("current count: {:?}", number.to::<u64>());
            Ok(number)
        }
        Err(e) => {
            println!("error getting current count: {:?}", e);
            Err(anyhow::anyhow!("error getting current count: {:?}", e))
        }
    }
}

call_init!(init);
fn init(our: Address) {
    println!("begin");

    let provider = Provider::new(31337, 5);

    let _count = read(&provider);

    loop {
        match handle_message(&our, &provider) {
            Ok(()) => {}
            Err(e) => {
                println!("error: {:?}", e);
            }
        };
    }
}
```

Now add the 2 writes that are possible: increment() and setNumber(newNumber).
To do this, you'll need to define a wallet, and import a few new crates:

```
alloy-primitives = "0.7.6"
alloy-rlp = "0.3.5"
alloy = { version = "0.1.2", features = [
    "network",
    "signers",
    "signer-local",
    "consensus",
    "rpc-types"
]}
```

You'll also define a simple enum so you can call the program with each of the 3 actions:

```rust
#[derive(Debug, Deserialize, Serialize)]
pub enum CounterAction {
    Increment,
    Read,
    SetNumber(u64),
}
```

When creating a wallet, you can use one of the funded addresses on the anvil fakechain, like so:

```rust
use alloy::{
    consensus::{SignableTransaction, TxEip1559, TxEnvelope},
    network::{eip2718::Encodable2718, TxSignerSync},
    primitives::TxKind,
    rpc::types::eth::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use alloy_rlp::Encodable;
use alloy_sol_types::{sol, SolCall, SolValue};
use hyperware_process_lib::{
    await_message, call_init,
    eth::{Address as EthAddress, Provider, U256},
    println, Address, Response,
};

use serde::{Deserialize, Serialize};
use std::str::FromStr;

let wallet =
    PrivateKeySigner::from_str("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
        .unwrap();
```

First, branching on the enum type `Increment`, call the increment() function with no arguments:

```rust
    CounterAction::Increment => {
        let increment = Counter::incrementCall {}.abi_encode();
        let nonce = provider
            .get_transaction_count(wallet.address(), None)
            .unwrap()
            .to::<u64>();

        let mut tx = TxEip1559 {
            chain_id: 31337,
            nonce: nonce,
            to: TxKind::Call(EthAddress::from_str(COUNTER_ADDRESS).unwrap()),
            gas_limit: 15000000,
            max_fee_per_gas: 10000000000,
            max_priority_fee_per_gas: 300000000,
            input: increment.into(),
            ..Default::default()
        };

        let sig = wallet.sign_transaction_sync(&mut tx)?;

        let signed = TxEnvelope::from(tx.into_signed(sig));

        let mut buf = vec![];
        signed.encode_2718(&mut buf);

        let tx_hash = provider.send_raw_transaction(buf.into());
        println!("tx_hash: {:?}", tx_hash);

    }
```

Note how you can do `provider.get_transaction_count()` to get the current nonce of the account!

Next, do the same for `setNumber`!

```rust
    CounterAction::SetNumber(n) => {
        let set_number = Counter::setNumberCall {
            newNumber: U256::from(n),
        }
        .abi_encode();

        let nonce = provider
            .get_transaction_count(wallet.address(), None)
            .unwrap()
            .to::<u64>();

        let mut tx = TxEip1559 {
            chain_id: 31337,
            nonce: nonce,
            to: TxKind::Call(EthAddress::from_str(COUNTER_ADDRESS).unwrap()),
            gas_limit: 15000000,
            max_fee_per_gas: 10000000000,
            max_priority_fee_per_gas: 300000000,
            input: set_number.into(),
            ..Default::default()
        };
        let sig = wallet.sign_transaction_sync(&mut tx)?;
        let signed = TxEnvelope::from(tx.into_signed(sig));

        let mut buf = vec![];
        signed.encode(&mut buf);

        let tx_hash = provider.send_raw_transaction(buf.into());
        println!("tx_hash: {:?}", tx_hash);
    }
```

Nice!
Putting it all together, you can build and start the package on a fake node (`kit f` if you don't have one running), `kit bs`.

```
fake.os > m our@counter:counter:template.os '{"SetNumber": 55}'
counter:template.os: tx_hash: Ok(0x5dba574f2a9a2c095cee960868433e23c64b685966fba57568c4d6a0fd99ef6c)

fake.os > m our@counter:counter:template.os "Read"
counter:template.os: current count: 55

fake.os > m our@counter:counter:template.os "Increment"
counter:template.os: tx_hash: Ok(0xc38ee230c2605c294a37794244334c0d20a5b5e090704b34f4a7998021418d7b)

fake.os > m our@counter:counter:template.os "Read"
counter:template.os: current count: 56
```

You can find these steps outlined by commit in the counter [example repo!](https://github.com/barraguda/counterexample)
