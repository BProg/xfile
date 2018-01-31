#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct PasswordRecord {
    pub url: String,
    pub username: String,
    pub password: String,
    pub extra: String,
    pub name: String,
    pub grouping: String,
    pub fav: u32,
    pub custom_fields: String,
    pub last_modified_time: i64,
    pub uid: i64,
    pub username_label: String,
    pub password_label: String,
    pub website_label: String,
    pub notes_label: String,
    pub password_set_date: u64,
    pub flags: i32,
    pub image_index: i32,
    pub data_version: i32
}
