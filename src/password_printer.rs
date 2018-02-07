extern crate console;
use self::console::Term;
use password_records::PasswordRecord;

pub fn print_password_records_names_passwords(records: Vec<PasswordRecord>) {
    let term = Term::stdout();
    for record in records {
        let result = term.write_line(configure_passwords_and_names(&record).as_ref());
        if result.is_err() {
            println!("Cannot output titles!");
        }
    }
}

pub fn print_password_records_passwords(records: Vec<PasswordRecord>) {
    let term = Term::stdout();
    for record in records {
        let result = term.write_line(configure_password_output(&record).as_ref());
        if result.is_err() {
            println!("Cannot output titles!");
        }
    }
}

pub fn print_password_records_titles(records: Vec<PasswordRecord>) {
    let term = Term::stdout();
    for record in records {
        let result = term.write_line(configure_names_output(&record).as_ref());
        if result.is_err() {
            println!("Cannot output titles!");
        }
    }
}

fn configure_names_output(record: &PasswordRecord) -> String {
    format!("{}", record.name)
}

fn configure_password_output(record: &PasswordRecord) -> String {
    format!("{} - {}",record.password_label, record.password)
}

fn configure_passwords_and_names(record: &PasswordRecord) -> String {
    format!("{} - {}",record.name, record.password)
}
