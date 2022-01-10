mod eval;
mod repl;
mod prompt;
mod utils;
mod builtins;

use repl::Repl;

fn main() {
    Repl::new().start_shell().unwrap();
}