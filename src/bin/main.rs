extern crate geml;

use geml::GemlFile;
use std::path::Path;

fn main() {
    let geml_str = String::from(r#"
$test1$
*stuff*, __stuff__, stuff.
"#);
    println!("{:?}", GemlFile::from_string(geml_str, Path::new("test")).unwrap().parse().unwrap());
}
