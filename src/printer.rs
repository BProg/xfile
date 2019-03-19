extern crate console;
use self::console::Term;
use password_record::PasswordRecord;

pub fn print_record_name_and_password(records: Vec<PasswordRecord>) {
  let term = Term::stdout();
  for record in records {
    let result = term.write_line(format_name_and_password(&record).as_ref());
    if result.is_err() {
      println!("Cannot output titles!");
    }
  }
}

pub fn print_record_password(records: Vec<PasswordRecord>) {
  let term = Term::stdout();
  for record in records {
    let result = term.write_line(format_password(&record).as_ref());
    if result.is_err() {
      println!("Cannot output titles!");
    }
  }
}

pub fn print_record_name(records: Vec<PasswordRecord>) {
  let term = Term::stdout();
  for record in records {
    let result = term.write_line(format_name(&record).as_ref());
    if result.is_err() {
      println!("Cannot output titles!");
    }
  }
}

fn format_name(record: &PasswordRecord) -> String {
  format!("{}", record.name)
}

fn format_password(record: &PasswordRecord) -> String {
  format!("{} - {}", record.password_label, record.password)
}

fn format_name_and_password(record: &PasswordRecord) -> String {
  format!("{} - {}", record.name, record.password)
}
