use super::action::Action;
use super::cursor::{move_down, move_left, move_right, move_up};
use super::signal::exit_signal_safe;
use super::terminal::Terminal;
use nix::sys::signal::Signal;
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;
use std::string::ToString;
use termion::event::{Event, Key};
#[derive(Clone, Debug)]
pub struct Handler {}

impl Handler {
  pub fn handle_signal(
    terminal: &mut Terminal,
    signal: Signal,
  ) -> Option<Result<Action, io::Error>> {
    match signal {
      Signal::SIGINT => {
        terminal.history.clear_buffer();
        terminal.reset();
        terminal.write_linefeed();
        terminal.write_line();
        return Some(Ok(Action::Cancel));
      }
      Signal::SIGTSTP | Signal::SIGQUIT => {
        terminal.reset();
        terminal.history.clear_buffer();
        return Some(Ok(Action::Exit));
      }
      Signal::SIGCONT => {
        return None;
      }
      _ => None,
    }
  }
  pub fn handle_event(terminal: &mut Terminal, event: Event) -> Option<Result<Action, io::Error>> {
    match event {
      Event::Key(key) => {
        return Handler::handle_keypress(terminal, key);
      }
      _ => None,
    }
  }
  fn handle_keypress(terminal: &mut Terminal, key: Key) -> Option<Result<Action, io::Error>> {
    let mut buf = [0u8; 10];
    match key {
      Key::Ctrl('d') => {
        terminal.reset();
        terminal.history.clear_buffer();
        return Some(Ok(Action::Exit));
      }
      Key::Delete => {
        terminal.delete_right(1);
      }
      Key::Backspace => {
        terminal.delete_left(1);
      }
      Key::Home | Key::Ctrl('a') => {
        terminal.move_to_first();
      }
      Key::End | Key::Ctrl('e') => {
        terminal.move_to_end();
      }
      Key::Left | Key::Ctrl('b') => {
        terminal.move_left(1);
      }
      Key::Right | Key::Ctrl('f') => {
        terminal.move_right(1);
      }
      Key::Up | Key::Ctrl('p') => {
        if terminal.history.is_end() {
          terminal.set_buffer();
        }
        let cmd = match terminal.history.prev() {
          Some(c) => c.clone(),
          None => {
            return None;
          }
        };
        terminal.update_buffer(&cmd);
        terminal.move_to(0);
        terminal.clear_to_screen_end();
        terminal.write_buffer();
        terminal.move_to_end();
      }
      Key::Down | Key::Ctrl('n') => {
        let cmd = match terminal.history.next() {
          Some(c) => c.clone(),
          None => {
            return None;
          }
        };
        if terminal.history.is_end() {
          terminal.history.clear_buffer();
        }
        terminal.update_buffer(&cmd);
        terminal.move_to(0);
        terminal.clear_to_screen_end();
        terminal.write_buffer();
        terminal.move_to_end();
      }
      Key::Char('\n') | Key::Ctrl('j') | Key::Ctrl('m') => match terminal.get() {
        Some(line) => {
          terminal.history.push(line.clone());
          terminal.history.clear_buffer();
          terminal.reset();
          terminal.write_linefeed();
          terminal.write_line();
          return Some(Ok(Action::from(line)));
        }
        None => {
          terminal.history.clear_buffer();
          terminal.reset();
          terminal.write_linefeed();
          terminal.write_line();
          return Some(Ok(Action::Cancel));
        }
      },
      Key::Char(c) => {
        terminal.push(c.encode_utf8(&mut buf));
      }
      Key::Ctrl(c) => match c {
        'l' => {
          terminal.clear_screen();
          terminal.write_buffer();
        }
        _ => {}
      },
      Key::Esc | Key::Insert => {}
      Key::Alt(_c) => {}
      Key::F(_f) => {}
      _ => unreachable!(),
    };
    None
  }
}

impl Handler {
  fn write(s: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    write!(lock, "{}", s)?;
    lock.flush()
  }
}
