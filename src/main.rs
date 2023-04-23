use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;

use mongodb::{Client, options::ClientOptions };

use std::sync::Arc;
use log::{error, info};

 mod domain;
 mod controller;

 mod util;
 use util::settings::Settings;

use crate::util::auth_header::AuthHeader;

// use util::state::State;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "debug, actix_web=debug");
    env_logger::init();

    info!("Getting settings");
   
    let settings = Settings::init();

//https://github.com/ramosbugs/oauth2-rs/blob/main/examples/google.rs
//https://blog.logrocket.com/9-rust-authentication-libraries-that-are-ready-for-production/

    // let client_options = ClientOptions::parse(settings.db_connection_string).await.unwrap();
    // let client = Client::with_options(client_options).unwrap();
    // let database = client.database("savier");

    HttpServer::new(move || {        
        App::new()        
            // .app_data(Data::new(State{ db: Arc::new(database.clone())}))
            .app_data(Data::new(settings.clone()))
            .wrap(AuthHeader(settings.auth_secret.to_string()))
            .wrap(Logger::default())
            .configure(controller::config)
            
    })
    .bind("127.0.0.1:58080")?
    .run()
    .await
}