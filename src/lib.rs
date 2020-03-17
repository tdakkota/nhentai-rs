pub mod error;
mod types;

pub use crate::{
    error::{
        HentaiError,
        HentaiResult,
    },
    types::Comic,
};
use lazy_static::lazy_static;
pub use reqwest::Url;
pub use select::document::Document;
use std::io::Write;

lazy_static! {
    pub static ref NHENTAI_BASE: Url = Url::parse("https://nhentai.net").unwrap();
    pub static ref NHENTAI_IMAGE_BASE: Url = Url::parse("https://i.nhentai.net").unwrap();
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

    pub fn new_with_client(client: reqwest::Client) -> Self {
        Client { client }
    }

    pub fn get_comic(&self, id: u64) -> HentaiResult<Comic> {
        let url = NHENTAI_BASE.join(&format!("g/{}", id))?;

        let res = self.client.get(url).send()?;

        let status = res.status();
        if !status.is_success() {
            return Err(HentaiError::BadStatusCode(status));
        }

        let doc = Document::from_read(res)?;
        Comic::from_doc(&doc).ok_or(HentaiError::InvalidBody)
    }

    pub fn get_random(&self) -> HentaiResult<Comic> {
        let url = NHENTAI_BASE.join("random")?;
        let res = self.client.get(url).send()?;

        let status = res.status();
        if !status.is_success() {
            return Err(HentaiError::BadStatusCode(status));
        }

        let doc = Document::from_read(res)?;
        Comic::from_doc(&doc).ok_or(HentaiError::InvalidBody)
    }

    pub fn copy_res_to<T: Write>(&self, url: &Url, mut writer: T) -> HentaiResult<()> {
        self.client.get(url.as_str()).send()?.copy_to(&mut writer)?;
        Ok(())
    }
}
