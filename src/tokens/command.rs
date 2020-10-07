use super::input::Input;
use super::output::Output;

#[derive(Debug,Clone)]
pub struct Command {
  pub command: String,
  pub args: Vec<String>,
  pub input: Option<Input>,
  pub output: Option<Output>,
}
impl Command {
  pub fn new(
    command: String,
    args: Vec<String>,
    input: Option<Input>,
    output: Option<Output>,
  ) -> Command {
    Command {
      command,
      args,
      input,
      output,
    }
  }
}
impl PartialEq for Command {
  fn eq(&self, other: &Self) -> bool {
    self.command == other.command && self.args == other.args
  }
}
impl Command {
  pub fn out(&mut self, output: Output) {
    self.output = Some(output);
  }
  pub fn inp(&mut self, input: Input) {
    self.input = Some(input);
  }
}
