use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct CreateUserData {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignInUserData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignInResponse {
    pub message: String,
}

#[derive(Type, Serialize, Deserialize, Debug)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
    Moderator,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub role: UserRole,
    pub password: String,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let role = match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
            UserRole::Moderator => "moderator",
        };
        write!(f, "{}", role)
    }
}
