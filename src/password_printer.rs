extern crate console;
use self::console::Term;

pub fn print_password_titles(titles: Vec<String>) {
    let term = Term::stdout();
    for title in titles {
        let success  = term.write_line(title.as_ref());
        if success.is_err() {
            println!("Error outputing titles!");
        }
    }
}
