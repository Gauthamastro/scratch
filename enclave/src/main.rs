use jsonrpc_ws_server::jsonrpc_core::*;
use jsonrpc_ws_server::ServerBuilder;

fn main() {
    let mut io = IoHandler::new();
    io.add_sync_method("say_hello", |_params| {
        Ok(Value::String("hello".into()))
    });

    let server = ServerBuilder::new(io)
        .start(&"0.0.0.0:3030".parse().unwrap())
        .expect("Server must start with no issues");

    server.wait().unwrap()
}