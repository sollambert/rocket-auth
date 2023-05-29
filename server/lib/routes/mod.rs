
use rocket::{
    response::NamedFile
};
use std::path::{Path, PathBuf};
use std::io::Error;

#[get("/")]
pub fn index<'a>() -> &'a str {
    "Hello, world!"
}

#[get("/echo/<echo>")]
pub fn echo(echo: String) -> String {
    format!("{}", echo)
}

#[get("/images/<image..>")]
pub fn image_server<'a>(image: PathBuf) -> Result<NamedFile, Error> {
    NamedFile::open(Path::new("public/images").join(image))
}