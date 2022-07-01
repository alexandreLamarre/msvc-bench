use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
enum Gender {
    A,
    B,
    C,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::A
    }
}
#[derive(Debug, Serialize, Deserialize)]
enum DocumentType {
    A,
    B,
    C,
}
#[derive(Serialize)]
struct Point {
    x: i32,
    y: i32,
}

impl Default for DocumentType {
    fn default() -> Self {
        DocumentType::A
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct User {
    user_id: String,
    username: String,
    password: String,
    gender: Gender,
    docuemnt_type: DocumentType,
    document_num: u32,
    email: String,
}

pub fn save_user() {}

fn create_default_auth_user() {}

pub fn get_all_users() {}

pub fn find_by_user_name() {}

pub fn find_by_user_id() {}

pub fn delete_user() {}

pub fn update_user() {}

pub fn delete_user_auth() {}
