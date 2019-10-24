use super::{CHANNEL, SELF, TARGET, TOKEN_ENV};
use anyhow::{anyhow, Context as _, Result};
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::Mutex;
use std::env::var;
use std::fmt::Debug;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread::spawn;
use std::time::Duration;

pub type Chan = (Sender<String>, Receiver<Result<String>>);

pub fn simple(chan: &Chan, cmd: String, expected: String) -> Result<()> {
    chan.0.send(cmd).context("Command channel closed")?;
    let result = chan
        .1
        .recv_timeout(Duration::new(5, 0))
        .context("Test request timed out")?
        .context("An error was reported through the result channel")?;
    assert_eq(result, expected)?;
    Ok(())
}

pub fn check_reply(chan: &Chan, cmd: String, expected: String) -> Result<()> {
    simple(chan, cmd, format!("<@{}>: {}", SELF, expected))
}

pub fn get_channels() -> Chan {
    let (send_command, recv_command) = channel();
    let (send_result, recv_result) = channel();
    let send_result2 = send_result.clone();

    spawn(move || run_main_bot(send_result));
    spawn(move || run_test_bot(recv_command, send_result2));

    (send_command, recv_result)
}

pub fn run_main_bot(send_result: Sender<Result<String>>) {
    report_failure(
        send_result,
        ::example_discord_test::run(),
        "Cannot set up the main bot",
    );
}

pub fn run_test_bot(recv_command: Receiver<String>, send_result: Sender<Result<String>>) {
    fn _run(recv_command: Receiver<String>, send_result: Sender<Result<String>>) -> Result<()> {
        let token = var(TOKEN_ENV).context("Missing test bot token environment variable")?;
        let mut client = Client::new(
            token,
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
                        m.content(format!("{}{}", ::example_discord_test::PREFIX, cmd))
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
