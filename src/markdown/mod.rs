extern crate regex;

use regex::{Regex, Captures};
use crate::reg;

mod rust_syntax;

//All of the Regexs
lazy_static!{
    static ref CODE_BLOCK: Regex = reg(r"```(\S*)\s+([\s\S]*?)\s+```");

    static ref CODE_INLINE: Regex = reg(r"(`)(.*?)(`)");

    static ref LINK: Regex = reg(r"\[([^\[]+)\]\(([^\)]+)\)");

    static ref HEADING: Regex = reg(r"\n(#+)[^\S\r\n]+(.+)");

    static ref EMPHASIS: Regex = reg(r"(\*{1,2})(.*?)(\*{1,2})");

    static ref EMPHASIS_UNDER: Regex = reg(r"(_{1,2})(.*?)(_{1,2})");

    static ref STRIKETHROUGH: Regex = reg(r"(\~\~)(.*?)(\~\~)");

    static ref HORIZONTAL: Regex = reg(r"\n((\-{3,})|(={3,})|(#{3,}))");

    static ref UNORDERED: Regex = reg(r"(\n\s*(\-|\+)\s.*)+");

    static ref ORDERED: Regex = reg(r"(\n\s*([0-9]+\.)\s.*)+");

    static ref BREAK: Regex = reg(r"\n\s*\n");

    static ref PARAGRAPH: Regex = reg(r"(\r\n\r\n|\n\n)([\s\S]+?)(\r\n\r\n|\n\n)");

    /* Markdown or HTML reserved symbols */
    static ref LT: Regex = reg(r"<");

    static ref GT: Regex = reg(r">");

    static ref AMP: Regex = reg(r"&");

    static ref AST: Regex = reg(r"\*");

    static ref UND: Regex = reg(r"_");

    static ref TIC: Regex = reg(r"`");

    static ref EQL: Regex = reg(r"=");

    static ref HYP: Regex = reg(r"-");

    static ref HASH: Regex = reg(r"#");

    static ref LFEED: Regex = reg(r"(\r\n|\n)");
}

//function to replace HTML or Markdown reserved symbols
fn symbols(s: &str) -> String {
    LFEED.replace_all(&HYP.replace_all(&EQL.replace_all(&TIC.replace_all(&UND.replace_all(&AST.replace_all(&LT.replace_all(&GT.replace_all(&HASH.replace_all(&AMP.replace_all(s, "&amp;"), "&#35;"), "&gt;"), "&lt;"), "&#42;"), "&#95;"), "&#96;"), "&#61;"), "&#45;"), "&#10;").to_string()
}

/* The replacer functions */

fn break_replacer(cap: &str) -> String {
    LFEED.replace_all(cap, "<br />").to_string()
}

fn paragraph_replacer(cap: &Captures) -> String {
    format!("<p>{}</p>", break_replacer(&cap[2]))
}

fn code_block_replacer(cap: &Captures) -> String {
    let code = match &cap[1] {
        "rust" => rust_syntax::parse(symbols(&cap[2])),
        _ => symbols(&cap[2]),
    };
    format!("<pre>{}</pre>", code)
}

fn code_inline_replacer(cap: &Captures) -> String {
    format!("<code>{}</code>", &cap[2])
}

fn link_replacer(cap: &Captures) -> String {
    format!("<a href='{}'>{}</a>", &cap[2], &cap[1])
}

fn heading_replacer(cap: &Captures) -> String {
    format!("\n<h{}>{}</h{}>", cap[1].len().to_string(), &cap[2], cap[1].len().to_string())
}

fn emphasis_replacer(cap: &Captures) -> String {
    format!("<{}>{}</{}>", if{cap[1].len()==1}{"em"}else{"strong"}, &cap[2], if{cap[1].len()==1}{"em"}else{"strong"})
}

fn rule_replacer(_cap: &Captures) -> String {
    format!("\n<hr />")
}

fn unordered_replacer(cap: &Captures) -> String {
    let mut items = String::from("");
    for i in cap[0].trim().split('\n') {
        items = format!("{}<li>{}</li>", items, &i[2..]);
    }
    format!("<ul>{}</ul>", items)
}

fn ordered_replacer(cap: &Captures) -> String {
    let mut items = String::from("");
    for i in cap[0].trim().split('\n') {
        items = format!("{}<li>{}</li>", items, &i[i.find('.').unwrap()+2..]);
    }
    format!("<ol>{}</ol>", items)
}

//The main format function; call this to get markdown with the best results
pub fn parse(s: String) -> String {
    replace::paragraphs(replace::unordered(replace::ordered(replace::rules(replace::emphasis(replace::headings(replace::links(replace::code_inline(replace::code_blocks(s)))))))))
}

//Individual markdown replacement functions.
pub mod replace {
    use crate::markdown::*;

    pub fn paragraphs(s: String) -> String {
        PARAGRAPH.replace_all(&s, &paragraph_replacer).to_string()
    }

    pub fn code_blocks(s: String) -> String {
        CODE_BLOCK.replace_all(&s, &code_block_replacer).to_string()
    }

    pub fn code_inline(s: String) -> String {
        CODE_INLINE.replace_all(&s, &code_inline_replacer).to_string()
    }

    pub fn links(s: String) -> String {
        LINK.replace_all(&s, &link_replacer).to_string()
    }

    pub fn headings(s: String) -> String {
        HEADING.replace_all(&s, &heading_replacer).to_string()
    }

    pub fn emphasis(s: String) -> String {
        EMPHASIS_UNDER.replace_all(&EMPHASIS.replace_all(&s, &emphasis_replacer), &emphasis_replacer).to_string()
    }

    pub fn rules(s: String) -> String {
        HORIZONTAL.replace_all(&s, &rule_replacer).to_string()
    }

    pub fn unordered(s: String) -> String {
        UNORDERED.replace_all(&s, &unordered_replacer).to_string()
    }

    pub fn ordered(s: String) -> String {
        ORDERED.replace_all(&s, &ordered_replacer).to_string()
    }
}
