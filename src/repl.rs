#![warn(unreachable_code)]

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::process;
use crate::eval::{CmdErr, InterCmd};
use crate::prompt::Prompt;
use crate::utils::{fetch_data, get_alias, get_conf};
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Repl;
impl Repl {
    pub fn new() -> Self {
        Self {}
    }
    pub fn start_shell(&mut self) -> io::Result<()> {
        let mut rl = Editor::<()>::new();
        let home_dir = env::var("HOME").unwrap();
        if rl.load_history(&format!("{}/.rsh_hist", home_dir)).is_err() {
            eprintln!("rsh: No previous history.");
            if File::create(format!("{}/.rsh_hist", home_dir)).is_err() {eprintln!("vsh: Could not create history file!");}
        }
        let conf = match get_conf(fetch_data()) {
            Ok(x) => x,
            Err(e) => {
                println!("rsh: {:?}", e);
                get_conf(String::from("")).unwrap()
            }
        };
        let aliases = get_alias(&conf);
        loop {
            let prmt = Prompt::new(&conf).gen_prompt();
            let readl = rl.readline(prmt.as_str());
            match readl {
                Ok(x) => {
                    rl.add_history_entry(x.as_str());
                    if let Err(e) = Self::run(x, &aliases) {
                        match e {
                            CmdErr::Exit => {
                                if rl.save_history(&format!("{}/.rsh_hist", home_dir)).is_err() {
                                    eprintln!("rsh: Could not save history.");
                                }
                                process::exit(0);
                            }
                            CmdErr::Error(e) => eprintln!("rsh: {}", e),
                            CmdErr::Terminated(x) => eprintln!("rsh: Process exited with code {}", x),
                            CmdErr::Finished(_) => continue
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => println!(),
                Err(ReadlineError::Eof) => break,
                Err(e) => {
                    print!("rsh: Unexpected Error: {:?}", e);
                    break;
                }
            }
            if rl.save_history(&format!("{}/.rsh_hist", home_dir)).is_err() {
                eprintln!("rsh: Could not save history.");
            }
        }
        Ok(())
    }
    pub fn run(x: String, y: &HashMap<&str, &str>) -> Result<(), CmdErr> {
        let mut last_return = Ok(());
        for com in x.split(';') {
            last_return = Self::run_linked_commands(com.into(), y);
        }
        last_return
    }

    fn run_command(com: String, x: &HashMap<&str, &str>) -> Result<(), CmdErr> {
        InterCmd::new(com).eval(x)
    }
    fn run_linked_commands(commands: String, x: &HashMap<&str, &str>) -> Result<(), CmdErr> {
        for linked_com in commands.split("&&") {
            if let Err(e) = Self::run_command(linked_com.to_string(), x) {
                return Err(e);
            }
        }
        Ok(())
    }
}