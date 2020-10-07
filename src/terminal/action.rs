pub enum Action {
  Line(String),
  Exit,
  Cancel,
  Continue,
}
impl From<String> for Action {
  fn from(line: String) -> Self {
    Action::Line(line)
  }
}
impl Clone for Action {
  fn clone(&self) -> Self {
    match *self {
      Action::Line(ref line) => Action::Line(line.clone()),
      Action::Exit => Action::Exit,
      Action::Cancel => Action::Cancel,
      Action::Continue => Action::Continue,
    }
  }
}
