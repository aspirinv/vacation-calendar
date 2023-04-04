use actix_files::NamedFile;
use actix_web::{get, HttpRequest, Responder};
use std::path::PathBuf;


#[get("/{filename}")]
pub async fn files(req: HttpRequest) -> impl Responder {
    let path: PathBuf = ("public/build/".to_owned() + req.match_info().query("filename")).parse().unwrap();
    NamedFile::open(path)
}

#[get("/")]
pub async fn index() -> impl Responder {
    NamedFile::open("public/index.html")
}