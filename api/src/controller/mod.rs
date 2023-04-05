use actix_web::web;

pub mod file;
mod calendar;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(calendar::get);
    
    cfg.service(file::index);
    cfg.route("/{filename:.*}", web::get().to(file::files));

}