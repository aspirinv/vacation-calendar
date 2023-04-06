use actix_files::NamedFile;
use actix_web::{get, HttpRequest, Responder};
use std::path::PathBuf;

use crate::util::error::JsonError;

pub async fn files(req: HttpRequest) -> Result<NamedFile, JsonError> {
    let path: PathBuf = ("public/".to_owned() + req.match_info().query("filename")).parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[get("/")]
pub async fn index() -> impl Responder {
    NamedFile::open("public/index.html")
}