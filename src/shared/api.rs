use std::io::Cursor;

use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};

use serde_json::json;

use super::error::Error;

pub enum ApiResponse {
    Success(String),
    Error((Status, String)),
}

pub type ApiResult = Result<ApiResponse, Error>;

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiResponse {
    fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'o> {
        match self {
            Self::Error((status, err)) => Ok(Response::build()
                .status(status)
                .header(ContentType::JSON)
                .streamed_body(Cursor::new(json!(err).to_string()))
                .finalize()),
            Self::Success(value) => Response::build()
                .status(Status::Ok)
                .header(ContentType::JSON)
                .streamed_body(Cursor::new(value))
                .ok(),
        }
    }
}
