use crate::client::TwarkClient;
use regex::Regex;
use ws::Result as WSResult;

#[derive(Debug)]
pub struct Command {
    name: String,
    args: Vec<String>,
}

impl Command {
    fn new<I: Into<Vec<String>>>(name: String, args: I) -> Self {
        Self {
            name,
            args: args.into(),
        }
    }

    pub fn exec(&self, client: &TwarkClient) -> WSResult<()> {
        if self.name == "echo" {
            client.send(&self.args.join(" "))
        } else {
            println!("command not found: {}", self.name);
            Ok(())
        }
    }
}
#[derive(Debug)]
enum ParseError {
    InvalidSource(String),
    CaptureFailed,
    RegexError,
    InvalidMessageFormat,
}

fn validate(raw: String) -> Result<Command, ParseError> {
    let re =
        Regex::new(r"(?P<channel>#\w+) :(?P<message>.+$)").map_err(|_| ParseError::RegexError)?;
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

            Ok(Command::new(command, args))
        })
}

pub fn handle_functions(raw: String) -> Option<Command> {
    println!("raw: {}", raw);
    match validate(raw) {
        Ok(command) => Some(command),
        Err(e) => {
            println!("command parse error: {:?}", e);
            None
        }
    }
}
