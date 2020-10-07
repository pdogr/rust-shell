use super::buffer::Buffer;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug)]
pub struct History {
  buffer: Option<String>,
  pub cmd_list: VecDeque<String>,
  pub pos: usize,
}
impl<'a> History {
  pub fn new() -> History {
    History {
      cmd_list: VecDeque::new(),
      buffer: None,
      pos: 0,
    }
  }
  pub fn is_end(&self) -> bool {
    return self.pos == self.cmd_list.len();
  }
  pub fn is_start(&self) -> bool {
    return self.pos == 0;
  }
  pub fn is_last(&self) -> bool {
    return self.cmd_list.len() > 0 && self.pos == self.cmd_list.len() - 1;
  }
  pub fn set_buffer(&mut self, buffer: Buffer) {
    self.buffer = Some(buffer.as_str());
  }
  pub fn get_buffer(&mut self) -> Option<&String> {
    match &self.buffer {
      Some(b) => Some(b),
      None => None,
    }
  }
  pub fn clear_buffer(&mut self) {
    self.buffer = None;
  }
  pub fn reset(&mut self) {
    self.pos = 0;
    self.cmd_list = VecDeque::new();
    self.buffer = None;
  }
  pub fn prev(&mut self) -> Option<&String> {
    match self.pos {
      _ if self.pos > 0 => {
        self.pos -= 1;
        self.cmd_list.get(self.pos)
      }
      _ => None,
    }
  }
  pub fn next(&mut self) -> Option<&String> {
    match self.pos {
      _ if self.is_last() => {
        self.pos += 1;
        self.get_buffer()
      }
      _ if self.pos + 1 < self.cmd_list.len() => {
        self.pos += 1;
        self.cmd_list.get(self.pos)
      }
      _ => None,
    }
  }
  pub fn push(&mut self, cmd: String) {
    match self.cmd_list.back() {
      Some(s) => {
        if *s != cmd {
          self.cmd_list.push_back(cmd);
        }
      }
      None => {
        self.cmd_list.push_back(cmd);
      }
    };
    self.pos = self.cmd_list.len()
  }
}

#[cfg(test)]
pub mod history_test {
  use super::History;
  #[test]
  fn push_test() {
    let mut history = History::new();
    let string: String = "awdadad awd81274387  a ii!@!#".into();
    history.push(string.clone());
    assert_eq!(history.cmd_list.back(), Some(&string));
  }
  #[test]
  fn duplicate_push_test() {
    let mut history = History::new();
    let string: String = "awdadad 78678.,./k  a ii!@!#".into();
    history.push(string.clone());
    history.push(string);
    assert_eq!(history.cmd_list.len(), 1);
  }
  #[test]
  fn history_test() {
    let mut history = History::new();
    history.push("A".to_string());
    history.push("B".to_string());
    history.push("C".to_string());
    assert_eq!(history.next(), None);
    assert_eq!(history.prev(), Some(&"C".to_string()));
    assert_eq!(history.prev(), Some(&"B".to_string()));
    assert_eq!(history.prev(), Some(&"A".to_string()));
    assert_eq!(history.next(), Some(&"B".to_string()));
    assert_eq!(history.prev(), Some(&"A".to_string()));
    assert_eq!(history.prev(), None);
    assert_eq!(history.next(), Some(&"B".to_string()));
    assert_eq!(history.next(), Some(&"C".to_string()));
    assert_eq!(history.next(), None);
  }
}
