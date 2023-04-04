use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use mongodb::{Client, options::ClientOptions};

use std::sync::Arc;
use log::{error, info};

 mod domain;
 mod controller;

 mod util;
 use util::settings::Settings;
// use util::state::State;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "debug, actix_web=debug");
    env_logger::init();

    info!("Getting settings");
   
    let settings = Settings::init().unwrap();

    // let client_options = ClientOptions::parse(settings.db_connection_string).await.unwrap();
    // let client = Client::with_options(client_options).unwrap();
    // let database = client.database("savier");
    
    HttpServer::new(move || {
        App::new()
            // .app_data(Data::new(State{ db: Arc::new(database.clone())}))
            .wrap(Logger::default())
            .configure(controller::config)
            
    })
    .bind("127.0.0.1:58080")?
    .run()
    .await
}