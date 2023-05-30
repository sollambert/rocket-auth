use core::fmt;
use std::error::Error;
use rocket::{post, serde::json::{Json}};

use serde_derive::Serialize;
use tokio_postgres::types::ToSql;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

use crate::models::user::{User, LoginUser, ResponseUser, InsertableUser};
use crate::pool;

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