#![feature(duration_as_u128)]
extern crate geml;

use geml::GemlFile;
use std::path::Path;
use std::fs;
use std::time::{Instant};

fn main() {
    let application_start = Instant::now();
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

    .ins {{
        color: blue;
    }}

    .pointer {{
        color: red;
    }}

    .disc {{
        color: purple;
    }}

    .int {{
        color: orangered;
    }}

    pre {{
        background: #E0E0E0;
        padding: 1rem 1rem;
    }}

   </style>
   {}
   </html>", GemlFile::from_path(Path::new("test.geml")).unwrap().parse().unwrap().gemls[1].value.to_owned());
   fs::write("output.html", content).unwrap();
   println!("Application took {}ms.", application_start.elapsed().as_millis());
}
