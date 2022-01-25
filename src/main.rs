mod cmd;
use std::process;

use cmd::cmd::CmdArgs;

fn main() {
    if let Err(e) = cjuggler::run() {
        println!("Application error: {}", e);
        CmdArgs::print_help();
        process::exit(1);
    }
}
