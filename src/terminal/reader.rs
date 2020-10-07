use super::action::Action;
use super::handler::Handler;
use super::signal::{exit_signal_safe, prepare, take};
use super::terminal::Terminal;
use nix::sys::select::{select, FdSet};
use nix::sys::signal::Signal;
use std::io::{self, Read, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use termion::async_stdin;
use termion::event::{self, Event, Key};
use termios;

#[derive(Debug)]
pub struct Reader {
  saved_terminal: termios::Termios,
  leftover: Option<u8>,
  terminal: Terminal,
}
impl<'a> Drop for Reader {
  fn drop(&mut self) {
    use termios::*;
    let fd = io::stdout().as_raw_fd();
    tcsetattr(fd, TCSAFLUSH, &mut self.saved_terminal).unwrap();
  }
}
impl<'a> Reader {
  pub fn new() -> Reader {
    let saved_terminal = setup_terminal().unwrap();
    Reader {
      leftover: None,
      terminal: Terminal::new(),
      saved_terminal: saved_terminal,
    }
  }

  pub fn read_line(&mut self) -> Result<Action, io::Error> {
    self.terminal.write_prompt();
    self.terminal.display().unwrap();

    match prepare() {
      Ok(_) => {}
      Err(_) => {
        return Ok(Action::Exit);
      }
    }
    loop {
      if let Some(signal) = take() {
        match Handler::handle_signal(&mut self.terminal, signal) {
          Some(Ok(Action::Line(_line))) => {}
          Some(Ok(Action::Cancel)) => {
            return Ok(Action::Cancel);
          }
          Some(Ok(Action::Exit)) => {
            return Ok(Action::Exit);
          }
          Some(Ok(Action::Continue)) => {}
          Some(Err(e)) => {
            return Err(e);
          }
          None => {}
        }
      }
      if wait_input() {
        match self.read_char() {
          Some(Ok(res)) => {
            self.terminal.display().unwrap();
            match Handler::handle_event(&mut self.terminal, res.0) {
              Some(Ok(Action::Line(line))) => {
                self.terminal.display().unwrap();
                return Ok(Action::Line(line));
              }
              Some(Ok(Action::Exit)) => {
                return Ok(Action::Exit);
              }
              Some(Ok(Action::Cancel)) => {
                return Ok(Action::Exit);
              }
              Some(Ok(Action::Continue)) => {}
              Some(Err(e)) => {
                return Err(e);
              }
              None => {
                self.terminal.display().unwrap();
              }
            };
          }
          Some(Err(e)) => {
            return Err(e);
          }
          _ => {
            return Ok(Action::Cancel);
          }
        }
        // println!("{:?}", self.terminal.history);
      }
    }
  }
  fn read_char(&mut self) -> Option<Result<(Event, Vec<u8>), io::Error>> {
    let mut source = io::stdin();
    if let Some(c) = self.leftover {
      // we have a leftover byte, use it
      self.leftover = None;
      return Some(parse_event(c, &mut source.bytes()));
    }
    let mut buf = [0u8; 2];
    let res = match source.read(&mut buf) {
      Ok(0) => {
        return None;
      }
      Ok(1) => match buf[0] {
        b'\x1B' => Ok((Event::Key(Key::Esc), vec![b'\x1B'])),
        c => parse_event(c, &mut source.bytes()),
      },
      Ok(2) => {
        let option_iter = &mut Some(buf[1]).into_iter();
        let result = {
          let mut iter = option_iter.map(|c| Ok(c)).chain(source.bytes());
          parse_event(buf[0], &mut iter)
        };
        self.leftover = option_iter.next();
        result
      }
      Ok(_) => unreachable!(),
      Err(e) => Err(e),
    };
    Some(res)
  }
}
fn setup_terminal() -> io::Result<termios::Termios> {
  use termios::*;
  let fd = io::stdout().as_raw_fd();
  let mut termios = Termios::from_fd(fd).unwrap();
  let saved_terminal = termios.clone();
  termios.c_cflag |= CREAD | CLOCAL;
  termios.c_lflag &= !(ICANON | ECHO);
  termios.c_oflag &= !OPOST;
  termios.c_iflag &= !(INLCR | ICRNL);

  termios.c_cc[VMIN] = 0;
  termios.c_cc[VTIME] = 0;
  termios::tcsetattr(fd, TCSANOW, &mut termios).unwrap();
  Ok(saved_terminal)
}
fn wait_input() -> bool {
  let stdin_fileno = io::stdout().as_raw_fd();
  let mut r_fds = FdSet::new();
  r_fds.insert(stdin_fileno);

  let mut e_fds = FdSet::new();

  loop {
    match select(
      stdin_fileno + 1,
      Some(&mut r_fds),
      None,
      Some(&mut e_fds),
      None.as_mut(),
    ) {
      Ok(n) => return n == 1,
      Err(_e) => return false,
    }
  }
}
fn parse_event<I>(item: u8, iter: &mut I) -> Result<(Event, Vec<u8>), io::Error>
where
  I: Iterator<Item = Result<u8, io::Error>>,
{
  let mut buf = vec![item];
  let result = {
    let mut iter = iter.inspect(|byte| {
      if let &Ok(byte) = byte {
        buf.push(byte);
      }
    });
    event::parse_event(item, &mut iter)
  };
  result
    .or(Ok(Event::Unsupported(buf.clone())))
    .map(|e| (e, buf))
}
