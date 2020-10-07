use crate::tokens::command::Command;
use std::env;
use std::io::prelude::*;

pub fn run(cmd: Command) -> Result<(), String> {
  let result = format!("{}\r\n", env::current_dir().unwrap().display());
  let mut out = cmd.output.unwrap();
  match out.write_all(result.as_bytes()) {
    Ok(_) => Ok(()),
    Err(_) => Err("Error: pwd".to_string()),
  }
}
