extern crate xfile;
use xfile::{printer, reader};

mod app;

fn main() {
  let app = app::create_app();
  let matches = app.get_matches();
  match matches.subcommand() {
    ("show", Some(show_cmd)) => {
      if show_cmd.is_present("names") && show_cmd.is_present("passwords") {
        let records = reader::read_records();
        printer::print_record_name_and_password(records);
      } else if show_cmd.is_present("passwords") {
        let records = reader::read_records();
        printer::print_record_password(records);
      } else if show_cmd.is_present("names") {
        let records = reader::read_records();
        printer::print_record_name(records);
      }
    }
    _ => {
      println!("{}", matches.usage());
    }
  }
}
