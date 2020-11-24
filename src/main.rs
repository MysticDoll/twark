#[macro_use]
extern crate lazy_static;

mod client;
mod functions;
mod irc;

use crate::client::TwarkClient;
use crate::functions::Command;
use std::collections::HashMap;
use url::Url;
fn main() {
    connect();
    loop {}
}

pub fn connect() {
    let mut ws = ws::WebSocket::new(|out| TwarkClient::new(out, commands())).unwrap();

    std::thread::spawn(move || {
        ws.connect(Url::parse("wss://irc-ws.chat.twitch.tv:443").unwrap())
            .unwrap();
        if let Err(e) = ws.run() {
            println!("socket connect failed {}", e);
        };
    });
}

fn commands() -> HashMap<String, Box<dyn Command + Send>> {
    let mut commands: HashMap<String, Box<dyn Command + Send>> = HashMap::new();
    commands.insert(
        "echo".to_owned(),
        Box::new(crate::functions::common::Echo::new()),
    );
    commands.insert(
        "fallguys".to_owned(),
        Box::new(crate::functions::fallguys::FallGuysInjector::new(
            "/home/mysticdoll/fallguys_injector_executor.sh",
        )),
    );
    commands
}
