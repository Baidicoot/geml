extern crate geml;

use geml::GemlFile;
use std::path::Path;
use std::fs;

fn main() {
    let _geml_str = String::from(r#"
$test1$
*stuff*, __stuff__, stuff.
"#);
   let content = format!("
   <html>
   <style>

    .keyword {{
        color: purple;
    }}

    .type, .call {{
        color: blue;
    }}

    .invocation {{
        color: orangered;
    }}

    .method, .string {{
        color: red;
    }}

    .comment {{
        color: darkgreen;
    }}

    .caps {{
        color: green;
    }}

   </style>
   {}
   </html>", GemlFile::from_path(Path::new("test.geml")).unwrap().parse().unwrap().gemls[1].value.to_owned());
   fs::write("output.html", content).unwrap();
}
