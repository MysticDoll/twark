use crate::functions::handle_functions;
use crate::irc::IRCMessage;
use ws::{Handler, Handshake, Message, Result, Sender};

pub struct TwarkClient {
    out: Sender,
}

impl Handler for TwarkClient {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let token = std::env::var("TWARK_ACCESS_TOKEN").map_err(|e| {
            let kind = ws::ErrorKind::Custom(Box::new(e));
            ws::Error::new(kind, "Could not get oauth token")
        })?;
        let channel = std::env::var("TWARK_CHANNEL").unwrap_or("mysticdoll_".to_owned());
        self.out
            .send(format!("PASS oauth:{}", token))
            .and_then(|_| self.out.send(format!("NICK {}", channel)))
            .and_then(|_| self.out.send(format!("JOIN #{}", channel)))
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        for m in IRCMessage::from_ws_msg(msg).iter() {
            let params = m
                .params()
                .first()
                .map(ToString::to_string)
                .ok_or(())
                .map_err(|_| {
                    ws::Error::new(
                        ws::ErrorKind::Internal,
                        "Could not get first param of IRC command",
                    )
                })?;
            if let Some(cmd) = handle_functions(params) {
                println!("debug command:{:?}", cmd);
                if let Err(e) = cmd.exec(&self) {
                    println!("command execution error: {}", e);
                }
            };
        }
        Ok(())
    }
}

impl TwarkClient {
    pub fn new(out: Sender) -> Self {
        Self { out }
    }

    pub fn send(&self, msg: &str) -> Result<()> {
        self.out.send(format!("PRIVMSG #mysticdoll_ :{}", msg))
    }
}
