extern crate geml;

use geml::GemlFile;
use std::path::Path;

fn main() {
    let _geml_str = String::from(r#"
$test1$
*stuff*, __stuff__, stuff.
"#);
    println!("{:?}", GemlFile::from_path(Path::new("test.geml")));
}
