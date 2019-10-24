use anyhow::Result;
use test_bot_internals::{check_reply, get_channels, Chan};

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
    check_reply(chan, "ping".to_string(), "Pong!".to_string())
}

fn test_foo(chan: &Chan) -> Result<()> {
    check_reply(chan, "foo".to_string(), "Bar!".to_string())
}
