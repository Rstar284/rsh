use std::{env, path::Path};
use crate::eval::CmdErr;
use super::template::Cmd;
use crate::utils::expand;

pub struct Cd;

impl Cmd for Cd {
    fn name() -> &'static str {"cd"}
    fn about() -> &'static str {"Change working directory"}
    fn examples() -> [&'static str; 3] {["cd ~fred", "cd ../lib", "cd /"]}
    fn run(args: Vec<String>) -> Result<(), CmdErr> {
        match args.get(0) {
            Some(d) => {
                if env::set_current_dir(Path::new(&expand(d.to_string()))).is_err() {
                    Err(CmdErr::Error(format!("rsh: cd: {}: No such file or directory", d)))
                } else {
                    Ok(())
                }
            }
            None => {
                if env::set_current_dir(env::var("HOME").unwrap()).is_err() {
                    Err(CmdErr::Error("rsh: cd: Could not enter HOME directory".to_string()))
                } else {
                    Ok(())
                }
            }
        }
    }
}