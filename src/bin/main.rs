use shell::parser::parser::Parser;
use shell::terminal::action::Action;
use shell::terminal::reader::Reader;
fn main() {
  let mut reader = Reader::new();
  loop {
    match reader.read_line() {
      Ok(Action::Cancel) => {
        continue;
      }
      Ok(Action::Continue) => {}
      Ok(Action::Exit) => {
        break;
      }
      Ok(Action::Line(input)) => {
        let mut parser = Parser::new(input);
        parser.build();
      }
      Err(_e) => {
        break;
      }
    }
  }
}
