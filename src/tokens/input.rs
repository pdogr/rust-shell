use std::io;
use std::fs;
#[derive(Debug)]
pub enum Input {
  File(fs::File),
  Stdin(io::Stdin),
}

impl From<io::Stdin> for Input {
  fn from(input: io::Stdin) -> Self {
    Input::Stdin(input)
  }
}

impl From<fs::File> for Input {
  fn from(file: fs::File) -> Self {
    Input::File(file)
  }
}

impl Clone for Input {
  fn clone(&self) -> Self {
      match *self {
          Input::File(ref file) => Input::File(file.try_clone().unwrap()),
          Input::Stdin(_) => Input::Stdin(io::stdin()),
      }
  }
}
impl io::Read for Input {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    match *self {
      Input::File(ref mut f) => f.read(buf),
      Input::Stdin(ref mut stdin) => stdin.read(buf),
    }
  }
}
