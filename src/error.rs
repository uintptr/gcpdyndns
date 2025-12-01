pub type Result<T> = core::result::Result<T, Error>;

use derive_more::From;
use reqwest::StatusCode;

#[derive(Debug, From)]
pub enum Error {
    FileNameError,
    UploadFailure,
    DataDirNotFound,
    UpdateFailure(StatusCode),
    //
    // 2d party
    //
    #[from]
    Io(std::io::Error),
    #[from]
    Utf8(std::string::FromUtf8Error),
    //
    // 3rd party
    //
    #[from]
    HttpRequestError(reqwest::Error),
    #[from]
    Deserialize(serde_json::Error),
    #[from]
    AuthError(gcp_auth::Error),
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
