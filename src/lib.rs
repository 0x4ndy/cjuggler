mod cmd;
mod config;

use std::error::Error;

use cmd::cmd::CmdArgs;
use config::config::{get_config, Config};

pub fn run() -> Result<(), Box<dyn Error>> {
    let config: Config = get_config()?;
    let args = CmdArgs::new()?;

    if !args.is_file_name_set() && !args.is_file_alias_set() {
        CmdArgs::print_help();
        Err("\"file-name\" or \"file-alias\" option required!")?
    }

    println!("{}", config);
    Ok(())
}
