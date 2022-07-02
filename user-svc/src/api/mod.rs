use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
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

impl Default for DocumentType {
    fn default() -> Self {
        DocumentType::A
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct User {
    user_id: String,
    username: String,
    password: String,
    gender: Gender,
    docuemnt_type: DocumentType,
    document_num: u32,
    email: String,
}

pub fn save_user(Json(payload): Json<User>) -> impl IntoResponse {
    let user: User = payload;

    (StatusCode::CREATED, Json(user))
}

fn create_default_auth_user() {}

pub fn get_all_users() -> impl IntoResponse {
    let users: Vec<User> = vec![];
    (StatusCode::OK, Json(users))
}

pub fn find_by_user_name() -> impl IntoResponse {
    (StatusCode::OK)
}

pub fn find_by_user_id() -> impl IntoResponse {
    (StatusCode::OK)
}

pub fn delete_user() -> impl IntoResponse {
    (StatusCode::ACCEPTED)
}

pub fn update_user() -> impl IntoResponse {
    (StatusCode::OK)
}

pub fn delete_user_auth() -> impl IntoResponse {
    (StatusCode::ACCEPTED)
}
