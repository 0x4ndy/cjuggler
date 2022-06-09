use std::error::Error;

use crate::{cmd, config};

pub fn process(args: cmd::CmdArgs, config: config::Config) -> Result<(), Box<dyn Error>> {
    println!("{}", config);
    Ok(())
}
