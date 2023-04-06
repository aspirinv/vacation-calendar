use std::{ fmt };
use actix_http::body::BoxBody;

use serde::Serialize;
use serde_json;

use actix_web::{ error, HttpResponse, HttpResponseBuilder };
use actix_web::web::{ Json };
use actix_web::http::{ StatusCode };


#[derive(Debug, Serialize)]
pub struct JsonError{
    status_code: u16,
    pub message: String,
    details: String
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", json)
    }
}

impl error::ResponseError for JsonError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponseBuilder::new(self.status_code())
            .content_type(mime::APPLICATION_JSON)
            .body(Json(self).to_string())
    }
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status_code).unwrap()
    }
}

impl std::convert::From<mongodb::error::Error> for JsonError {
    fn from(value: mongodb::error::Error) -> Self {
        JsonError{ status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(), message: "Failure on data access".to_string(), details: value.to_string()}
    }
}

impl std::convert::From<std::io::Error> for JsonError {
    fn from(value: std::io::Error) -> Self {
        JsonError{ status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(), message: "IO exception".to_string(), details: value.to_string()}
    }
}