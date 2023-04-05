use chrono::{NaiveDate};
use serde::{Serialize, Deserialize};

#[derive( Serialize, Deserialize)]
pub struct Calendar{
    pub name: String,
    pub vacations: Vec<Vacation>
}

#[derive( Serialize, Deserialize)]
pub struct Vacation(pub NaiveDate, pub NaiveDate);
