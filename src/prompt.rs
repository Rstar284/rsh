use colored::*;
use serde_derive::Deserialize;
use crate::utils::Config;

#[derive(Debug)]
pub enum Prompt {
    Classic {
        promptchar: String,
        text_color: (u8, u8, u8),
        color: (u8, u8, u8),
        double: bool,
    },
    // TODO: Add more styles
}

#[derive(Deserialize)]
pub struct PromptConfig {
    style: Option<String>,
    promptchar: Option<String>,
    color: Option<[u8; 3]>,
    text_color: Option<[u8; 3]>,
    double: Option<bool>,
}

impl Prompt {
    pub fn new(data: &Config) -> Self {
        let mut color = (0, 102, 204);
        let mut text_color = (255, 255, 255);
        let mut promptchar = String::from("➤");
        let mut double = false;
        let rt = Self::Classic {
            promptchar: promptchar.clone(),
            text_color,
            color,
            double,
        };
        if let Some(prompt) = &data.prompt {
            if let Some(x) = prompt.color {color = (x[0], x[1], x[2]);}
            if let Some(x) = prompt.text_color {text_color = (x[0], x[1], x[2]);}
            if let Some(x) = prompt.double {double = x;}
            if let Some(x) = &prompt.promptchar {promptchar = x.clone();}
            if let Some(x) = &prompt.style {
                return match x.to_lowercase().as_str() {
                    "classic" => Self::Classic {
                        promptchar,
                        text_color,
                        color,
                        double,
                    },
                    _ => Self::Classic {
                        promptchar,
                        text_color,
                        color,
                        double,
                    },
                };
            } else {
                return rt;
            }
        }
        rt
    }

    pub fn gen_prompt(&self) -> String {
        let mut current_dir = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
        match self {
            Self::Classic {promptchar, color, text_color, double} => {
                if current_dir == std::env::var("HOME").unwrap() {
                    current_dir = String::from("~");
                }
                if current_dir.starts_with(&std::env::var("HOME").unwrap()) {
                    current_dir = current_dir.replace(&std::env::var("HOME").unwrap(), "~");
                }
                let directory = format!(" {} ", current_dir)
                    .on_truecolor(color.0, color.1, color.2).truecolor(text_color.0, text_color.1, text_color.2).bold();
                let pr_char = promptchar
                    .replace("\"", "").truecolor(color.0, color.1, color.2);
                if *double {
                    format!("{}\n{} ", directory, pr_char)
                } else {
                    format!("{} {} ", directory, pr_char)
                }
            }
        }
    }
}
