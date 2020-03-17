pub type HentaiResult<T> = Result<T, HentaiError>;

#[derive(Debug)]
pub enum HentaiError {
    Reqwest(reqwest::Error),
    Url(url::ParseError),
    BadStatusCode(reqwest::StatusCode),
    Io(std::io::Error),

    InvalidBody,
}

impl From<reqwest::Error> for HentaiError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<url::ParseError> for HentaiError {
    fn from(e: url::ParseError) -> Self {
        Self::Url(e)
    }
}

impl From<std::io::Error> for HentaiError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
