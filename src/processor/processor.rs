use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::{cmd::CmdArgs, config::Config};

pub fn process(args: CmdArgs, config: Config) -> Result<(), Box<dyn Error>> {
    let file_name = get_file_name(args, config)?;

    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            println!("{}", line?);
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_file_name(args: CmdArgs, config: Config) -> Result<String, Box<dyn Error>> {
    if args.is_file_name_set() {
        Ok(args.file_name)
    } else {
        get_file_name_by_alias(args.file_alias, config)
    }
}

fn get_file_name_by_alias(alias: String, config: Config) -> Result<String, Box<dyn Error>> {
    for format in config.formats {
        for file in format.files {
            if alias == file.alias {
                return Ok(file.path);
            }
        }
    }

    Err("File alias not found.")?
}
