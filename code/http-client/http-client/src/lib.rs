/// Simple example of sending an HTTP request.
/// Usage:
/// ```
/// # Start node.
/// kit f
///
/// # Start package from a new terminal.
/// kit bs http-client
/// ```
use hyperware_process_lib::{call_init, http, println, Address};

wit_bindgen::generate!({
    path: "../target/wit",
    world: "process-v1",
});

const URL: &str =
    "https://raw.githubusercontent.com/hyperware-ai/hyperware-wit/master/hyperware.wit";

call_init!(init);
fn init(_our: Address) {
    println!("begin");

    let url = url::Url::parse(URL).expect("failed to parse url");
    let response =
        http::client::send_request_await_response(http::Method::GET, url, None, 5, vec![]);

    match response {
        Err(e) => panic!("request failed: {e:?}"),
        Ok(r) => {
            let r = String::from_utf8(r.body().clone()).expect("couldn't read response");
            println!("{r}");
        }
    }
}
