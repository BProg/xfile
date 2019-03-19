use super::PasswordRecord;

pub fn read_records() -> Vec<PasswordRecord> {
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
