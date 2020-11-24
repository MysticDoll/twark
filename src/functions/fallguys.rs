use crate::client::TwarkClient;
use crate::functions::Command;
use std::process::Command as OSCommand;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};
use ws::Result;

lazy_static! {
    static ref LAST_EXECUTEE: Arc<RwLock<Option<(String, SystemTime)>>> =
        Arc::new(RwLock::new(None));
}

#[derive(Debug)]
pub struct FallGuysInjector {
    injector: String,
}

impl Command for FallGuysInjector {
    fn exec(&self, client: &TwarkClient, args: Vec<String>, user: String) -> Result<()> {
        if args.len() != 1 {
            return client.send("This command will take only one argument. up|down|left|right|dive|jump|emote{1-4} are allowed.");
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
            _ => return client.send(
                "Invalid command, !fallguys up|down|left|right|dive|jump|emote{1-4} are allowed.",
            ),
        }

        let injector = self.injector.clone();

        std::thread::spawn(move || {
            if !LAST_EXECUTEE
                .read()
                .and_then(|t| {
                    Ok((*t)
                        .as_ref()
                        .filter(|(u, t)| {
                            u == &user
                                && SystemTime::now()
                                    .duration_since(*t)
                                    .unwrap_or(Duration::new(100, 0))
                                    .as_secs()
                                    < 5
                        })
                        .is_some())
                })
                .unwrap_or(false)
            {
                if let Err(e) = OSCommand::new(&injector)
                    .arg(command)
                    .stdin(std::process::Stdio::null())
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn()
                {
                    println!("failed exectuion {:?}", e);
                } else {
                    println!("success excution");
                    if let Ok(mut l) = LAST_EXECUTEE.write() {
                        *l = Some((user, SystemTime::now()));
                    }
                };
            } else {
                println!("prevent from the last user who has executed since 5 secs");
            }
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
