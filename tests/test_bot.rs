use anyhow::Result;
use test_bot_internals::{get_channels, simple, Chan};

mod test_bot_internals;

pub const TOKEN: &str = "NDU1ODI4MzI3OTAzNzg5MDk2.XbF1eA.ZiYAhMwMpP7FA0UuHt5u_vZsf-4";
pub const SELF: u64 = 455828327903789096;
pub const TARGET: u64 = 553215369427550219;
pub const CHANNEL: u64 = 636866066030788608;

#[test]
fn test_bot() -> Result<()> {
    let chan = get_channels();

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
