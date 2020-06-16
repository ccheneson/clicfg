use std::fmt;

use rusoto_core::request::TlsError;
use rusoto_core::RusotoError;
use rusoto_s3::{GetObjectError, ListObjectsV2Error, PutObjectError};


//PartialEq : used for unit test
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum CliConfigError {
    RusotoError(String),
    FileError(String),
    HttpClientError(TlsError),
    CliArgError(String),
}

impl From<RusotoError<GetObjectError>> for CliConfigError {
    fn from(error: RusotoError<GetObjectError>) -> Self {
        CliConfigError::RusotoError(error.to_string())
    }
}

impl From<TlsError> for CliConfigError {
    fn from(error: TlsError) -> Self {
        CliConfigError::HttpClientError(error)
    }
}

impl From<RusotoError<ListObjectsV2Error>> for CliConfigError {
    fn from(error: RusotoError<ListObjectsV2Error>) -> Self {
        CliConfigError::RusotoError(error.to_string())
    }
}

impl From<RusotoError<PutObjectError>> for CliConfigError {
    fn from(error: RusotoError<PutObjectError>) -> Self {
        CliConfigError::RusotoError(error.to_string())
    }
}


impl From<std::io::Error> for CliConfigError {
    fn from(error: std::io::Error) -> Self {
        CliConfigError::FileError(error.to_string())
    }
}


impl fmt::Display for CliConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CliConfigError::*;
        match &*self {
            RusotoError(err) => write!(f, "{}", err),
            FileError(err) => write!(f, "{}", err),
            HttpClientError(err) => write!(f, "Fail to create S3 Http Client : {}", err),
            CliArgError(err) => write!(f, "Fail to parse arguments : {}", err),
        }
    }
}
