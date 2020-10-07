use std::error::Error;
use std::fmt::{self, Debug, Display};
pub enum ParserError {
  PipeError,
  SyntaxError,
  RedirectionError,
}
impl Debug for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ParserError::PipeError => write!(f, "Pipes should be succeded/preceded by command"),
      ParserError::SyntaxError => write!(f, "Error in parsing"),
      ParserError::RedirectionError => {
        write!(f, "Redirection should be succeded/preceded by command")
      }
    }
  }
}
impl Display for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ParserError::PipeError => write!(f, "Pipes should be succeded/preceded by command"),
      ParserError::SyntaxError => write!(f, "Error in parsing"),
      ParserError::RedirectionError => {
        write!(f, "Redirection should be succeded/preceded by command")
      }
    }
  }
}
impl Error for ParserError {
  fn description(&self) -> &str {
    "Parser error"
  }
  
}
