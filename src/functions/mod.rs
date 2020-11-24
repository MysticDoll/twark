pub(crate) mod common;
pub(crate) mod fallguys;

use crate::client::TwarkClient;
use std::collections::HashMap;
use ws::Result as WSResult;

pub trait Command: std::fmt::Debug {
    fn exec(&self, client: &TwarkClient, args: Vec<String>, user: String) -> WSResult<()>;
}

pub struct FunctionBuilder {
    commands: HashMap<String, Box<dyn Command + Send>>,
}

impl FunctionBuilder {
    pub fn new() -> Self {
        FunctionBuilder {
            commands: HashMap::new(),
        }
    }

    pub fn register<'a>(&'a mut self, key: &str, command: Box<dyn Command + Send>) -> &'a mut Self {
        self.commands.insert(key.to_owned(), command);
        self
    }

    pub fn build(self) -> HashMap<String, Box<dyn Command + Send>> {
        self.commands
    }
}
