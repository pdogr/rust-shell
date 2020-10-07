use super::command::Command;
#[derive(Debug, PartialEq)]
pub enum Redirection {
  Lt,
  Gt,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  Command(Command),
  Pipe,
  Redirection(Redirection),
}

impl From<Command> for Token {
  fn from(command: Command) -> Self {
    Token::Command(command)
  }
}
impl From<Redirection> for Token {
  fn from(rd: Redirection) -> Self {
    Token::Redirection(rd)
  }
}
impl Clone for Redirection {
  fn clone(&self) -> Self {
    match *self {
      Redirection::Gt => Redirection::Gt,

      Redirection::Lt => Redirection::Lt,
    }
  }
}
