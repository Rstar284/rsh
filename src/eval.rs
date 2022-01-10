use crate::repl::Repl;
use crate::utils::expand;
use crate::builtins::cd::Cd;
use crate::builtins::template::Cmd;
use std::collections::HashMap;
use std::process::Command;
use std::string::ToString;

pub struct InterCmd {
    keyword: String,
    args: Vec<String>,
}

pub enum CmdErr {
    Error(String),
    Exit,
    Finished(i32),
    Terminated(i32),
}

impl InterCmd {
    pub fn new(input: String) -> Self {
        let mut splitted = input.trim().split_whitespace();
        let keyword = match splitted.next() {
            Some(x) => x.to_string(),
            None => String::from(""),
        };

        Self {
            keyword,
            args: splitted.map(ToString::to_string).collect::<Vec<String>>(),
        }
    }

    pub fn eval(&mut self, aliases: &HashMap<&str, &str>) -> Result<(), CmdErr> {
        match (self.keyword.as_str(), self.args.clone()) {
            ("cd", args) => Cd::run(args),
            ("", _) => {
                println!("???");
                Ok(())
            }
            ("exit", _) => Err(CmdErr::Exit),
            (x, y) => match *x.as_bytes().last().unwrap() as char {
                '/' => Cd::run(vec![x.to_string()]),
                _ => {
                    let args = y.into_iter().map(expand).collect::<Vec<_>>();
                    if let Some(alias) = &aliases.get(x) {
                        let mut new_x = alias.to_string();

                        for flag in &args {
                            new_x.push_str(&format!(" {}", flag));
                        }

                        return Repl::run(new_x, aliases);
                    }

                    match Command::new(&x).args(args).spawn() {
                        Ok(mut ok) => {
                            if let Ok(status) = ok.wait() {
                                match status.code() {
                                    Some(code) => {
                                        if code > 0 {
                                            Err(CmdErr::Finished(code))
                                        } else {
                                            Ok(())
                                        }
                                    }
                                    None => Err(CmdErr::Terminated(127)),
                                }
                            } else {
                                Err(CmdErr::Error(
                                    "Command could not be executed".to_string(),
                                ))
                            }
                        }
                        Err(_) => Err(CmdErr::Error(format!("No such command as `{}`", x))),
                    }
                }
            },
        }
    }
}