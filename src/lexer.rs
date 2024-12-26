#[derive(Debug, PartialEq, Clone)]
pub enum PrimitiveFnc {
    Sum,
    Mul,
    And,
    Or,
    Xor,
    Not,
    LogAnd,
    LogOr,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    If,
    Else,
    For,
    Fnc,
    Ret,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Body {
    Open,
    Close,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Bracket {
    LeftBracket,
    RightBracket,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i64),

    Assign,
    Declaration,

    Variable(String),

    PrimFuncs(PrimitiveFnc),
    Brackets(Bracket),
    Bodys(Body),
    Keywords(Keyword),

    Dot,
    Comma,

    End,
}

struct Context {
    startoftoken: usize,
    index: usize,
    tokens: Vec<Token>,
    str2tok: HashMap<String, Token>,
}

impl Context {
    pub fn parse(&mut self, string: &String, newsubstr: &String) {
        if self.startoftoken != self.index { //ya ne dolbaeb (mb), tam nizhe block dolzhen vipolnit`sya
            let strofoldtoken: String = string[self.startoftoken..self.index].to_string();

            if strofoldtoken
                .chars()
                .nth(0)
                .unwrap()
                .is_ascii_alphabetic() {
                self.tokens.push(Token::Variable(strofoldtoken
                    .as_str()
                    .trim()
                    .to_string()
                ));
            } else {
                self.tokens.push(Token::Number(strofoldtoken
                    .parse::<i64>()
                    .unwrap()
                ));
            }
        }
        self.index += newsubstr.len() - 1;
        self.startoftoken = self.index + 1;
    }

    pub fn parsepush(&mut self, string: &String, newsubstr: &str) {
        self.parse(string, &newsubstr.to_string());
        self.tokens.push(self.str2tok.get(newsubstr).unwrap().clone());
    }
}

use std::collections::HashMap;

fn init() -> HashMap<String, Token> {
    let mut map = HashMap::new();

    map.insert(String::from(":="), Token::Declaration);
    map.insert(String::from("="), Token::Assign);

    map.insert(String::from("if"), Token::Keywords(Keyword::If));
    map.insert(String::from("else"), Token::Keywords(Keyword::Else));
    map.insert(String::from("ret"), Token::Keywords(Keyword::Ret));
    map.insert(String::from("for"), Token::Keywords(Keyword::For));
    map.insert(String::from("fnc"), Token::Keywords(Keyword::Fnc));

    map.insert(String::from("+"), Token::PrimFuncs(PrimitiveFnc::Sum));
    map.insert(String::from("*"), Token::PrimFuncs(PrimitiveFnc::Mul));
    map.insert(String::from("&"), Token::PrimFuncs(PrimitiveFnc::And));
    map.insert(String::from("|"), Token::PrimFuncs(PrimitiveFnc::Or));
    map.insert(String::from("^"), Token::PrimFuncs(PrimitiveFnc::Xor));
    map.insert(String::from("!"), Token::PrimFuncs(PrimitiveFnc::Not));
    map.insert(String::from("&&"), Token::PrimFuncs(PrimitiveFnc::LogAnd));
    map.insert(String::from("||"), Token::PrimFuncs(PrimitiveFnc::LogOr));

    map.insert(String::from("("), Token::Brackets(Bracket::LeftBracket));
    map.insert(String::from(")"), Token::Brackets(Bracket::RightBracket));

    map.insert(String::from("{"), Token::Bodys(Body::Open));
    map.insert(String::from("}"), Token::Bodys(Body::Close));

    map.insert(String::from("."), Token::Dot);
    map.insert(String::from(","), Token::Comma);
    map.insert(String::from(";"), Token::End);

    map
}

pub fn lex(string: String) -> Vec<Token> {

    let mut cntxt = Context {
        startoftoken: 0,
        index: 0,
        tokens: Vec::new(),
        str2tok: init(),
    };

    while cntxt.index < string.len() {

        let c = string.chars().nth(cntxt.index).unwrap();
        match c {
            ' ' => {
                cntxt.parse(&string, &" ".to_string());
            }
            _ => match string[cntxt.index..].to_string() {
                s if s.starts_with(";") => {
                    cntxt.parsepush(&string, ";");
                }
                s if s.starts_with(":=") => {
                    cntxt.parsepush(&string, ":=");
                }
                s if s.starts_with("=") => {
                    cntxt.parsepush(&string, "=");
                }
                s if s.starts_with("+") => {
                    cntxt.parsepush(&string, "+");
                }
                s if s.starts_with("*") => {
                    cntxt.parsepush(&string, "*");
                }
                s if s.starts_with("&") => {
                    cntxt.parsepush(&string, "&");
                }
                s if s.starts_with("|") => {
                    cntxt.parsepush(&string, "|");
                }
                s if s.starts_with("^") => {
                    cntxt.parsepush(&string, "^");
                }
                s if s.starts_with("!") => {
                    cntxt.parsepush(&string, "!");
                }
                s if s.starts_with("&&") => {
                    cntxt.parsepush(&string, "&&");
                }
                s if s.starts_with("||") => {
                    cntxt.parsepush(&string, "||");
                }
                s if s.starts_with("if") => {
                    cntxt.parsepush(&string, "if");
                }
                s if s.starts_with("else") => {
                    cntxt.parsepush(&string, "else");
                }
                s if s.starts_with("for") => {
                    cntxt.parsepush(&string, "for");
                }
                s if s.starts_with("fnc") => {
                    cntxt.parsepush(&string, "fnc");
                }
                s if s.starts_with("ret") => {
                    cntxt.parsepush(&string, "ret");
                }
                s if s.starts_with("{") => {
                    cntxt.parsepush(&string, "{");
                }
                s if s.starts_with("}") => {
                    cntxt.parsepush(&string, "}");
                }
                s if s.starts_with("(") => {
                    cntxt.parsepush(&string, "(");
                }
                s if s.starts_with(")") => {
                    cntxt.parsepush(&string, ")");
                }
                s if s.starts_with(".") => {
                    cntxt.parsepush(&string, ".");
                }
                s if s.starts_with(",") => {
                    cntxt.parsepush(&string, ",");
                }

                _ => {}
            }
        }
        cntxt.index += 1;
    }
    cntxt.tokens
}
