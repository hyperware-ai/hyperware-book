use crate::hyperware::process::tester::{
    FailResponse, Request as TesterRequest, Response as TesterResponse, RunRequest,
};

use hyperware_process_lib::{await_message, call_init, println, Address, Request, Response};

mod tester_lib;

wit_bindgen::generate!({
    path: "target/wit",
    world: "spawn-test-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [PartialEq, serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn run_test(our: &Address) -> anyhow::Result<()> {
    let message = await_message()?;

    if !message.is_request() {
        fail!("spawn-test");
    }
    let source = message.source();
    if our.node != source.node {
        fail!("spawn-test");
    }

    let TesterRequest::Run(RunRequest {
        input_node_names: node_names,
        ..
    }) = message.body().try_into()?;
    if node_names.len() != 1 {
        fail!("spawn-test");
    }

    let our_test_process_address = Address {
        node: our.node.clone(),
        process: "spawned-child-process:spawn:template.os".parse()?,
    };
    let response = Request::new()
        .target(our_test_process_address)
        .body(vec![])
        .send_and_await_response(5)??;

    let Ok(Ok::<(), ()>(())) = serde_json::from_slice(response.body()) else {
        fail!("spawn-test");
    };

    Response::new()
        .body(TesterResponse::Run(Ok(())))
        .send()
        .unwrap();

    Ok(())
}

call_init!(init);
fn init(our: Address) {
    println!("begin");

    match run_test(&our) {
        Ok(()) => {}
        Err(e) => {
            println!("spawn-test: error: {e:?}");
            fail!("spawn-test");
        }
    };
}
