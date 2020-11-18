use crate::client::TwarkClient;
use crate::functions::Command;
use std::process::Command as OSCommand;
use ws::Result;

#[derive(Debug)]
pub struct FallGuysInjector {
    injector: String,
}

impl Command for FallGuysInjector {
    fn exec(&self, client: &TwarkClient, args: Vec<String>) -> Result<()> {
        if (args.len() != 1) {
            return client.send("This command will take only one argument. !fallguys up/down/left/right/dive/jump are allowed.");
        }
        let command = args.get(0).cloned().unwrap_or("".to_owned());
        match command.as_str() {
            "up" => {}
            "down" => {}
            "left" => {}
            "right" => {}
            "dive" => {}
            "jump" => {}
            "emote1" => {}
            "emote2" => {}
            "emote3" => {}
            "emote4" => {}
            "help" => return client.send(
                "type !fallguys up|down|left|right|dive|jump|emote{1-4} to controll my fallguy.",
            ),
            _ => {
                return client
                    .send("Invalid command, !fallguys up/down/left/right/dive/jump are allowed.")
            }
        }

        let injector = self.injector.clone();

        std::thread::spawn(move || {
            OSCommand::new(&injector)
                .arg(command)
                .spawn()
                .and_then(|_| {
                    println!("success excution");
                    Ok(())
                })
                .map_err(|e| {
                    println!("failed exectuion {:?}", e);
                    e
                });
        });
        Ok(())
    }
}

impl FallGuysInjector {
    pub fn new(injector: &str) -> Self {
        Self {
            injector: injector.to_owned(),
        }
    }
}
