use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use lazy_static::lazy_static;
use serde_derive::Deserialize;

use crate::prompt::PromptConfig;

const BASE_CONFIG: &str = r#"
# This is the config file for rsh.
# These are the default values

[prompt]
double = false
color = [0, 102, 204]
text_color = [255, 255, 255]
promptchar = "➤"
style = "classic"
"#;

#[derive(Deserialize)]
pub struct Config {
    pub prompt: Option<PromptConfig>,
    pub misc: Option<Misc>,
}

#[derive(Deserialize)]
pub struct Misc {
    pub alias: Option<Vec<[String; 2]>>,
}

pub fn fetch_data() -> String {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
    path.push(".rshrc");
    let mut data = String::new();
    if path.exists() {
        match File::open(&path) {
            Ok(mut x) => if x.read_to_string(&mut data).is_err() {eprintln!("rsh: config file is not in UTF-8 encoding, So it cannot be read D:");},
            Err(_) => eprintln!("rsh: Error Occured while opening config file D:")
        }
    } else {
        match File::create(&path) {
            Ok(mut x) => {
                if x.write_all(BASE_CONFIG.as_bytes()).is_err() {
                    eprintln!("rsh: Could not write to config file D:")
                }
                data = String::from("");
            }
            Err(_) => eprintln!("rsh: Config File could not be created D:"),
        }
    }
    data
}

pub fn get_conf(data: String) -> Result<Config, String> {
    match toml::from_str::<Config>(&data) {
        Ok(ok) => Ok(ok),
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_alias(data: &Config) -> HashMap<&str, &str> {
    let mut list: HashMap<&str, &str> = HashMap::new();
    if let Some(misc) = &data.misc {
        if let Some(alias) = &misc.alias {
            for x in alias.iter() {
                list.insert(&x[0], &x[1]);
            }
        }
    }
    list
}

pub fn expand(raw: String) -> String {
    lazy_static! {
        static ref RE: fancy_regex::Regex = fancy_regex::Regex::new("(?<!\\\\)\\~").unwrap();
    }
    RE.replace_all(&raw, env::var("HOME").unwrap()).to_string()
}