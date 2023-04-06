use actix_web::get;
use actix_web::web::Json;
use chrono::NaiveDate;

use crate::util::error::JsonError;
use crate::domain::calendar::{Calendar, Vacation};

#[get("/api/calendar")]
pub async fn get() -> Result<Json<Calendar>, JsonError>  {    
    Ok(Json(Calendar{
        name: "test".to_string(),
        vacations: vec![
            Vacation(NaiveDate::from_ymd_opt(2023,1,10).unwrap(), NaiveDate::from_ymd_opt(2023,1,15).unwrap()),
            Vacation(NaiveDate::from_ymd_opt(2023,3,1).unwrap(), NaiveDate::from_ymd_opt(2023,3,7).unwrap()),
            Vacation(NaiveDate::from_ymd_opt(2023,4,20).unwrap(), NaiveDate::from_ymd_opt(2023,4,21).unwrap())
            ]
    }))
}

