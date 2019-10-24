use anyhow::{Context as _, Result};
use serenity::client::Client;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};

pub const PREFIX: &str = "~";
const TOKEN: &str = "NTUzMjE1MzY5NDI3NTUwMjE5.XbE2qA.iPZjhety7JoAwMBhLwTX7OPC4vg";

group!({
    name: "general",
    options: {},
    commands: [ping, foo],
});

struct Handler;

impl EventHandler for Handler {}

pub fn run() -> Result<()> {
    let mut client = Client::new(TOKEN, Handler).context("Error creating client")?;

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
