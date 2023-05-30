use core::fmt;
use std::{io::Cursor, str::FromStr};
use std::error::Error;
use rocket::{post, serde::json::{Json, self}, response::Responder, Request, Response};
use std::io::ErrorKind;
use serde_derive::{Deserialize, Serialize};
use tokio_postgres::{types::ToSql, Row};

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::pool;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUser {
    pub id: i32,
    pub username: String,
    pub email: String
}

impl ResponseUser {
    pub fn new(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email
        }
    }
}

impl<'r> Responder<'r, 'static> for ResponseUser {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let serialized = json::serde_json::to_string_pretty(&self).unwrap();
        Ok(Response::build()
        .status(rocket::http::Status::Created)
        .sized_body(serialized.len(), Cursor::new(serialized))
        .finalize())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub hashed_password: String
}

#[derive(Serialize, Debug)]
pub enum LoginError {
    InvalidData,
    UsernameDoesNotExist,
    WrongPassword
}

impl Error for LoginError {}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidData => 
                write!(f, "Invalid user data provided"),
            Self::UsernameDoesNotExist => 
                write!(f, "Provided username does not exist"),
            Self::WrongPassword => 
                write!(f, "Password provided is incorrect"),
        }
    }
}

impl<'r> Responder<'r, 'static> for LoginError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let serialized = json::serde_json::to_string_pretty(&self).unwrap();
        Ok(Response::build()
        .status(rocket::http::Status::Unauthorized)
        .sized_body(serialized.len(), Cursor::new(serialized))
        .finalize())
    }
}

impl User {
    pub fn new(id: Option<i32>, username: String, email: String, password: String) -> Self {
        let (hashed_password, _) = password_hasher(password.as_bytes());
        Self {
            id: id.unwrap_or(0),
            username,
            email,
            hashed_password
        }
    }
    pub fn from_insertable(insertable: InsertableUser) -> Self {
        User::new(None, insertable.username, insertable.email, insertable.password)
    }
    pub fn validate_password(&self, password: &String) -> bool {
        let hash: PasswordHash = PasswordHash::new(self.hashed_password.as_str()).unwrap();
        Argon2::default().verify_password(password.as_bytes(), &hash ).is_ok()
    }
    pub fn get_response_user(self) -> ResponseUser {
        ResponseUser::new(self)
    }
}

impl<'r> From<&'r Row> for User {
    fn from(row: &'r Row) -> Self {
        Self {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            hashed_password: row.get("password")
        }
    }
}

pub fn password_hasher(password: &[u8]) -> (String, SaltString)  {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, salt.as_salt()).unwrap();
    (password_hash.to_string(), salt)
}

#[post("/register", format = "json", data = "<user>")]
pub async fn register_user(user: Json<InsertableUser>) -> Result<ResponseUser, LoginError> {
    let generated_user = User::from_insertable(user.into_inner());
    let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
    params.push(&generated_user.username);
    params.push(&generated_user.email);
    params.push(&generated_user.hashed_password);
    match pool::execute("
    INSERT INTO users (username, email, password)
    VALUES ($1, $2, $3)", params).await {
        Ok(_) => {
            Ok(ResponseUser::new(generated_user))
        }
        Err(_) => {
            Err(LoginError::InvalidData)
        }
    }
}

#[post("/login", format = "json", data = "<user>")]
pub async fn login_user(user: Json<LoginUser>) -> Result<ResponseUser, LoginError> {
    let login_user = user.into_inner();
    let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
    params.push(&login_user.username);
    match pool::query("
    SELECT * from users
    WHERE username = $1;", params).await {
        Ok(rows) => {
            if rows.len() == 0 {
                return Err(LoginError::UsernameDoesNotExist)
            }
            for row in rows.iter() {
                let user = User::from(row);
                if user.validate_password(&login_user.password) {
                    println!("user validated");
                    return Ok(ResponseUser::new(user))
                }
            }
            Err(LoginError::WrongPassword)
        }
        Err(_) => {
            Err(LoginError::InvalidData)
        }
    }
}