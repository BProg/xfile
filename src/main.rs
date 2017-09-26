extern crate clap;
use clap::{Arg, App};
extern crate csv;
extern crate rustc_serialize;
mod password_printer;

#[derive(RustcDecodable)]
#[derive(Debug)]
struct PasswordRecord {
    url: String,
    username: String,
    password: String,
    extra: String,
    name: String,
    grouping: String,
    fav: u32,
    custom_fields: String,
    last_modified_time: i64,
    uid: i64,
    username_label: String,
    password_label: String,
    website_label: String,
    notes_label: String,
    password_set_date: u64,
    flags: i32,
    image_index: i32,
    data_version: i32
}

fn main() {
    let mut passes: Vec<String> = Vec::new();
    passes.push(String::from("ebay"));
    passes.push(String::from("amazon"));
    password_printer::print_password_titles(passes);
}

#[allow(dead_code)]
fn read_and_output_passwords() {
    let passwords = csv::Reader::from_file("/Users/iostafi/Documents/Documents_offline/pk_backup_2016-09-30.txt");
    if let Ok(mut ps) = passwords {
        for p in ps.decode() {
            if let Ok(pass) = p {
                let record: PasswordRecord = pass;
                println!("{:?}", record);
            }
        }
    }
}

#[allow(dead_code)]
fn parse_input_arguments() {
    let matches = App::new("zfiles")
        .version("0.1.0")
        .author("Ion Ostafi <ostafi_ion@yahoo.com>")
        .about("tool for saving passwords")
        .get_matches();
}
