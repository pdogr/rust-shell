use super::buffer::Buffer;
use super::cursor;
use super::history::History;
use super::window::{get_winsize, Winsize};
use libc::STDOUT_FILENO;
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

#[derive(Debug)]
pub struct Terminal {
  pub pos: usize,
  stdout_buffer: String,
  buffer: Rc<RefCell<Buffer>>,
  pub history: History,
  pub prompt: String,
  pub window_size: Winsize,
}
impl Terminal {
  pub fn new() -> Terminal {
    let terminal = Terminal {
      pos: 0,
      history: History::new(),
      buffer: Rc::new(RefCell::new(Buffer::new())),
      stdout_buffer: String::new(),
      prompt: "$ ".into(),
      window_size: get_winsize(STDOUT_FILENO).unwrap(),
    };
    return terminal;
  }
  pub fn set_buffer(&mut self) {
    self.history.set_buffer(self.buffer.borrow().clone());
  }
  pub fn update_buffer(&mut self, string: &str) {
    self.buffer = Rc::new(RefCell::new(Buffer::from(string)));
  }
  pub fn reset(&mut self) {
    self.pos = 0;
    self.buffer.borrow_mut().clear();
  }
  pub fn get(&mut self) -> Option<String> {
    let line = self.buffer.borrow().as_str();
    if line.len() == 0 {
      return None;
    }
    return Some(line);
  }
  pub fn push(&mut self, string: &str) {
    if self.pos == self.buffer.borrow().len() {
      self.buffer.borrow_mut().insert_str(self.pos, string);
      self.stdout_buffer.push_str(string);
      self.pos += string.len();
    } else {
      self.buffer.borrow_mut().insert_str(self.pos, string);
      self.clear_to_screen_end();
      self
        .stdout_buffer
        .push_str(&self.buffer.borrow_mut().as_str().get(self.pos..).unwrap());
      self.pos += string.len();
      self.move_to(self.pos);
    }
  }
  pub fn delete_left(&mut self, n: usize) {
    let old_pos = self.pos;
    if self.pos >= n {
      self.pos -= n;
    } else {
      self.pos = 0;
    }
    self.buffer.borrow_mut().remove(self.pos, old_pos);
    self.move_to(self.pos);
    self.clear_to_screen_end();
    self
      .stdout_buffer
      .push_str(&self.buffer.borrow().as_str().get(self.pos..).unwrap());
    self.move_to(self.pos);
  }
  pub fn delete_right(&mut self, n: usize) {
    let old_pos = self.pos;
    if self.pos + n <= self.buffer.borrow().len() {
      self.pos += n;
    } else {
      self.pos = self.buffer.borrow().len();
    }
    self.buffer.borrow_mut().remove(old_pos, self.pos);
    self.clear_to_screen_end();
    self
      .stdout_buffer
      .push_str(&self.buffer.borrow().as_str().get(old_pos..).unwrap());
    self.move_to(old_pos);
  }

  pub fn clear_screen(&mut self) {
    self
      .stdout_buffer
      .push_str(&format!("\x1b[2J\x1b[1;1H{}", self.prompt));
  }
  pub fn clear_to_screen_end(&mut self) {
    self.stdout_buffer.push_str(&cursor::clear_to_screen_end());
  }
  pub fn move_left(&mut self, n: usize) {
    if self.pos == 0 {
      return;
    }
    self.pos -= n;
    self.stdout_buffer.push_str(&cursor::move_left(n));
  }
  pub fn move_right(&mut self, n: usize) {
    let buf = self.buffer.borrow();
    if self.pos == buf.len() {
      return;
    }
    self.pos += n;
    if self.pos > buf.len() {
      self.pos = buf.len();
    }
    self.stdout_buffer.push_str(&cursor::move_right(n));
  }
  pub fn move_to_first(&mut self) {
    self.move_to(0);
  }
  pub fn move_to_end(&mut self) {
    let n = self.buffer.borrow().len();
    self.move_to(n);
  }
  pub fn move_to(&mut self, n: usize) {
    self.pos = n;
    self
      .stdout_buffer
      .push_str(&cursor::move_to(self.prompt.len() + n + 1));
  }
  pub fn display(&mut self) -> io::Result<()> {
    self.write(&self.stdout_buffer)?;
    self.stdout_buffer.clear();
    Ok(())
  }
  pub fn write_str(&mut self, s: &str) {
    self.stdout_buffer.push_str(s);
  }
  pub fn write_line(&mut self) {
    self.stdout_buffer.push_str("\n");
  }
  pub fn write_linefeed(&mut self) {
    self.stdout_buffer.push_str("\r");
  }
  pub fn write_buffer(&mut self) {
    let string = self.buffer.borrow().as_str();
    self.write_str(&string);
  }
  pub fn write_prompt(&mut self) {
    self.write_str(&self.prompt.clone());
  }
  fn write(&self, s: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    write!(lock, "{}", s)?;
    lock.flush()
  }
}

#[cfg(test)]
pub mod terminal_test {
  use super::Terminal;
  #[test]
  fn move_to_first_test() {
    let mut terminal = Terminal::new();
    terminal.push("tester");
    terminal.move_to_first();
    assert_eq!(terminal.pos, 0);
  }
  #[test]
  fn move_to_end_test() {
    let mut terminal = Terminal::new();
    let string = "Asdasdasdad".into();
    terminal.push(string);
    terminal.move_to_end();
    assert_eq!(terminal.pos, string.len());
  }
  #[test]
  fn set_buffer_test() {
    let mut terminal = Terminal::new();
    let string = "AdwHuiw dre iY FEAWUFY ".into();
    terminal.push(string);
    terminal.set_buffer();
    assert_eq!(
      Some(&terminal.buffer.borrow().as_str()),
      terminal.history.get_buffer()
    );
  }
  #[test]
  fn update_buffer_test() {
    let mut terminal = Terminal::new();
    let string = "awdwda|| wdau h2".into();
    terminal.update_buffer(string);
    assert_eq!(terminal.buffer.borrow().as_str(), string);
  }
  #[test]
  fn reset_test() {
    let mut terminal = Terminal::new();
    let string = "awdwddwada  wda247284 wd2".into();
    terminal.push(string);
    terminal.reset();
    assert_eq!(terminal.pos, 0);
    assert_eq!(terminal.buffer.borrow().as_str(), "");
  }
  #[test]
  fn get_none_test() {
    let mut terminal = Terminal::new();
    assert_eq!(terminal.get(), None);
  }
  #[test]
  fn get_some_test() {
    let mut terminal = Terminal::new();
    let string: String = "awdwddwadawd awd wad dau h2".into();
    terminal.push(&string);
    assert_eq!(terminal.get(), Some(string));
  }
  #[test]
  fn push_test(){
    let mut terminal = Terminal::new();
    let string: String = "dawdawd0981740 54 !!@!@$ ui h2".into();
    terminal.push(&string);
    assert_eq!(terminal.buffer.borrow().as_str(),string);
  }
}
