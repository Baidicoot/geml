# GEML
GEML (GEnerator Markdown Language) is a simple string-orientated ml parser, made with regex, designed to be used with static site generators using HTML templates. Each entry is structured as a title (which is surrounded by `$` when serialized), a map of tags (specified like rust attributes), and the text itself. A GemlFile is a vector of these combined with some metadata. As such, it can compile to HTML, and has it's own (somewhat limited) markdown parser.

## Usage
To deserialize a GEML file, you can use the `GemlFile::from_string` or `from_path` functions: (Note that you have to specify a root path if reading from a string)
```rust
let geml = String::from("
$test1$
#[markdown(enabled)]
Note that markdown is enabled by default.
*markdown*, cool.
");
let deserialized = GemlFile::from_string(geml, Path::new("root/dir/")).unwrap();
```
You can also serialize a vector of raw GEML using `Geml::deserialize`:
```rust
let geml = String::from("
$test1$
#[markdown(enabled)]
Note that markdown is enabled by default.
*markdown*, cool.
");
let deserialized = Geml:deserialize(geml).unwrap();
```

The main idea behind GEML is that the SSG looks for GEML-style titles in an HTML template, and then replaces them with the corresponding GEML's value.

So the template:
```html
<html>
<body>
$test1$
</body>
</html>
```
Would become:
```html
<html>
<body>

Note that markdown is enabled by default.
<em>markdown</em>, cool.

</body>
</html>
```

Test, Please Ignore
