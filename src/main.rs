#[macro_use]
extern crate lazy_static;

mod client;
mod functions;
mod irc;

use crate::client::TwarkClient;
use ws::connect;
fn main() {
    if let Err(e) = connect("wss://irc-ws.chat.twitch.tv:443", move |out| {
        TwarkClient::new(out)
    }) {
        panic!("Failed to create WebSocket: {:?}", e);
    };
}
