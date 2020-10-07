use std::io;
pub fn move_to(n: usize) -> String {
  format!("\x1b[{}G", n)
}

pub fn move_up(n: usize) -> String {
  format!("\x1b[{}A", n)
}

pub fn move_down(n: usize) -> String {
  format!("\x1b[{}B", n)
}

pub fn move_right(n: usize) -> String {
  format!("\x1b[{}C", n)
}

pub fn move_left(n: usize) -> String {
  format!("\x1b[{}D", n)
}

pub fn move_under_line_first(n: usize) -> String {
  format!("\x1b[{}E", n)
}

pub fn clear_to_screen_end() -> String {
  "\x1b[0J".into()
}
