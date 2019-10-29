use crate::{
    NHENTAI_BASE,
    NHENTAI_IMAGE_BASE,
};
use ratel::ast::{
    expression::{
        CallExpression,
        Expression,
        PrefixExpression,
    },
    statement::DeclarationStatement,
    Declarator,
    Pattern,
    StatementNode,
};
use ratel_visitor::{
    Visitable,
    Visitor,
};
use reqwest::Url;
use select::{
    document::Document,
    predicate::*,
};
use serde::{
    de::Deserializer,
    Deserialize,
    Serialize,
};
use std::{
    collections::HashMap,
    convert::{
        TryFrom,
        TryInto,
    },
    fmt::Display,
    str::FromStr,
};

struct ComicVisitor {
    start: Option<usize>,
    end: Option<usize>,
}

impl ComicVisitor {
    pub fn new() -> Self {
        ComicVisitor {
            start: None,
            end: None,
        }
    }
}

impl<'ast> Visitor<'ast> for ComicVisitor {
    fn on_declaration_statement(
        &mut self,
        item: &DeclarationStatement,
        _node: &'ast StatementNode<'ast>,
    ) {
        if let Some(Declarator { id, init }) = item.declarators.iter().next().map(|el| ***el) {
            if **id != Pattern::Identifier("gallery") {
                return;
            }

            if let Some(Expression::Prefix(PrefixExpression { operand, .. })) = init.map(|el| **el)
            {
                if let Expression::Call(CallExpression { arguments, .. }) = **operand {
                    if let Some((start, end, Expression::Object(_obj))) =
                        arguments.iter().next().map(|el| (el.start, el.end, ***el))
                    {
                        self.start = start.try_into().ok();
                        self.end = end.try_into().ok();
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    count: u64,
    id: u32,
    name: String,

    #[serde(rename = "type")]
    kind: String,

    url: String,

    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "h")]
    height: u32,

    #[serde(rename = "w")]
    width: u32,

    t: String,

    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ComicImages {
    cover: Image,
    pages: Vec<Image>,
    thumbnail: Image,

    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ComicTitle {
    english: String,
    japanese: String,
    pretty: String,

    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comic {
    #[serde(deserialize_with = "from_str_or_num")]
    id: u64,
    num_pages: u32,
    scanlator: String,
    tags: Vec<Tag>,
    num_favorites: u64,
    upload_date: u64,
    media_id: String,
    images: ComicImages,
    title: ComicTitle,

    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}

impl Comic {
    pub fn from_doc(doc: &Document) -> Option<Self> {
        let text = doc
            .find(Name("script"))
            .last()?
            .find(Text)
            .last()?
            .as_text()?;
        let ast = ratel::parse(text).ok()?;
        let mut visitor = ComicVisitor::new();
        ast.visit_with(&mut visitor);
        let data = &text[visitor.start?..visitor.end?];

        serde_json::from_str(data).ok()
    }

    pub fn get_pages(&self) -> &[Image] {
        &self.images.pages
    }

    pub fn iter_pages(&self) -> impl Iterator<Item = &Image> {
        self.images.pages.iter()
    }

    pub fn iter_page_urls<'a>(&'a self) -> impl Iterator<Item = Url> + 'a {
        let id = self.id;
        self.iter_pages()
            .enumerate()
            .map(move |(i, _img)| NHENTAI_BASE.join(&format!("g/{}/{}/", id, i + 1)).unwrap())
    }

    pub fn iter_resolved_page_urls<'a>(&'a self) -> impl Iterator<Item = Url> + 'a {
        let media_id = &self.media_id;
        self.iter_pages().enumerate().map(move |(i, _img)| {
            NHENTAI_IMAGE_BASE
                .join(&format!("galleries/{}/{}.jpg", media_id, i + 1))
                .unwrap()
        })
    }

    pub fn get_num_pages(&self) -> usize {
        self.images.pages.len()
    }

    pub fn get_english_title(&self) -> &str {
        &self.title.english
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_media_id(&self) -> &str {
        &self.media_id
    }

    pub fn get_num_favorites(&self) -> u64 {
        self.num_favorites
    }
}

fn from_str_or_num<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T: TryFrom<u64>,
    <T as TryFrom<u64>>::Error: std::fmt::Display,
    <T as FromStr>::Err: Display,
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::String(s) => T::from_str(&s).map_err(serde::de::Error::custom),
        serde_json::Value::Number(n) => {
            if let Some(n) = n.as_u64() {
                T::try_from(n).map_err(serde::de::Error::custom)
            } else {
                Err(serde::de::Error::custom("No type match"))
            }
        }
        _ => Err(serde::de::Error::custom("No type match")),
    }
}
