use regex::{Regex, Captures};
use crate::reg;

lazy_static!{
    static ref INS: Regex = reg(r"&#10;\s*([A-Z]+)(\s([:@0-9 ]+)|&#10;|$)");

    static ref POINTER: Regex = reg(r"^@[0-9]+$");

    static ref DISC: Regex = reg(r"^:[0-9]+$");

    static ref LABEL: Regex = reg(r"&#10;\s*([A-Z]+):(\s|&#10;|$)");

    static ref LITERAL: Regex = reg(r"^([0-9]+|[A-Za-z])$");

    static ref COMMENT: Regex = reg(r"&#35;.+$");
}

fn ins_replacer(cap: &Captures) -> String {
    let v: String = cap[0].split(" ")
        .map(|x| {
            replace::literals(replace::discs(replace::pointers(x.to_string())))
        })
        .collect::<Vec<String>>()
        .join(" ");
    format!("<span class='ins'>{}</span>", v)
}

fn pointer_replacer(cap: &Captures) -> String {
    format!("<span class='pointer'>{}</span>", &cap[0])
}

fn disc_replacer(cap: &Captures) -> String {
    format!("<span class='disc'>{}</span>", &cap[0])
}

fn literal_replacer(cap: &Captures) -> String {
    format!("<span class='int'>{}</span>", &cap[0])
}

fn comment_replacer(cap: &Captures) -> String {
    format!("<span class='comment'>{}</span>", &cap[0])
}

pub fn parse(s: String) -> String {
    replace::instructions(replace::comments(s))
}

pub mod replace {
    use super::*;

    pub fn instructions(s: String) -> String {
        INS.replace_all(&s, &ins_replacer).to_string()
    }

    pub fn pointers(s: String) -> String {
        POINTER.replace_all(&s, &pointer_replacer).to_string()
    }

    pub fn discs(s: String) -> String {
        DISC.replace_all(&s, &disc_replacer).to_string()
    }

    pub fn literals(s: String) -> String {
        LITERAL.replace_all(&s, &literal_replacer).to_string()
    }

    pub fn comments(s: String) -> String {
        s.split("&#10;")
            .map(|x| {
                COMMENT.replace_all(x, &comment_replacer).to_string()
            })
            .collect::<Vec<String>>()
            .join("&#10;")
    }
}