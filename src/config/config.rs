use serde_json::Value;
use std::fmt;
use std::fs;
//use std::io::Error;
use std::error::Error;

pub struct ConfigFile {
    pub alias: String,
    pub path: String,
}

impl fmt::Display for ConfigFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nFile:\n\tAlias: {}\n\tPath: {}", self.alias, self.path)
    }
}

pub struct ConfigFormat {
    pub name: String,
    pub sep: String,
    pub fields_no: u16,
    pub key_pos: u16,
    pub strip: bool,
    pub comment: String,
    pub files: Vec<ConfigFile>,
}

impl fmt::Display for ConfigFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from(format!(
            "Name: {}\nSeparator: \"{}\"\nNumber of fields: {}\nKey position: {}\nStrip: {}\nComment: {}",
            self.name, self.sep, self.fields_no, self.key_pos, self.strip, self.comment
        ));

        for file in self.files.iter() {
            result.push_str(format!("{}", file).as_str());
        }

        write!(f, "{}", result)
    }
}

pub struct Config {
    pub formats: Vec<ConfigFormat>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("");

        for format in self.formats.iter() {
            result.push_str(format!("{}\n", format).as_str());
        }
        write!(f, "{}", result)
    }
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    match get_config_with_filename(self::DEFAULT_CONFIGURATION_PATH) {
        Ok(config) => Ok(config),
        Err(_) => self::get_default_config(),
    }
}

pub fn get_config_with_filename(s: &str) -> Result<Config, Box<dyn Error>> {
    let config_str = fs::read_to_string(String::from(s))?;
    //let config_str = fs::read_to_string(String::from(s));
    //let config_str = match config_str {
    //    Ok(value) => value,
    //    Err(e) => return Err(""),
    //};

    let json_config: Value = serde_json::from_str(config_str.as_str())?;

    let mut config = Config {
        formats: Vec::new(),
    };

    for cformat in json_config[self::STR_FORMATS].as_array().unwrap() {
        let mut config_format = ConfigFormat {
            name: get_string_value(self::STR_NAME, cformat, true, self::STR_EMPTY),
            sep: get_string_value(self::STR_SEPARATOR, cformat, false, self::DEFAULT_SEPARATOR),
            fields_no: get_int_value(
                self::STR_FIELDS_NO,
                cformat,
                false,
                self::DEFAULT_FIELDS_NO as u64,
            ) as u16,
            key_pos: get_int_value(
                self::STR_KEY_POS,
                cformat,
                false,
                self::DEFAULT_KEY_POS as u64,
            ) as u16,
            strip: get_bool_value(self::STR_STRIP, cformat, false, self::DEFAULT_STRIP),
            comment: get_string_value(self::STR_COMMENT, cformat, false, self::DEFAULT_COMMENT),
            files: Vec::new(),
        };
        for cfile in cformat[self::STR_FILES].as_array().unwrap() {
            let config_file = ConfigFile {
                alias: get_string_value(self::STR_ALIAS, cfile, true, self::STR_EMPTY),
                path: get_string_value(self::STR_PATH, cfile, true, self::STR_EMPTY),
            };
            config_format.files.push(config_file);
        }
        config.formats.push(config_format);
    }

    Ok(config)
}

fn get_default_config() -> Result<Config, Box<dyn Error>> {
    let mut config = Config {
        formats: Vec::new(),
    };

    let config_format = ConfigFormat {
        name: String::from(self::DEFAULT_FORMAT_NAME),
        sep: String::from(self::DEFAULT_SEPARATOR),
        fields_no: self::DEFAULT_FIELDS_NO as u16,
        key_pos: self::DEFAULT_KEY_POS as u16,
        strip: self::DEFAULT_STRIP,
        comment: String::from(self::DEFAULT_COMMENT),
        files: Vec::new(),
    };

    config.formats.push(config_format);

    Ok(config)
}

fn get_int_value(key: &str, value: &Value, required: bool, default: u64) -> u64 {
    if required && !value[key].is_u64() {
        panic!(
            "Field \"{}\" has an incorrect value. Expected: unsigned integer.",
            key
        );
    }

    value[key].as_u64().unwrap_or(default as u64)
}

fn get_string_value(key: &str, value: &Value, required: bool, default: &str) -> String {
    if required && !value[key].is_string() {
        panic!(
            "Field \"{}\" has an incorrect value. Expected: string.",
            key
        );
    }

    String::from(value[key].as_str().unwrap_or(default))
}

fn get_bool_value(key: &str, value: &Value, required: bool, default: bool) -> bool {
    if required && value[key].as_bool() == None {
        panic!(
            "Field \"{}\" has an incorrect value. Expected: true/false.",
            key
        );
    }

    value[key].as_bool().unwrap_or(default)
}

// Default string representation related to the configuration
const STR_FORMATS: &str = "formats";
const STR_NAME: &str = "name";
const STR_SEPARATOR: &str = "sep";
const STR_FIELDS_NO: &str = "fields-no";
const STR_KEY_POS: &str = "key-pos";
const STR_STRIP: &str = "strip";
const STR_FILES: &str = "files";
const STR_ALIAS: &str = "alias";
const STR_PATH: &str = "path";
const STR_EMPTY: &str = "";
const STR_COMMENT: &str = "comment";

// Default configuration file
const DEFAULT_CONFIGURATION_PATH: &str = "$HOME/.config/cjuggler/cjuggler.json";

// Default values of the config
const DEFAULT_FORMAT_NAME: &str = "key-value";
const DEFAULT_SEPARATOR: &str = "\t";
const DEFAULT_FIELDS_NO: u8 = 2;
const DEFAULT_KEY_POS: u8 = 1;
const DEFAULT_STRIP: bool = false;
const DEFAULT_COMMENT: &str = "#";
