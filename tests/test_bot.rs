use anyhow::{anyhow, Context as _, Result};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use std::fmt::Debug;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread::spawn;
use std::time::Duration;

const TOKEN: &str = "NDU1ODI4MzI3OTAzNzg5MDk2.XbF1eA.ZiYAhMwMpP7FA0UuHt5u_vZsf-4";
const SELF: u64 = 455828327903789096;
const TARGET: u64 = 553215369427550219;
const CHANNEL: u64 = 636866066030788608;

mod general {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn ping_works() -> Result<()> {
        let (send, recv) = channel();
        spawn(|| ::rust_discord_bot::run());
        spawn(move || {
            let mut client = Client::new(
                TOKEN,
                Handler {
                    send: Arc::new(Mutex::new(send)),
                    cmd: "ping".to_string(),
                },
            )
            .context("Error creating test client")
            .unwrap();
            client
                .start()
                .context("Error starting test client")
                .unwrap();
        });
        let result = recv
            .recv_timeout(Duration::new(5, 0))
            .context("Test request timed out")?;
        eq(result, format!("<@{}>: Pong!", SELF))
    }

    #[test]
    fn foo_works() -> Result<()> {
        let (send, recv) = channel();
        spawn(|| ::rust_discord_bot::run());
        spawn(move || {
            let mut client = Client::new(
                TOKEN,
                Handler {
                    send: Arc::new(Mutex::new(send)),
                    cmd: "foo".to_string(),
                },
            )
                .context("Error creating test client")
                .unwrap();
            client
                .start()
                .context("Error starting test client")
                .unwrap();
        });
        let result = recv
            .recv_timeout(Duration::new(5, 0))
            .context("Test request timed out")?;
        eq(result, format!("<@{}>: Bar!", SELF))
    }
}

struct Handler {
    send: Arc<Mutex<Sender<String>>>,
    cmd: String,
}

impl EventHandler for Handler {
    fn message(&self, _ctx: Context, _new_message: Message) {
        if _new_message.author.id.0 != TARGET {
            return;
        }
        let mut _send = self.send.lock();
        let _ = _send.send(_new_message.content);
    }
    fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        let _ = ChannelId(CHANNEL).send_message(_ctx.http, |m|
            m.content(format!("{}{}", ::rust_discord_bot::PREFIX, self.cmd))
        );
    }
}

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
