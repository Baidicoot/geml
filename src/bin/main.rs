#![feature(duration_as_u128)]
extern crate geml;

use geml::GemlFile;
use std::path::Path;
use std::time::{Instant};

fn main() {
    let application_start = Instant::now();
    println!("{:?}", GemlFile::from_path(&Path::new("../rust_ssg/test.geml")).unwrap());
    println!("Application took {}ms.", application_start.elapsed().as_millis());
}
