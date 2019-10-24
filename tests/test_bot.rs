use anyhow::{anyhow, Context as _, Result};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use std::fmt::Debug;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread::spawn;
use std::time::Duration;

const TOKEN: &str = "NDU1ODI4MzI3OTAzNzg5MDk2.XbF1eA.ZiYAhMwMpP7FA0UuHt5u_vZsf-4";
const SELF: u64 = 455828327903789096;
const TARGET: u64 = 553215369427550219;
const CHANNEL: u64 = 636866066030788608;

#[test]
fn test_bot() -> Result<()> {
    let (send_command, recv_command) = channel();
    let (send_result, recv_result) = channel();
    let send_result2 = send_result.clone();

    spawn(move || run_main_bot(send_result));
    spawn(move || run_test_bot(recv_command, send_result2));

    let chan = (send_command, recv_result);

    test_ping(&chan)?;
    test_foo(&chan)?;

    Ok(())
}

fn test_ping(chan: &Chan) -> Result<()> {
    simple(chan, "ping".to_string(), format!("<@{}>: Pong!", SELF))
}

fn test_foo(chan: &Chan) -> Result<()> {
    simple(chan, "foo".to_string(), format!("<@{}>: Bar!", SELF))
}

type Chan = (Sender<String>, Receiver<Result<String>>);

fn simple(chan: &Chan, cmd: String, expected: String) -> Result<()> {
    chan.0
        .send(cmd)
        .context("Command channel closed")?;
    let result = chan.1
        .recv_timeout(Duration::new(5, 0))
        .context("Test request timed out")?
        .context("An error was reported through the result channel")?;
    assert_eq(result, expected)?;
    Ok(())
}

fn run_main_bot(send_result: Sender<Result<String>>) {
    report_failure(
        send_result,
        ::rust_discord_bot::run(),
        "Cannot set up the main bot",
    );
}

fn run_test_bot(recv_command: Receiver<String>, send_result: Sender<Result<String>>) {
    fn _run(recv_command: Receiver<String>, send_result: Sender<Result<String>>) -> Result<()> {
        let mut client = Client::new(
            TOKEN,
            Handler {
                recv_command: Arc::new(Mutex::new(recv_command)),
                send_result: Arc::new(Mutex::new(send_result)),
            },
        )
        .context("Cannot create test client")?;
        client.start().context("Cannot start test client")?;
        Ok(())
    }

    report_failure(
        send_result.clone(),
        _run(recv_command, send_result),
        "Cannot set up the test bot",
    );
}

fn report_failure(chan: Sender<Result<String>>, op: Result<()>, msg: &'static str) {
    if let Err(err) = op {
        let _ = chan.send(Err(anyhow!(msg).context(err)));
    }
}

struct Handler {
    recv_command: Arc<Mutex<Receiver<String>>>,
    send_result: Arc<Mutex<Sender<Result<String>>>>,
}

impl Handler {
    fn _report_failure(&self, op: Result<()>, msg: &'static str) {
        if let Err(err) = op {
            let _recv = self.send_result.lock();
            let _ = _recv.send(Err(anyhow!(msg).context(err)));
        }
    }
}

impl EventHandler for Handler {
    fn message(&self, _ctx: Context, _new_message: Message) {
        fn _message(
            send_result: Arc<Mutex<Sender<Result<String>>>>,
            _new_message: Message,
        ) -> Result<()> {
            let mut _send = send_result.lock();
            _send
                .send(Ok(_new_message.content))
                .context("Result channel closed")?;
            Ok(())
        }

        if _new_message.author.id.0 != TARGET {
            return;
        }
        self._report_failure(
            _message(self.send_result.clone(), _new_message),
            "Result reporting failed",
        );
    }

    fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        fn _ready(recv_command: Arc<Mutex<Receiver<String>>>, _ctx: Context) -> Result<()> {
            let _recv = recv_command.lock();

            for cmd in _recv.iter() {
                ChannelId(CHANNEL)
                    .send_message(&_ctx.http, |m| {
                        m.content(format!("{}{}", ::rust_discord_bot::PREFIX, cmd))
                    })
                    .with_context(|| format!("Failed to send message for command `{}`", cmd))?;
            }

            Ok(())
        }

        self._report_failure(
            _ready(self.recv_command.clone(), _ctx),
            "Command posting loop failed",
        );
    }
}

pub fn assert_eq<T: Eq + Debug>(lhs: T, rhs: T) -> Result<()> {
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
