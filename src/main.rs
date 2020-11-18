#[macro_use]
extern crate lazy_static;

mod client;
mod functions;
mod irc;

use crate::client::TwarkClient;
use crate::functions::FunctionBuilder;
use ws::connect;
fn main() {
    if let Err(e) = connect("wss://irc-ws.chat.twitch.tv:443", move |out| {
        let mut commands = FunctionBuilder::new();
        commands.register("echo", Box::new(crate::functions::common::Echo::new()));
        commands.register(
            "fallguys",
            Box::new(crate::functions::fallguys::FallGuysInjector::new(
                "/home/mysticdoll/fallguys_injector_executor.sh",
            )),
        );

        TwarkClient::new(out, commands.build())
    }) {
        panic!("Failed to create WebSocket: {:?}", e);
    };
}
