
use rocket::{
    get,
    fs::NamedFile
};
use tokio_postgres::types::ToSql;
use std::path::{Path, PathBuf};
use std::io::Error;
use std::io::ErrorKind;

use crate::pool;

pub mod auth;
pub mod user;

#[get("/")]
pub async fn index() -> Result<String, Error> {
    let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
    struct User {
        id: i32
    }
    let test = User {id: 1};
    params.push(&test.id);
    match pool::query("
    SELECT * from users
    WHERE id = $1", params).await {
        Ok(rows) => {
            let username: &str = rows[0].get(0);
            Ok(username.to_string())
        }
        Err(err) => {
            Err(Error::new(ErrorKind::InvalidData, err))
        }
    }
}

#[get("/echo/<echo>")]
pub fn echo<'r>(echo: String) -> Result<String, Error> {
    Ok(echo)
}

#[get("/images/<image..>")]
pub async fn image_server<'a>(image: PathBuf) -> Result<NamedFile, Error> {
    NamedFile::open(Path::new("public/images").join(image)).await
}