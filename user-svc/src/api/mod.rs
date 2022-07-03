use crate::ServiceState;
use axum::{extract::Extension, response::IntoResponse, Json};
use futures::stream::TryStreamExt;
use hyper::StatusCode;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
#[derive(Debug, Deserialize, Serialize)]
pub struct UserNameRequest {
    username: String,
}

pub struct UserIdRequest {
    user_id: String,
}

pub async fn save_user(
    Json(payload): Json<User>,
    state: Extension<Arc<ServiceState>>,
) -> impl IntoResponse {
    let resp = find_by_user_name(
        Json(UserNameRequest {
            username: payload.username.clone(),
        }),
        state.clone(),
    )
    .await;
    // if let Ok(resp) = resp {
    //     if resp.status() == StatusCode::OK {
    //         return StatusCode::CONFLICT;
    //     }
    //     if resp.status() != StatusCode::NOT_FOUND {
    //         return StatusCode::INTERNAL_SERVER_ERROR;
    //     }
    // }

    let user_db = state.mongodb.database("users");
    let collection = user_db.collection::<User>("users");
    collection.insert_one(payload, None).await;
    StatusCode::CREATED
}

pub async fn get_all_users() -> impl IntoResponse {
    let users: Vec<User> = vec![];
    (StatusCode::OK, Json(users))
}

pub async fn find_by_user_name(
    Json(payload): Json<UserNameRequest>,
    state: Extension<Arc<ServiceState>>,
) -> impl IntoResponse {
    let user_db = state.mongodb.database("users");
    let collection = user_db.collection::<User>("users");

    let filter = doc! { "username": payload.username };
    let mut cursor = match collection.find(filter, None).await {
        Err(e) => {
            println!("{}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(User::default()));
        }
        Ok(curs) => curs,
    };
    match cursor.try_next().await {
        Ok(res) => {
            if let Some(res) = res {
                (StatusCode::OK, Json(res))
            } else {
                (StatusCode::NOT_FOUND, Json(User::default()))
            }
        }
        Err(e) => {
            println!("{}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(User::default()))
        }
    }
}

pub async fn find_by_user_id(
    Json(payload): Json<UserIdRequest>,
    state: Extension<Arc<ServiceState>>,
) -> impl IntoResponse {
    let user_db = state.mongodb.database("users");
    let collection = user_db.collection::<User>("users");

    let filter = doc! { "username": payload.user_id };
    let mut cursor = match collection.find(filter, None).await {
        Err(e) => {
            println!("{}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response();
        }
        Ok(curs) => curs,
    };
    match cursor.try_next().await {
        Ok(res) => {
            if let Some(res) = res {
                (StatusCode::OK, Json(res)).into_response()
            } else {
                (StatusCode::NOT_FOUND).into_response()
            }
        }
        Err(e) => {
            println!("{}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response()
        }
    }
}

pub async fn delete_user() -> impl IntoResponse {
    (StatusCode::ACCEPTED).into_response()
}

pub async fn update_user() -> impl IntoResponse {
    (StatusCode::OK).into_response()
}

pub async fn delete_user_auth() -> impl IntoResponse {
    (StatusCode::ACCEPTED).into_response()
}

async fn create_default_auth_user() {}
