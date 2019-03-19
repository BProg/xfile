extern crate xfile;
use xfile::password_record::PasswordRecord;
use xfile::app;
use xfile::printer;

fn main() {
  let app = app::create_app();
  let matches = app.get_matches();
  match matches.subcommand() {
    ("show", Some(show_cmd)) => {
      if show_cmd.is_present("names") && show_cmd.is_present("passwords") {
        let records = read_records();
        printer::print_record_name_and_password(records);
      } else if show_cmd.is_present("passwords") {
        let records = read_records();
        printer::print_record_password(records);
      } else if show_cmd.is_present("names") {
        let records = read_records();
        printer::print_record_name(records);
      }
    }
    _ => {}
  }
}

fn read_records() -> Vec<PasswordRecord> {
  let mut records: Vec<PasswordRecord> = Vec::new();
  let csv = csv::Reader::from_file("/Users/iostafi/Documents/Documents_offline/pk_backup_2016-09-30.txt");
  if let Ok(mut passwords) = csv {
    for password in passwords.decode() {
      if let Ok(pass) = password {
        let record: PasswordRecord = pass;
        records.push(record);
      }
    }
  }
  records
}
