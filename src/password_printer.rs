extern crate console;
use self::console::Term;
use password_records::PasswordRecord;

pub fn print_password_records_titles(records: Vec<PasswordRecord>) {
    let term = Term::stdout();
    for record in records {
        let result = term.write_line(record.name.as_ref());
        if result.is_err() {
            println!("Cannot output titles!");
        }
    }
}
