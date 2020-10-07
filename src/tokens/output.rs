use std::fs;
use std::io;

#[derive(Debug)]
pub enum Output {
  File(fs::File),
  Stdout(io::Stdout),
}

impl From<io::Stdout> for Output {
  fn from(output: io::Stdout) -> Self {
    Output::Stdout(output)
  }
}

impl From<fs::File> for Output {
  fn from(file: fs::File) -> Self {
    Output::File(file)
  }
}

impl Clone for Output {
  fn clone(&self) -> Self {
      match *self {
          Output::File(ref file) => Output::File(file.try_clone().unwrap()),
          Output::Stdout(_) => Output::Stdout(io::stdout()),
      }
  }
}


impl io::Write for Output {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    match *self {
      Output::File(ref mut f) => f.write(buf),
      Output::Stdout(ref mut stdout) => stdout.write(buf),
    }
  }
  fn write_all(&mut self,buf: &[u8]) -> io::Result<()>{
    match *self{
      Output::File(ref mut f)=>f.write_all(buf),
      Output::Stdout(ref mut stdout)=>stdout.write_all(buf)
    }
  }
  fn flush(&mut self)->io::Result<()>{
    match *self{
      Output::File(ref mut f)=>f.flush(),
      Output::Stdout(ref mut stdout)=>stdout.flush()
    }
  }
}
