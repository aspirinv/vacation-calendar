use actix_web::web;

pub mod file;
//pub mod requests;

pub fn config(cfg: &mut web::ServiceConfig){
    //cfg.service(requests::list);
    
    cfg.service(file::index);
    cfg.service(file::files);

}