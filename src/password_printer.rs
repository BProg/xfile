extern crate console;
use self::console::Term;
use password_records::PasswordRecord;

pub fn print_password_titles(titles: Vec<String>) {
    let term = Term::stdout();
    for title in titles {
        let success  = term.write_line(title.as_ref());
        if success.is_err() {
            println!("Error outputing titles!");
        }
    }
}

pub fn print_password_records(records: Vec<PasswordRecord>) {
    let term = Term::stdout();
    for record in records {
        let success  = term.write_line(record.name.as_ref());
        if success.is_err() {
            println!("Error outputing titles!");
        }
    }
}
