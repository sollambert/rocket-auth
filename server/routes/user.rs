use std::io::{Error, Cursor};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub hashed_password: String
}


impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        let (hashed_password, _) = password_hasher(password.as_bytes());
        User {
            id: 0,
            username,
            email,
            hashed_password
        }
    }
    pub fn from_insertable(insertable: InsertableUser) -> Self {
        User::new(insertable.username, insertable.email, insertable.password)
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

impl<'r> Responder<'r, 'static> for User {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let serialized = json::serde_json::to_string_pretty(&ResponseUser::new(self)).unwrap();
        Ok(Response::build()
        .status(rocket::http::Status::Created)
        .sized_body(serialized.len(), Cursor::new(serialized))
        .finalize())
    }
}

pub fn password_hasher(password: &[u8]) -> (String, SaltString)  {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, salt.as_salt()).unwrap();
    (password_hash.to_string(), salt)
}

#[post("/users/register", format = "json", data = "<user>")]
pub async fn register_user(user: Json<InsertableUser>) -> Result<User, Error> {
    let generated_user = User::from_insertable(user.into_inner());
    let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
    params.push(&generated_user.username);
    params.push(&generated_user.email);
    params.push(&generated_user.hashed_password);
    match pool::execute("
    INSERT INTO users (username, email, password)
    VALUES ($1, $2, $3)", &params).await {
        Ok(_) => {
            Ok(generated_user)
        }
        Err(err) => {
            Err(Error::new(ErrorKind::InvalidData, err))
        }
    }
}

#[post("/users/login", format = "json", data = "<user>")]
pub async fn login_user(user: Json<LoginUser>) -> Result<User, Error> {
    let login_user = user.into_inner();
    let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
    params.push(&login_user.username);
    match pool::query("
    SELECT * from users
    WHERE username = $1;", params).await {
        Ok(rows) => {
            for row in rows.iter() {
                let user = User::from(row);
                if user.validate_password(&login_user.password) {
                    println!("user validated");
                }
            }
            Ok(User::from_insertable(InsertableUser {username: login_user.username, email: "test".to_string(), password: login_user.password}))
        }
        Err(err) => {
            Err(Error::new(ErrorKind::InvalidData, err))
        }
    }
}