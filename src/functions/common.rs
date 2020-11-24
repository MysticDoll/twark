use crate::client::TwarkClient;
use crate::functions::Command;
use ws::Result;

#[derive(Debug, Clone)]
pub struct Echo {}

impl Command for Echo {
    fn exec(&self, client: &TwarkClient, args: Vec<String>, user: String) -> Result<()> {
        client.send(&args.join(" "))
    }
}

impl Echo {
    pub fn new() -> Self {
        Self {}
    }
}
