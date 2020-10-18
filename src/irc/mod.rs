use getset::Getters;
use regex::Regex;
use ws::Message;

#[derive(Debug, Getters)]
pub struct IRCMessage {
    #[getset(get = "pub")]
    prefix: String,
    #[getset(get = "pub")]
    command: String,
    #[getset(get = "pub")]
    params: Vec<String>,
}

impl IRCMessage {
    pub fn from_ws_msg(msg: Message) -> Vec<IRCMessage> {
        lazy_static! {
            static ref IRCREX: Regex =
                Regex::new(r":(?P<prefix>[^:]+)\s(?P<command>[a-zA-Z0-9]+)\s(?P<params>[^\n]+)")
                    .unwrap();
        }

        msg.as_text()
            .unwrap_or("")
            .split("\r\n")
            .filter_map(|s| {
                let msg = s.to_owned();
                IRCREX.captures(&msg).map(|cap| {
                    let prefix = cap
                        .name("prefix")
                        .map(|c| c.as_str().to_owned())
                        .unwrap_or("".to_owned());
                    let command = cap
                        .name("command")
                        .map(|c| c.as_str().to_owned())
                        .unwrap_or("".to_owned());
                    let params = cap
                        .name("params")
                        .map(|c| c.as_str().to_owned())
                        .unwrap_or("".to_owned())
                        .split("\n")
                        .map(|s| s.to_owned())
                        .collect::<Vec<String>>();
                    IRCMessage {
                        prefix,
                        command,
                        params,
                    }
                })
            })
            .collect()
    }
}
