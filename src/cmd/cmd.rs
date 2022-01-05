use clap::{App, Arg};
use std::error::Error;

pub struct CmdArgs {
    pub format_name: String,
    pub file_alias: String,
    pub file_name: String,
    pub separator: String,
    pub field_no: u16,
    pub key_pos: u16,
    pub strip: bool,
    is_format_name_set: bool,
    is_file_alias_set: bool,
    is_file_name_set: bool,
    is_separator_set: bool,
    is_field_no_set: bool,
    is_key_pos_set: bool,
    is_strip_set: bool,
}

impl CmdArgs {
    pub fn is_format_name_set(&self) -> bool {
        self.is_format_name_set
    }

    pub fn is_file_alias_set(&self) -> bool {
        self.is_file_alias_set
    }

    pub fn is_file_name_set(&self) -> bool {
        self.is_file_name_set
    }

    pub fn is_separator_set(&self) -> bool {
        self.is_separator_set
    }

    pub fn is_field_no_set(&self) -> bool {
        self.is_field_no_set
    }

    pub fn is_key_pos_set(&self) -> bool {
        self.is_key_pos_set
    }

    pub fn is_strip_set(&self) -> bool {
        self.is_strip_set
    }

    pub fn new() -> Result<CmdArgs, Box<dyn Error>> {
        let app = CmdArgs::create_app();

        let matches = app.get_matches();

        Ok(CmdArgs {
            format_name: String::from(matches.value_of("format-name").unwrap_or("")),
            is_format_name_set: matches.is_present("format-name"),
            file_alias: String::from(matches.value_of("file-alias").unwrap_or("")),
            is_file_alias_set: matches.is_present("file-alias"),
            file_name: String::from(matches.value_of("file-name").unwrap_or("")),
            is_file_name_set: matches.is_present("file-name"),
            separator: String::from(matches.value_of("separator").unwrap_or("")),
            is_separator_set: matches.is_present("separator"),

            field_no: matches
                .value_of("fields-no")
                .unwrap_or("2")
                .parse::<u16>()?,
            is_field_no_set: matches.is_present("fields-no"),

            key_pos: matches.value_of("key-pos").unwrap_or("1").parse::<u16>()?,
            is_key_pos_set: matches.is_present("key-pos"),

            strip: matches.is_present("strip"),
            is_strip_set: matches.is_present("strip"),
        })
    }

    pub fn print_help() {
        CmdArgs::create_app().print_help().unwrap();
        println!();
    }

    fn create_app() -> App<'static, 'static> {
        let version = env!("CARGO_PKG_VERSION");
        let authors = env!("CARGO_PKG_AUTHORS");

        App::new("Config Juggler")
            .version(version)
            .author(authors)
            .about("https://github.com/0x4ndy/cjuggler")
            .arg(
                Arg::with_name("format-name")
                    .required(false)
                    .short("n")
                    .long("format")
                    .takes_value(true)
                    .help("Format name defined in the configuration file."),
            )
            .arg(
                Arg::with_name("file-alias")
                    .required(false)
                    .short("a")
                    .long("alias")
                    .takes_value(true)
                    .help("File alias defined in the configuration file."),
            )
            .arg(
                Arg::with_name("file-name")
                    .required(false)
                    .short("f")
                    .long("file")
                    .takes_value(true)
                    .help("Name and path to the file for processing."),
            )
            .arg(
                Arg::with_name("separator")
                    .required(false)
                    .short("d")
                    .long("separator")
                    .takes_value(true)
                    .help("Field separator/delimiter used in the processed file."),
            )
            .arg(
                Arg::with_name("fields-no")
                    .required(false)
                    .short("i")
                    .long("field-no")
                    .takes_value(true)
                    .help("Number of fields in the processed file."),
            )
            .arg(
                Arg::with_name("key-pos")
                    .required(false)
                    .short("k")
                    .long("key-pos")
                    .takes_value(true)
                    .help("Position of the key field."),
            )
            .arg(
                Arg::with_name("strip")
                    .required(false)
                    .short("s")
                    .long("strip")
                    .takes_value(false)
                    .help("Indicates that the lines will be stripped out of white spaces."),
            )
    }
}
