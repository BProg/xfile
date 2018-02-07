extern crate clap;
use clap::{Arg, App, SubCommand};
extern crate csv;
extern crate rustc_serialize;
mod password_printer;
mod password_records;
use password_records::PasswordRecord;

fn main() {
    let app = create_app();
    let matches = app.get_matches();
    match matches.subcommand() {
        ("show", Some(show_cmd)) => {
            if show_cmd.is_present("--names") {
                let records = read_records();
                password_printer::print_password_records_titles(records);
            }
        },
        _ => {},
    }
}

fn read_records() -> Vec<PasswordRecord> {
    let mut password_records: Vec<PasswordRecord> = Vec::new();
    let csv_values = csv::Reader::from_file("/Users/iostafi/Documents/Documents_offline/pk_backup_2016-09-30.txt");
    if let Ok(mut passwords) = csv_values {
        for password in passwords.decode() {
            if let Ok(pass) = password {
                let record: PasswordRecord = pass;
                password_records.push(record);
            }
        }
    }
    password_records
}

fn create_app<'a>() -> App<'a, 'a> {
    App::new("zfiles")
        .version("0.1.0")
        .author("Ion Ostafi <ostafi_ion@yahoo.com>")
        .about("tool for saving passwords")
        .subcommand(
            SubCommand::with_name("show")
            .about("print passwords")
            .subcommand(
                SubCommand::with_name("--names")
            )
        )
}
