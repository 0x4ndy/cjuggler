use clap::{App, Arg};
use std::error::Error;

pub struct CmdArgs {
    pub format_name: String,
    pub file_alias: String,
    pub file_name: String,
    pub separator: String,
    pub field_no: u8,
    pub key_pos: u8,
    pub value_pos: u8,
    pub strip: bool,
    pub comment: String,
    is_format_name_set: bool,
    is_file_alias_set: bool,
    is_file_name_set: bool,
    is_separator_set: bool,
    is_field_no_set: bool,
    is_key_pos_set: bool,
    is_value_pos_set: bool,
    is_strip_set: bool,
    is_comment_set: bool,
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

    pub fn is_value_pos_set(&self) -> bool {
        self.is_value_pos_set
    }

    pub fn is_strip_set(&self) -> bool {
        self.is_strip_set
    }

    pub fn is_comment_set(&self) -> bool {
        self.is_comment_set
    }

    pub fn new() -> Result<CmdArgs, Box<dyn Error>> {
        let app = CmdArgs::create_app();

        let matches = app.get_matches();

        Ok(CmdArgs {
            format_name: String::from(matches.value_of("format_name").unwrap_or("")),
            is_format_name_set: matches.is_present("format_name"),
            file_alias: String::from(matches.value_of("file_alias").unwrap_or("")),
            is_file_alias_set: matches.is_present("file_alias"),
            file_name: String::from(matches.value_of("file_name").unwrap_or("")),
            is_file_name_set: matches.is_present("file_name"),
            separator: String::from(matches.value_of("separator").unwrap_or("\t")),
            is_separator_set: matches.is_present("separator"),
            field_no: matches.value_of("fields_no").unwrap_or("2").parse::<u8>()?,
            is_field_no_set: matches.is_present("fields_no"),
            key_pos: matches.value_of("key_pos").unwrap_or("1").parse::<u8>()?,
            is_key_pos_set: matches.is_present("key_pos"),
            value_pos: matches.value_of("value_pos").unwrap_or("2").parse::<u8>()?,
            is_value_pos_set: matches.is_present("value_pos"),
            strip: matches.is_present("strip"),
            is_strip_set: matches.is_present("strip"),
            comment: String::from(matches.value_of("comment").unwrap_or("#")),
            is_comment_set: matches.is_present("comment"),
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
                Arg::with_name("format_name")
                    .required(false)
                    .short("n")
                    .long("format")
                    .takes_value(true)
                    .help("Format name defined in the configuration file."),
            )
            .arg(
                Arg::with_name("file_alias")
                    .required(false)
                    .short("a")
                    .long("alias")
                    .takes_value(true)
                    .help("File alias defined in the configuration file."),
            )
            .arg(
                Arg::with_name("file_name")
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
                Arg::with_name("fields_no")
                    .required(false)
                    .short("i")
                    .long("field-no")
                    .takes_value(true)
                    .help("Number of fields in the processed file."),
            )
            .arg(
                Arg::with_name("key_pos")
                    .required(false)
                    .short("k")
                    .long("key-pos")
                    .takes_value(true)
                    .help("Position of the key field."),
            )
            .arg(
                Arg::with_name("value_pos")
                    .required(false)
                    .short("v")
                    .long("value-pos")
                    .takes_value(true)
                    .help("Position of the value field."),
            )
            .arg(
                Arg::with_name("strip")
                    .required(false)
                    .short("s")
                    .long("strip")
                    .takes_value(false)
                    .help("Indicates that the lines will be stripped out of white spaces."),
            )
            .arg(
                Arg::with_name("comment")
                    .required(false)
                    .short("c")
                    .long("comment")
                    .takes_value(true)
                    .help("Field comment used in the processed file."),
            )
    }
}
