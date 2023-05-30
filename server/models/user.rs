use core::fmt;
use std::{io::Cursor, str::FromStr};
use std::error::Error;
use argon2::PasswordVerifier;
use rocket::{post, serde::json::{Json, self}, response::Responder, Request, Response};
use std::io::ErrorKind;
use serde_derive::{Deserialize, Serialize};
use tokio_postgres::{types::ToSql, Row};

use argon2::{
    password_hash::{
        PasswordHash
    },
    Argon2
};

use crate::routes::auth::{LoginError, password_hasher};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
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