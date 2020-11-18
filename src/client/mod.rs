use crate::functions::Command;
use crate::irc::IRCMessage;
use regex::Regex;
use std::collections::HashMap;
use ws::{Handler, Handshake, Message, Result as WSResult, Sender};

pub struct TwarkClient {
    out: Sender,
    commands: HashMap<String, Box<dyn Command>>,
}

impl Handler for TwarkClient {
    fn on_open(&mut self, _: Handshake) -> WSResult<()> {
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

    fn on_message(&mut self, msg: Message) -> WSResult<()> {
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
            if let Some((cmd, args)) = self.handle_commands(params) {
                println!("debug command:{:?}", cmd);
                if let Err(e) = cmd.exec(&self, args) {
                    println!("command execution error: {}", e);
                }
            };
        }
        Ok(())
    }
}

#[derive(Debug)]
enum ParseError {
    InvalidSource(String),
    CaptureFailed,
    RegexError,
    InvalidMessageFormat,
    CommandNotFound,
}

impl TwarkClient {
    pub fn new(out: Sender, commands: HashMap<String, Box<dyn Command>>) -> Self {
        Self { out, commands }
    }

    pub fn send(&self, msg: &str) -> WSResult<()> {
        self.out.send(format!("PRIVMSG #mysticdoll_ :{}", msg))
    }

    fn validate(&self, raw: String) -> Result<(&Box<dyn Command>, Vec<String>), ParseError> {
        let re = Regex::new(r"(?P<channel>#\w+) :(?P<message>.+$)")
            .map_err(|_| ParseError::RegexError)?;
        re.captures(&raw)
            .ok_or(ParseError::InvalidSource(raw.clone()))
            .and_then(|cap| cap.name("message").ok_or(ParseError::CaptureFailed))
            .and_then(|msg| {
                let re = Regex::new(r"!(?P<command>\w+) (?P<args>.+)")
                    .map_err(|_| ParseError::RegexError)?;
                re.captures(msg.as_str())
                    .ok_or(ParseError::InvalidMessageFormat)
            })
            .and_then(|cap| {
                let command = cap
                    .name("command")
                    .map(|s| s.as_str().to_string())
                    .ok_or(ParseError::InvalidMessageFormat)?;
                let capture_args = cap
                    .name("args")
                    .map(|s| s.as_str().to_string())
                    .ok_or(ParseError::InvalidMessageFormat)?;
                let args: Vec<String> = capture_args.split(" ").map(ToString::to_string).collect();

                self.commands
                    .get(&command)
                    .map(|c| (c, args))
                    .ok_or(ParseError::CommandNotFound)
            })
    }

    pub fn handle_commands(&self, raw: String) -> Option<(&Box<dyn Command>, Vec<String>)> {
        println!("raw: {}", raw);
        match self.validate(raw) {
            Ok(result) => Some(result),
            Err(e) => {
                println!("command parse error: {:?}", e);
                None
            }
        }
    }
}
