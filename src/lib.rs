mod cmd;
mod config;

use std::error::Error;

use cmd::CmdArgs;
use config::{get_config, Config};

pub fn run() -> Result<(), Box<dyn Error>> {
    let app_config: Config = get_config()?;
    let app_args = CmdArgs::new()?;

    if !app_args.is_file_name_set() && !app_args.is_file_alias_set() {
        Err("\"file_name\" or \"file_alias\" option required!")?
    }

    println!("{}", app_config);
    Ok(())
}
