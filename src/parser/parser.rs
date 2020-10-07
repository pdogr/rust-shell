use super::error::ParserError;
use crate::constants::{GT, LT, PIPE};
use crate::tokens::command::Command;
use crate::tokens::input::Input;
use crate::tokens::output::Output;
use crate::tokens::tokens::{Redirection, Token};
use nix::unistd::pipe;
use std::fs::File;
use std::io;
use std::os::unix::io::{FromRawFd, RawFd};
pub struct Parser {
  pub pos: usize,
  pub input: String,
  pub artifacts: Vec<Token>,
  pub commands: Vec<Command>,
  pub pipes: Vec<(RawFd, RawFd)>,
}

impl Parser {
  pub fn new(input: String) -> Parser {
    return Parser {
      pos: 0,
      input: input,
      artifacts: Vec::new(),
      commands: Vec::new(),
      pipes: Vec::new(),
    };
  }
  pub fn build(&mut self) -> Option<ParserError> {
    let _tokens = self.parse();
    self.artifacts = _tokens;
    match self.setup_pipes(self.artifacts.clone()) {
      Ok(cmd) => {
        self.commands = cmd;
        return None;
      }
      Err(e) => Some(e),
    }
  }
  fn setup_pipes(&mut self, mut artifacts: Vec<Token>) -> Result<Vec<Command>, ParserError> {
    let mut prev_output: Input = Input::from(io::stdin());
    let mut commands: Vec<Command> = Vec::new();
    let mut i = 0;
    while i < artifacts.len() {
      match artifacts.get(i) {
        Some(token) => {
          if self.is_pipe(token) {
            if !self.is_next_cmd(i) || !self.is_prev_cmd(i) {
              return Err(ParserError::PipeError);
            } else {
              if let Some(Token::Command(cmd)) = artifacts.get_mut(i - 1) {
                cmd.inp(prev_output.clone());
                let pipe = self.pipes.pop().unwrap();
                cmd.out(unsafe { Output::from(File::from_raw_fd(pipe.1)) });
                prev_output = unsafe { Input::from(File::from_raw_fd(pipe.0)) };
                commands.push(cmd.clone());
              } else {
                return Err(ParserError::PipeError);
              }
            }
          } else if self.is_command(token) {
            if self.pipes.len() == 0 {
              if let Some(Token::Command(cmd)) = artifacts.get_mut(i) {
                cmd.inp(prev_output.clone());
                cmd.out(Output::Stdout(io::stdout()));
                commands.push(cmd.clone());
              }
            }
          } else {
            unreachable!();
          }
        }
        None => {
          break;
        }
      };
      i += 1;
    }
    Ok(commands)
  }
  fn is_prev_cmd(&self, i: usize) -> bool {
    if i == 0 {
      return false;
    } else {
      match self.artifacts.get(i - 1) {
        Some(token) => self.is_command(token),
        None => false,
      }
    }
  }
  fn is_next_cmd(&self, i: usize) -> bool {
    match self.artifacts.get(i + 1) {
      Some(token) => self.is_command(token),
      None => false,
    }
  }

  fn is_pipe(&self, token: &Token) -> bool {
    match token {
      Token::Pipe => true,
      _ => false,
    }
  }
  fn is_command(&self, token: &Token) -> bool {
    match token {
      Token::Command(_c) => true,
      _ => false,
    }
  }
  fn parse(&mut self) -> Vec<Token> {
    let mut parsed_vector: Vec<Token> = Vec::new();
    let input_len = self.input.len();
    loop {
      if self.pos >= input_len {
        break;
      }
      let token = self.parse_token();
      parsed_vector.push(token)
    }
    return parsed_vector;
  }
  fn skip_whitespace(&mut self) {
    let input_len = self.input.len();
    while self.pos < input_len && self.get(self.pos).is_whitespace() {
      self.pos += 1
    }
  }
  fn peek_next_token(&mut self) -> String {
    self.skip_whitespace();
    let mut j = self.pos;
    let mut tok = String::new();
    let input_len = self.input.len();
    while j < input_len && valid_char(self.get(j)) {
      tok.push(self.get(j));
      j += 1;
    }
    return tok;
  }
  fn next_token(&mut self) {
    let mut j = self.pos;
    let input_len = self.input.len();
    while j < input_len && valid_char(self.get(j)) {
      j += 1;
    }
    self.pos = j;
  }
  fn parse_token(&mut self) -> Token {
    self.skip_whitespace();
    let tok = self.peek_next_token();
    return match tok {
      _ if tok == PIPE => self.handle_pipe(),
      _ if tok == LT || tok == GT => self.handle_redirection(),
      _ => self.handle_command(),
    };
  }
  fn handle_pipe(&mut self) -> Token {
    self.pipes.push(pipe().unwrap());
    self.next_token();
    return Token::Pipe;
  }
  fn handle_redirection(&mut self) -> Token {
    let token = self.peek_next_token();
    self.next_token();
    return match token {
      _ if token == LT => Token::Redirection(Redirection::Lt),
      _ if token == GT => Token::Redirection(Redirection::Gt),
      _ => panic!("LT or GT"),
    };
  }
  fn handle_command(&mut self) -> Token {
    let command = self.peek_next_token();
    self.next_token();
    let input_len = self.input.len();
    let mut command_args: Vec<String> = Vec::new();
    while self.pos < input_len {
      let next_token = self.peek_next_token();
      match next_token {
        _ if next_token == PIPE || next_token == LT || next_token == GT => {
          break;
        }
        _ => {
          command_args.push(next_token);
          self.next_token();
        }
      }
    }
    return Token::Command(Command::new(command, command_args, None, None));
  }
  fn get(&mut self, pos: usize) -> char {
    return self.input[pos..].chars().next().unwrap();
  }
}
fn valid_char(ch: char) -> bool {
  return match ch {
    ' ' => false,
    _ => true,
  };
}

#[cfg(test)]
pub mod parser_test {
  use super::*;
  use crate::tokens::*;
  #[test]
  fn parse_test() {
    let input = "a | asd --a | aaw | pp".to_string();
    let mut parser = Parser::new(input);
    parser.build();
    let tok1 = tokens::Token::Command(command::Command::new(
      String::from("a"),
      Vec::new(),
      None,
      None,
    ));
    let tok2 = tokens::Token::Command(command::Command::new(
      String::from("asd"),
      vec![String::from("--a")],
      None,
      None,
    ));
    let tok3 = tokens::Token::Command(command::Command::new(
      String::from("aaw"),
      Vec::new(),
      None,
      None,
    ));
    let tok4 = tokens::Token::Command(command::Command::new(
      String::from("pp"),
      Vec::new(),
      None,
      None,
    ));
    let expected = vec![
      tok1,
      tokens::Token::Pipe,
      tok2,
      tokens::Token::Pipe,
      tok3,
      tokens::Token::Pipe,
      tok4,
    ];
    assert_eq!(parser.artifacts, expected);
  }
}
