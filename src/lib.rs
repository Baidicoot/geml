#[macro_use] extern crate lazy_static;
extern crate regex;

mod markdown;

use regex::{Regex, RegexBuilder};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;

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

impl GemlFile {
    pub fn from_path(path: &Path) -> std::io::Result<GemlFile> {
        let content = String::from_utf8_lossy(&fs::read(&path)?).to_string();
        GemlFile::from_string(content, path)
    }

    pub fn from_string(content: String, path: &Path) -> std::io::Result<GemlFile> {
        let gemls = Geml::deserialize(content);
        let pathbuf = path.to_owned();
        let metadata = Metadata { pathbuf, };
        Ok(GemlFile {
            gemls,
            metadata,
        })
    }

    pub fn parse(&self) -> Result<GemlFile, String> {
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
    pub fn deserialize(s: String) -> Vec<Geml> {
        lazy_static!{
            static ref TAGS: Regex = reg(r"^#\[(.+?)\((.+?)\)\]");
            static ref RMWS: Regex = reg(r"\s*([\s\S]*)\s*");
        }
        s.split('$').collect::<Vec<&str>>()[1..]
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
            }).collect()
    }

    pub fn parse(&self) -> Result<Geml, String> {
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

    pub fn to_HTML(&self) -> Result<String, String> {
        Ok(self.parse()?.value.clone())
    }
}
