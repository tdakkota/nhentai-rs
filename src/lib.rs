mod types;

pub use crate::types::Comic;
use lazy_static::lazy_static;
pub use reqwest::Url;
pub use select::document::Document;
use std::io::Write;

lazy_static! {
    pub static ref NHENTAI_BASE: Url = Url::parse("https://nhentai.net").unwrap();
    pub static ref NHENTAI_IMAGE_BASE: Url = Url::parse("https://i.nhentai.net").unwrap();
}

pub type HentaiResult<T> = Result<T, HentaiError>;

#[derive(Debug)]
pub enum HentaiError {
    Network,

    InvalidBody,

    File,
    Url,
    Io,
}

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Client {
            client: reqwest::Client::new(),
        }
    }

    pub fn get_comic(&self, id: u64) -> HentaiResult<Comic> {
        let url = NHENTAI_BASE
            .join(&format!("g/{}", id))
            .map_err(|_| HentaiError::Url)?;

        let res = self
            .client
            .get(url)
            .send()
            .map_err(|_| HentaiError::Network)?;

        if !res.status().is_success() {
            return Err(HentaiError::Network);
        }

        let doc = Document::from_read(res).map_err(|_| HentaiError::InvalidBody)?;
        Comic::from_doc(&doc).ok_or(HentaiError::InvalidBody)
    }

    pub fn get_random(&self) -> HentaiResult<Comic> {
        let url = NHENTAI_BASE.join("random").map_err(|_| HentaiError::Url)?;
        let res = self
            .client
            .get(url)
            .send()
            .map_err(|_| HentaiError::Network)?;

        if !res.status().is_success() {
            return Err(HentaiError::Network);
        }

        let doc = Document::from_read(res).map_err(|_| HentaiError::InvalidBody)?;
        Comic::from_doc(&doc).ok_or(HentaiError::InvalidBody)
    }

    pub fn copy_res_to<T: Write>(&self, url: &Url, mut writer: T) -> HentaiResult<()> {
        self.client
            .get(url.as_str())
            .send()
            .map_err(|_| HentaiError::Network)?
            .copy_to(&mut writer)
            .map_err(|_| HentaiError::Io)?;
        Ok(())
    }
}
