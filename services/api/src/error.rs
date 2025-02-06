use sea_orm::DbErr;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket::Request;
use rocket::serde::json::serde_json;
use std::io::Cursor;

pub struct ErrorResponder {
    status: Status,
    message: String,
}

impl ErrorResponder {
    pub fn new(status: Status, message: &str) -> Self {
        ErrorResponder {
            status,
            message: message.to_string(),
        }
    }
}

impl<'r> Responder<'r, 'static> for ErrorResponder {
    fn respond_to(self, _: &'r Request<'_>) -> Result<Response<'static>, Status> {
        let body = serde_json::json!({ "error": self.message }).to_string();
        Ok(Response::build()
            .status(self.status)
            .header(ContentType::JSON)
            .sized_body(body.len(), Cursor::new(body))
            .finalize())
    }
}
        

impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> ErrorResponder {
        ErrorResponder {
            status: Status::InternalServerError,
            message: err.to_string(),
        }
    }
}

impl From<String> for ErrorResponder {
    fn from(string: String) -> ErrorResponder {
        ErrorResponder { 
            status: Status::InternalServerError, 
            message: string 
        }
    }
}

impl From<&str> for ErrorResponder {
    fn from(str: &str) -> ErrorResponder {
        str.to_owned().into()
    }
}
