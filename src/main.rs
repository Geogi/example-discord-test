use anyhow::Result;

pub mod lib;

fn main() -> Result<()> {
    lib::run()
}
