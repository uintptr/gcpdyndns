use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    FileNameError,
    UploadFailure,
    DataDirNotFound,
    UpdateFailure(StatusCode),
    DomainParsingFailure,
    DomainRecordNotFound,
    //
    // 2d party
    //
    Io(#[from] std::io::Error),
    Utf8(#[from] std::string::FromUtf8Error),
    //
    // 3rd party
    //
    HttpRequestError(#[from] reqwest::Error),
    Deserialize(#[from] serde_json::Error),
    AuthError(#[from] gcp_auth::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Error::Io(io) => {
                write!(fmt, "{}", io.kind())
            }
            _ => write!(fmt, "{self:?}"),
        }
    }
}
