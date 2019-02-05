#[macro_use] extern crate lazy_static;
extern crate regex;

mod markdown;

use regex::{Regex, RegexBuilder};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::error;
use std::io;
use std::fs;
use std::fmt;

//mod markdown;

#[derive(Debug)]
#[derive(Clone)]
pub struct Metadata {
    pathbuf: PathBuf,
}

#[derive(Debug)]
pub struct GemlFile {
    pub gemls: Vec<Geml>,
    pub metadata: Metadata,
}

#[derive(Debug)]
pub enum GemlErrorKind {
    IoError(String),
    MarkdownError(&'static str),
    HtmlError(&'static str),
    ParseError(&'static str),
}

impl GemlErrorKind {
    pub fn unwrap(&self) -> &str {
        use crate::GemlErrorKind::*;
        match self {
            IoError(x) => &x,
            MarkdownError(x) => x,
            HtmlError(x) => x,
            ParseError(x) => x,
        }
    }
}

#[derive(Debug)]
pub struct GemlParseError {
    details: GemlErrorKind,
}

impl fmt::Display for GemlParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.details)
    }
}

impl error::Error for GemlParseError {
    fn description(&self) -> &str {
        self.details.unwrap()
    }
}

impl From<io::Error> for GemlParseError {
    fn from(err: io::Error) -> GemlParseError {
        use crate::GemlErrorKind::*;
        GemlParseError { details: IoError(err.to_string()) }
    }
}

pub type Result<T> = std::result::Result<T, GemlParseError>;

impl GemlFile {
    pub fn from_path(path: &Path) -> Result<GemlFile> {
        let content = String::from_utf8_lossy(&fs::read(&path)?).to_string();
        GemlFile::from_string(content, path)
    }

    pub fn from_string(content: String, path: &Path) -> Result<GemlFile> {
        let gemls = Geml::deserialize(content)?;
        let pathbuf = path.to_owned();
        let metadata = Metadata { pathbuf, };
        Ok(GemlFile {
            gemls,
            metadata,
        })
    }

    pub fn parse(&self) -> Result<GemlFile> {
        let mut gemls: Vec<Geml> = vec![];
        for g in self.gemls.iter() {
            gemls.push(g.parse()?);
        }
        Ok(GemlFile {
            gemls,
            metadata: self.metadata.clone(),
        })
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Geml {
    pub key: String,
    pub value: String,
    pub tags: HashMap<String, String>,
}

fn reg(s: &str) -> regex::Regex {
    RegexBuilder::new(s)
        .multi_line(true)
        .build()
        .unwrap()
}

impl Geml {
    pub fn deserialize(s: String) -> Result<Vec<Geml>> {
        lazy_static!{
            static ref TAGS: Regex = reg(r"^#\[(.+?)\((.+?)\)\]");
            static ref RMWS: Regex = reg(r"\s*([\s\S]*)\s*");
        }
        Ok(s.split('$').collect::<Vec<&str>>()[1..]
            .chunks(2)
            .filter(|x| (x.len() == 2))
            .map(|x| {
                let mut val_start = 0;
                let mut tags = HashMap::new();
                for cap in TAGS.captures_iter(&x[1]) {
                    tags.insert(cap[1].to_string(), cap[2].to_string());
                    val_start = cap.get(0).unwrap().end();
                }
                let value = match RMWS.find(&x[1][val_start..]) {
                    Some(x) => x.as_str().to_owned(),
                    None => String::from(""),
                };
                Geml {
                    key: x[0].to_owned(),
                    value,
                    tags,
                }
            }).collect())
    }

    pub fn parse(&self) -> Result<Geml> {
        let mut value = self.value.clone();
        if self.tags.get(&"markdown".to_owned()).unwrap_or(&"enabled".to_owned()) == &"enabled".to_owned() {
            value = markdown::parse(value);
        }
        Ok(Geml {
            key: self.key.clone(),
            tags: self.tags.clone(),
            value: value,
        })
    }

    pub fn to_html(&self) -> Result<String> {
        Ok(self.parse()?.value.clone())
    }
}
