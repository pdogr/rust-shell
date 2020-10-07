use std::iter::FromIterator;
#[derive(Clone, Default, Debug)]
pub struct Buffer {
  data: Vec<char>,
}
impl FromIterator<char> for Buffer {
  fn from_iter<T: IntoIterator<Item = char>>(t: T) -> Self {
    Self {
      data: t.into_iter().collect(),
    }
  }
}
impl From<Buffer> for String {
  fn from(buf: Buffer) -> Self {
    Self::from_iter(buf.data)
  }
}

impl From<String> for Buffer {
  fn from(s: String) -> Self {
    Self::from_iter(s.chars())
  }
}

impl<'a> From<&'a str> for Buffer {
  fn from(s: &'a str) -> Self {
    Self::from_iter(s.chars())
  }
}
impl Buffer {
  pub fn new() -> Self {
    Self { data: Vec::new() }
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn as_str(&self) -> String {
    String::from(self.clone())
  }

  pub fn remove(&mut self, start: usize, end: usize) -> Vec<char> {
    self.data.drain(start..end).collect()
  }

  pub fn insert_str(&mut self, idx: usize, string: &str) {
    for (i, c) in string.chars().enumerate() {
      self.data.insert(idx + i, c)
    }
  }
  pub fn clear(&mut self) {
    self.data = Vec::new();
  }
}

#[cfg(test)]
pub mod terminal_test {
  use super::Buffer;

  #[test]
  fn from_test() {
    let string: String = "awdwad7*&D*WA%^V57a a w  a\n".into();
    let buffer = Buffer::from(string.clone());
    for (buf_data, str_data) in buffer.data.iter().zip(string.chars()) {
      assert_eq!(*buf_data, str_data);
    }
  }
  #[test]
  fn as_str_test() {
    let string: String = "awdwad7*&dwa wad wa wad wda w  a\n".into();
    let buffer = Buffer::from(string.clone());
    assert_eq!(buffer.as_str(), string);
  }
  #[test]
  fn insert_str_test() {
    let string: String = "Hello other side".into();
    let target: String = "Hello from the other side".into();
    let mut buffer = Buffer::from(string.clone());
    buffer.insert_str(6, "from the ".into());
    assert_eq!(buffer.as_str(), target);
  }
  #[test]
  fn remove_test() {
    let string: String = "Hello world hello".into();
    let target: String = "Hello llo".into();
    let mut buffer = Buffer::from(string.clone());
    buffer.remove(6, 14);
    assert_eq!(buffer.as_str(), target);
  }
  #[test]
  fn len_test() {
    let string: String = "awdwad7*&Ddawdawd a wd ad wa wd%^V57a a w  a\n".into();
    let buffer = Buffer::from(string.clone());
    assert_eq!(buffer.len(), string.len());
  }
  #[test]
  fn clear_test() {
    let string: String = "awdwad7*&Ddawdawd a wd ad wa wd%^V57a a w  a\n".into();
    let mut buffer = Buffer::from(string.clone());
    buffer.clear();
    assert_eq!(buffer.data, Vec::new());
  }
}
