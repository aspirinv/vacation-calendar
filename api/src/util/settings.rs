use std::env::{var, VarError};

pub struct Settings{
    pub db_connection_string: String
}

impl Settings {
    pub fn init() -> Result<Settings, VarError>{
        Ok(Settings { 
            db_connection_string: var("db_connection_string")?
        })
    }
}
