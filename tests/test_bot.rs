
use std::fmt::Debug;

const TOKEN: &str = "NDU1ODI4MzI3OTAzNzg5MDk2.XbF1eA.ZiYAhMwMpP7FA0UuHt5u_vZsf-4";
const SELF: u64 = 455828327903789096;
const TARGET: u64 = 553215369427550219;
const CHANNEL: u64 = 636866066030788608;

mod general {
    use super::{eq, SELF, TOKEN, TARGET, CHANNEL};
    use serenity::Client;
    use std::sync::mpsc::channel;
    use std::sync::mpsc::{Sender, Receiver};
    use std::sync::{Arc, Mutex, RwLock};
    use std::thread::spawn;
    use std::time::Duration;
    use lazy_static::lazy_static;
    use anyhow::{Context as _, Result};
    use serenity::client::{Context, EventHandler};
    use serenity::model::channel::Message;
    use serenity::model::gateway::Ready;
    use serenity::model::id::ChannelId;

    lazy_static! {
        static ref COMMAND_CHAN: (Sender<String>, Receiver<String>) = Arc::new(RwLock::new(channel()));
        static ref RESULT_CHAN: (Sender<Result<String>>, Receiver<Result<String>>) = Arc::new(RwLock::new(channel()));
        static ref MAIN_BOT: Client = Arc::new(Mutex::new(spawn(|| ::rust_discord_bot::run())));
        static ref TEST_BOT: Client = Arc::new(Mutex::new(spawn(|| inner())));
    }

    fn inner() {
        fn _inner() -> Result<()> {
            let mut client = Client::new(
                TOKEN,
                Handler,
            )
            .context("Error creating test client")?;
            client.start().context("Error starting test client")?;
            Ok(())
        }

        _inner().unwrap_or_else(|err| {
            let _ = RESULT_CHAN.read().0.send(Err(err));
        })
    }

    fn work(cmd: &'static str, res: &'static str) -> Result<()> {
        MAIN_BOT.lock().thread();
        TEST_BOT.lock().thread();

        let _ = COMMAND_CHAN.0.send(cmd);

        let result = RESULT_CHAN.read()
            .1
            .recv_timeout(Duration::new(5, 0))
            .context("Test request timed out")?
            .context("Result channel returned an error")?;

        eq(result, format!("<@{}>: {}!", SELF, res))
    }

    #[test]
    fn ping_works() -> Result<()> {
        work("ping", "Pong")
    }

    #[test]
    fn foo_works() -> Result<()> {
        work("foo", "Bar")
    }

    struct Handler;

    impl EventHandler for Handler {
        fn message(&self, _ctx: Context, _new_message: Message) {
            if _new_message.author.id.0 != TARGET {
                return;
            }

            let _ = RESULT_CHAN.read().0.send(Ok(_new_message.content));
        }

        fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
            fn _ready(_ctx: Context) -> Result<()> {
                let cmd = COMMAND_CHAN.read().1
                    .recv_timeout(Duration::new(1, 0))
                    .context("Could not read requested command: channel timed out")?;

                ChannelId(CHANNEL)
                    .send_message(_ctx.http, |m| {
                        m.content(format!("{}{}", ::rust_discord_bot::PREFIX, cmd))
                    })
                    .context("Failed to send command as a message")?;

                Ok(())
            }

            if let Err(err) = _ready(_ctx) {
                let _ = RESULT_CHAN.read().0.send(Err(err));
            };
        }
    }
}

use anyhow::{Result, anyhow};

pub fn eq<T: Eq + Debug>(lhs: T, rhs: T) -> Result<()> {
    if lhs == rhs {
        return Ok(());
    } else {
        return Err(anyhow!(
            "assert failed: `{:?}` different from `{:?}`",
            lhs,
            rhs
        ));
    }
}
