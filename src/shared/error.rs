use std::env::VarError;

use bcrypt::BcryptError;
use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};

use serde_json::json;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP Error {source:?}")]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },
    #[error("SerdeJson Error {source:?}")]
    SerdeJson {
        #[from]
        source: serde_json::Error,
    },
    #[error("StdError Error {source:?}")]
    StdError {
        #[from]
        source: Box<dyn std::error::Error>,
    },
    #[error("IoError Error {source:?}")]
    IoError {
        #[from]
        source: std::io::Error,
    },
    #[error("VarError Error {source:?}")]
    VarError {
        #[from]
        source: VarError,
    },
    #[error("JwtError Error {source:?}")]
    JwtError {
        #[from]
        source: jwt::Error,
    },
    #[error("BcryptError Error {source:?}")]
    BcryptError {
        #[from]
        source: BcryptError,
    },
}

impl<'r> Error {
    fn make_error(
        _req: &Request<'_>,
        status: Status,
        source: String,
    ) -> Result<Response<'r>, Status> {
        error!("{:?}", source);
        Response::build()
            .status(status)
            .header(ContentType::JSON)
            .streamed_body(std::io::Cursor::new(json!(source).to_string()))
            .ok()
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        match self {
            Error::Reqwest { source } => {
                Self::make_error(req, Status::InternalServerError, source.to_string())
            }
            Error::StdError { source } => {
                Self::make_error(req, Status::InternalServerError, source.to_string())
            }
            Error::SerdeJson { source } => {
                Self::make_error(req, Status::BadRequest, source.to_string())
            }
            Error::IoError { source } => {
                Self::make_error(req, Status::InternalServerError, source.to_string())
            }
            Error::VarError { source } => {
                Self::make_error(req, Status::InternalServerError, source.to_string())
            }
            Error::JwtError { source } => {
                Self::make_error(req, Status::InternalServerError, source.to_string())
            }
            Error::BcryptError { source } => {
                Self::make_error(req, Status::InternalServerError, source.to_string())
            }
        }
    }
}
