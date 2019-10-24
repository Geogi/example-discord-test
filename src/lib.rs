use anyhow::{Context as _, Result};
use serenity::client::Client;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};
use std::env::var;

pub const PREFIX: &str = "~";
const TOKEN_ENV: &str = "DISCORD_TOKEN";
const TOKEN_ENV_BETA: &str = "DISCORD_TOKEN_BETA";

group!({
    name: "general",
    options: {},
    commands: [ping, foo],
});

struct Handler;

impl EventHandler for Handler {}

pub fn run() -> Result<()> {
    let token = var(TOKEN_ENV_BETA)
        .or(var(TOKEN_ENV))
        .context("Missing bot token environment variable")?;
    let mut client = Client::new(token, Handler).context("Error creating client")?;

    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.prefix(PREFIX);
                c.ignore_bots(false)
            })
            .group(&GENERAL_GROUP),
    );

    client
        .start()
        .context("An error occurred while running the client")?;

    Ok(())
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
fn foo(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Bar!")?;

    Ok(())
}
