use regex::{Regex, Captures};
use crate::reg;

lazy_static!{
    static ref FUNC: Regex = reg(r"((\w|(&#95;))+)\(");

    static ref METHOD: Regex = reg(r"((\w|(&#95;))+)\.");

    static ref OPERAND: Regex = reg(r": ((\w|&#95;|&amp;)+)");

    static ref RETURNED: Regex = reg(r"&#45;&gt; ((\w|&#95;|&amp;)+)");

    static ref MULTILINE_COMMENT: Regex = reg(r"/&#42;([\s\S]*?)&#42;/");

    static ref COMMENT: Regex = reg(r"//([\s\S]*?)&#10;");

    static ref MODULE: Regex = reg(r"((\w|&#95;)+)::");

    static ref MACRO: Regex = reg(r"((\w|&#95;)+)!\(");

    static ref STRING: Regex = reg(r#""([\s\S]*?)""#);

    static ref KEYWORDS: Regex = reg(r"(abstract|alignof|as|become|box|break|const|continue|crate|do|else|enum|extern|false|final|fn|for|if|impl|in|let|loop|macro|match|mod|move|mut|offsetof|override|priv|proc|pub|pure|ref|return|Self|self|sizeof|static|struct|super|trait|true|type|typeof|unsafe|unsized|use|virtual|where|while|yield)(::|\s)");
}

fn replace_keywords(cap: &Captures) -> String {
    format!("<span class='keyword'>{}</span>{}", &cap[1], &cap[2])
}

fn replace_func(cap: &Captures) -> String {
    format!("<span class='call'>{}</span>(", &cap[1])
}

fn replace_method(cap: &Captures) -> String {
    format!("<span class='method'>{}</span>.", &cap[1])
}

fn replace_operand(cap: &Captures) -> String {
    format!(": <span class='type'>{}</span>", &cap[1])
}

fn replace_returned(cap: &Captures) -> String {
    format!("-> <span class='type'>{}</span>", &cap[1])
}

fn replace_multiline_comment(cap: &Captures) -> String {
    format!("<span class='comment'>/&#42;{}&#42;/</span>", &cap[1])
}

fn replace_comment(cap: &Captures) -> String {
    format!("<span class='comment'>//{}</span>\n", &cap[1])
}

fn replace_module(cap: &Captures) -> String {
    format!("<span class='module'>{}</span>::", &cap[1])
}

fn replace_macro(cap: &Captures) -> String {
    format!("<span class='invocation'>{}!</span>(", &cap[1])
}

fn replace_string(cap: &Captures) -> String {
    format!(r#"<span class='string'>"{}"</span>"#, &cap[1])
}

pub fn parse(s: String) -> String {
    replace::other(replace::calls(replace::types(replace::keywords(s))))
}

pub mod replace {
    use crate::markdown::rust_syntax::*;
    pub fn keywords(s: String) -> String {
        KEYWORDS.replace_all(&s, &replace_keywords).to_string()
    }

    pub fn types(s: String) -> String {
        RETURNED.replace_all(&OPERAND.replace_all(&s, &replace_operand), &replace_returned).to_string()
    }

    pub fn calls(s: String) -> String {
        MODULE.replace_all(&MACRO.replace_all(
            &METHOD.replace_all(&FUNC.replace_all(&s, &replace_func), &replace_method),
         &replace_macro), &replace_module).to_string()
    }

    pub fn other(s: String) -> String {
        STRING.replace_all(&MULTILINE_COMMENT.replace_all(
            &COMMENT.replace_all(&s, &replace_comment),
        &replace_multiline_comment), &replace_string).to_string()
    }
}