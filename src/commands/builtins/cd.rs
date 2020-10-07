use crate::tokens::command::Command;
use std::env;
use std::error::Error;
use std::io::{self, Read, Write};
#[allow(deprecated)]
pub fn run(command: Command) -> Result<(), String> {
  if command.args.is_empty() {
    env::set_current_dir(env::home_dir().unwrap()).unwrap();
    return Ok(());
  }

  let mut current_path = env::current_dir().unwrap();
  current_path.push(&command.args[0]);
  if env::set_current_dir(current_path.as_path()).is_err() {
    return Err(format!("{} not found", command.args[0]));
  };
  Ok(())
}
