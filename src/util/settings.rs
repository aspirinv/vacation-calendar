use std::env::{var, VarError};

#[derive(Clone)]
pub struct Settings{
    pub db_connection_string: String,
    pub auth_secret: String,
    pub auth_client: String
}

impl Settings {
    pub fn init() -> Settings{
        Settings { 
            db_connection_string: var("db_connection_string").expect("Var db_connection_string not found"),
            auth_secret: var("auth_secret").expect("Var auth_secret not found"),
            auth_client: var("auth_client").expect("Var auth_client not found")
        }
    }
}
