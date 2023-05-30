use std::io::Error;

use rocket::{post, serde::json::Json, response::Responder, Request, Response};
use std::io::ErrorKind;
use serde_derive::{Deserialize, Serialize};
use tokio_postgres::types::ToSql;
use uuid::Uuid;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::pool;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub hashed_password: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUser {
    pub id: String,
    pub username: String,
    pub email: String
}


impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        let (hashed_password, _) = password_hasher(password.as_bytes());
        User {
            id: Uuid::new_v4(),
            username,
            email,
            hashed_password
        }
    }
    pub fn from_insertable(insertable: InsertableUser) -> Self {
        User::new(insertable.username, insertable.email, insertable.password)
    }
    pub fn validate_password(&self, password: &String) {
        let hash: PasswordHash = PasswordHash::new(&password.as_str()).unwrap();
        Argon2::default().verify_password(password.as_bytes(), &hash ).unwrap();
    }
}

impl<'r> Responder<'r, 'static> for User {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let mut response = Response::new();
        response.set_status(rocket::http::Status::Created);
        Ok(response)
    }
}

pub fn password_hasher(password: &[u8]) -> (String, SaltString)  {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, salt.as_salt()).unwrap();
    (password_hash.to_string(), salt)
}

#[post("/users", format = "json", data = "<user>")]
pub async fn register_user(user: Json<InsertableUser>) -> Result<User, Error> {
    let generated_user = User::from_insertable(user.into_inner());
    let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
    let id = generated_user.id.to_string();
    params.push(&id);
    params.push(&generated_user.username);
    params.push(&generated_user.email);
    params.push(&generated_user.hashed_password);
    println!("{:?}", params);
    match pool::execute("
    INSERT INTO users (id, username, email, password)
    VALUES ($1, $2, $3, $4)", &params).await {
        Ok(_) => {
            Ok(generated_user)
        }
        Err(err) => {
            Err(Error::new(ErrorKind::InvalidData, err))
        }
    }
}